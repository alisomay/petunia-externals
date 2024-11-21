pub mod types;

use crate::api;
use crate::api::object_type::*;
use crate::error::ParseError;
use crate::parse::types::ParseResult;
use crate::types::CommandType;
use crate::value::{RytmValue, RytmValueList};
use error_logger_macro::log_errors;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::FromStr;
use tracing::error;
use tracing::instrument;
use types::{Number, ObjectTypeSelector, ParsedValue, PlockOperation};

/// Parses a 'get' or 'set' command, given the values (excluding the selector)
#[instrument]
pub fn parse_command(
    values: &RytmValueList,
    command_type: CommandType,
) -> ParseResult<Vec<ParsedValue>> {
    let mut iter = values.iter().peekable();
    let mut result = Vec::new();

    // Parse the object type and index
    let selector = iter
        .next()
        .ok_or(ParseError::QuerySelectorMissing)
        .inspect_err(|err| {
            error!("{}", err);
        })?;
    let next_value = iter.peek();
    let index = if ObjectTypeSelector::is_object_type_indexable(selector) {
        // If the object type is indexable, expect an index
        match next_value {
            Some(RytmValue::Int(_)) => iter.next(),
            _ => {
                return Err(ParseError::QuerySelectorIndexMissingOrInvalid).inspect_err(|err| {
                    error!("{}", err);
                })?
            }
        }
    } else {
        None
    };

    let object_type_selector =
        ObjectTypeSelector::try_from((selector, index)).inspect_err(|err| {
            error!("{}", err);
        })?;

    // Add the object type to the result
    result.push(ParsedValue::ObjectType(object_type_selector));

    // Continue parsing the remainder of the command
    parse_remainder(command_type, &object_type_selector, &mut iter, &mut result)?;

    Ok(result)
}

/// Parses the remainder of the command after the object type and index (if any)
#[instrument]
fn parse_remainder<'a, I>(
    command_type: CommandType,
    object_type_selector: &ObjectTypeSelector,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    match object_type_selector {
        ObjectTypeSelector::Pattern(_) | ObjectTypeSelector::PatternWorkBuffer => {
            parse_pattern(command_type, iter, result)
        }
        ObjectTypeSelector::Kit(_) | ObjectTypeSelector::KitWorkBuffer => {
            parse_kit(command_type, iter, result)
        }
        ObjectTypeSelector::Sound(_) | ObjectTypeSelector::SoundWorkBuffer(_) => {
            parse_sound(command_type, iter, result)
        }
        ObjectTypeSelector::Global(_) | ObjectTypeSelector::GlobalWorkBuffer => {
            parse_global(command_type, iter, result)
        }
        ObjectTypeSelector::Settings => parse_settings(command_type, iter, result),
    }
}

/// Parses the command for 'pattern' or 'pattern_wb' object types
#[instrument]
fn parse_pattern<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    // Parse optional track index
    if let Some(&RytmValue::Int(track_index)) = iter.peek() {
        validate_index(track_index, 0, 12, "Track index")?;
        iter.next();
        result.push(ParsedValue::TrackIndex(*track_index as usize));
    }

    // Parse optional trig index
    if let Some(&RytmValue::Int(trig_index)) = iter.peek() {
        validate_index(trig_index, 0, 63, "Trig index")?;
        iter.next();
        result.push(ParsedValue::TrigIndex(*trig_index as usize));
    }

    // Handle plock operations if present
    match iter.peek() {
        Some(RytmValue::Symbol(op)) if is_plock_operation(op) => {
            let op = PlockOperation::from_str(op)?;
            iter.next();
            result.push(ParsedValue::PlockOperation(op));

            if iter.peek().is_some() {
                parse_identifier_or_enum(command_type, iter, result)
            } else {
                Err(ParseError::InvalidPlockOperation(
                    op.to_string(),
                    "This operation needs to be followed by an identifier or an enum.".to_owned(),
                ))
                .inspect_err(|err| {
                    error!("{}", err);
                })
            }
        }
        _ => parse_identifier_or_enum(command_type, iter, result),
    }
}

/// Parses the command for 'kit' or 'kit_wb' object types
#[instrument]
fn parse_kit<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    // If not an element symbol, fall back to identifier/enum parsing
    let Some(RytmValue::Symbol(element)) = iter.peek() else {
        return parse_identifier_or_enum(command_type, iter, result);
    };

    if !is_element(element) {
        return parse_identifier_or_enum(command_type, iter, result);
    }

    // Consume the element and add it to result
    iter.next();
    result.push(ParsedValue::Element(element.clone()));

    // Parse the required element index
    let index = match iter.next() {
        Some(RytmValue::Int(index)) => index,
        _ => {
            return Err(ParseError::ExpectedKitElementIndex(format!(
            "Expected element index after '{}': Kit elements must be followed by an integer index.",
            element
        )))
            .inspect_err(|err| {
                error!("{}", err);
            })
        }
    };

    // Handle different index types
    if element == SOUND {
        validate_index(index, 0, 11, "Kit sound index")?;
        result.push(ParsedValue::SoundIndex(*index as usize));
    } else {
        result.push(ParsedValue::ElementIndex(*index as usize));
    }

    // Only parse additional identifiers if there's more input
    if iter.peek().is_some() {
        if element == SOUND {
            parse_sound(command_type, iter, result)
        } else {
            parse_identifier_or_enum(command_type, iter, result)
        }
    } else {
        Ok(())
    }
}

/// Parses the command for 'sound' or 'sound_wb' object types
#[instrument]
fn parse_sound<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    let symbol = match iter.peek() {
        Some(RytmValue::Symbol(s)) => s,
        _ => {
            return Err(ParseError::UnexpectedEnd).inspect_err(|err| {
                error!("{}", err);
            })
        }
    };

    // If the symbol is a special one which requires a string parameter
    // TODO: Generalization of this is possible if there are more cases.
    if ["name"].contains(&symbol.as_str()) && command_type == CommandType::Set {
        result.push(ParsedValue::Identifier(symbol.clone()));
        iter.next();
        if let Some(RytmValue::Symbol(s)) = iter.next() {
            result.push(ParsedValue::ParameterString(s.clone()));
            return Ok(());
        } else {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid parameter '{}': name must be a symbol with maximum 15 characters long and use only ascii characters.",
                symbol
            ))).inspect_err(|err| {
                    error!("{}", err);
                });
        }
    }

    // The sound index is already included in ObjectTypeSelector
    // Proceed to parse identifier or enum
    parse_identifier_or_enum(command_type, iter, result)
}

/// Parses the command for 'global' or 'global_wb' object types
#[instrument]
fn parse_global<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    parse_identifier_or_enum(command_type, iter, result)
}

/// Parses the command for 'settings' object type
#[instrument]
fn parse_settings<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    parse_identifier_or_enum(command_type, iter, result)
}

// if is_plock_operation(s) {
//     result.push(ParsedValue::PlockOperation(s.clone()));
//     parse_identifier_or_enum(command_type, iter, result)

#[derive(Debug)]
enum EnumParseResult {
    Complete(String, Option<String>),
    RequiresValue(String),
    Invalid(String),
}

/// Parses an identifier or enum, and any following parameter
#[instrument]
#[log_errors]
fn parse_identifier_or_enum<'a, I>(
    command_type: CommandType,
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) -> ParseResult<()>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    let symbol = match iter.next() {
        Some(RytmValue::Symbol(s)) => s,
        _ => return Err(ParseError::UnexpectedEnd),
    };

    if is_identifier(symbol) {
        result.push(ParsedValue::Identifier(symbol.clone()));
        parse_optional_parameters(iter, result);
        return Ok(());
    }

    if !is_enum(symbol) {
        return Err(ParseError::InvalidToken(format!(
            "Unexpected symbol '{}'. Expected an identifier or enum.",
            symbol
        )));
    }

    // Handle enum parsing
    match parse_enum_value(symbol) {
        EnumParseResult::Complete(name, value) => {
            result.push(ParsedValue::Enum(name, value));
            Ok(())
        }
        EnumParseResult::RequiresValue(name) => {
            if command_type == CommandType::Set {
                Err(ParseError::InvalidFormat(format!(
                    "Enum '{name}:' requires a value. Try using '{name}:<your-value>' instead.",                
                )))
            } else {
                result.push(ParsedValue::Enum(name, None));
                Ok(())
            }
        }
        EnumParseResult::Invalid(s) => Err(ParseError::InvalidFormat(format!(
            "Invalid enum format: '{}'. Enums may only have the format of <enum-type>: or <enum-type>:<variant>",
            s
        ))),
    }
}

#[instrument]
fn parse_optional_parameters<'a, I>(
    iter: &mut std::iter::Peekable<I>,
    result: &mut Vec<ParsedValue>,
) where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    // Parse first parameter if present
    if let Some(param) = parse_single_parameter(iter) {
        result.push(param);

        // Parse second optional parameter if present
        if let Some(param2) = parse_single_parameter(iter) {
            result.push(param2);
        }
    }
}

#[instrument]
fn parse_single_parameter<'a, I>(iter: &mut std::iter::Peekable<I>) -> Option<ParsedValue>
where
    I: Iterator<Item = &'a RytmValue> + std::fmt::Debug,
{
    match iter.peek() {
        Some(RytmValue::Int(param)) => {
            iter.next();
            Some(ParsedValue::Parameter(Number::Int(*param)))
        }
        Some(RytmValue::Float(f)) => {
            let param = *f;
            iter.next();
            Some(ParsedValue::Parameter(Number::Float(param)))
        }
        _ => None,
    }
}

#[instrument]
fn parse_enum_value(s: &str) -> EnumParseResult {
    let mut parts = s.splitn(2, ':');

    let name = match parts.next() {
        Some(name) => name.to_string(),
        None => return EnumParseResult::Invalid(s.to_string()),
    };

    match parts.next() {
        Some(value) if !value.is_empty() => {
            EnumParseResult::Complete(name, Some(value.to_string()))
        }
        Some(_) => EnumParseResult::RequiresValue(name),
        None => {
            if s.ends_with(':') {
                EnumParseResult::RequiresValue(name.trim_end_matches(':').to_string())
            } else {
                EnumParseResult::Invalid(s.to_string())
            }
        }
    }
}

/// Validates that an index is within a specified range
#[instrument]
#[log_errors]
fn validate_index(index: &isize, min: isize, max: isize, name: &str) -> ParseResult<()> {
    if *index >= min && *index <= max {
        Ok(())
    } else {
        Err(ParseError::IndexOutOfRange(format!(
            "{index} is out of range for {}. {name} must be an integer between  {min} and {max}.",
            name.to_lowercase(),
        )))
    }
}

/// Checks if a string is a valid identifier
#[instrument]
fn is_identifier(s: &str) -> bool {
    is_valid_identifier(s)
}

#[instrument]
fn is_valid_identifier(s: &str) -> bool {
    lazy_static! {
        static ref IDENTIFIERS: HashSet<&'static str> = {
            let mut m = HashSet::new();

            // Common identifiers from object types
            api::object_type::OBJECT_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From settings_action_type
            api::settings_action_type::SETTINGS_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From global_action_type
            api::global_action_type::GLOBAL_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From kit_action_type
            api::kit_action_type::KIT_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            api::kit_element_type::KIT_ELEMENTS_ACTION.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From trig_action_type
            api::trig_action_type::TRIG_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From track_action_type
            api::track_action_type::TRACK_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From pattern_action_type
            api::pattern_action_type::PATTERN_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From sound_action_type
            api::sound_action_type::SOUND_ACTION_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From plock_type
            api::plock_type::PLOCK_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            m
        };
    }

    IDENTIFIERS.contains(s)
}

/// Checks if a string represents an enum
#[instrument]
fn is_enum(s: &str) -> bool {
    is_valid_enum_type(s)
}

#[instrument]
fn is_valid_enum_type(s: &str) -> bool {
    lazy_static! {
        static ref ENUM_TYPES: HashSet<&'static str> = {
            let mut m = HashSet::new();

            // From pattern_enum_type
            api::pattern_enum_type::PATTERN_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From track_enum_type
            api::track_enum_type::TRACK_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From trig_enum_type
            api::trig_enum_type::TRIG_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From kit_enum_type
            api::kit_enum_type::KIT_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            api::kit_element_type::KIT_ELEMENTS_ENUM.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From settings_enum_type
            api::settings_enum_type::SETTINGS_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From sound_enum_type
            api::sound_enum_type::SOUND_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            // From global_enum_type
            api::global_enum_type::GLOBAL_ENUM_TYPES.iter().for_each(|s| {
                m.insert(s.to_owned());
            });

            m
        };
    }

    let enum_type = if s.contains(':') {
        s.split(':').next().unwrap()
    } else {
        s
    };

    ENUM_TYPES.contains(enum_type)
}

/// Checks if a string is a plock operation
#[instrument]
fn is_plock_operation(s: &str) -> bool {
    matches!(
        s,
        api::plock_type::PLOCK_GET | api::plock_type::PLOCK_SET | api::plock_type::PLOCK_CLEAR
    )
}

/// Checks if a string is a valid element (e.g., kit elements)
#[instrument]
fn is_element(s: &str) -> bool {
    api::kit_element_type::KIT_ELEMENTS.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::RytmValue;

    fn parse(values: RytmValueList) -> ParseResult<Vec<ParsedValue>> {
        parse_command(&values, CommandType::Get)
    }

    #[test]
    fn test_valid_pattern_get_identifier() {
        // get pattern 1 masterlen
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("masterlen".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::Identifier("masterlen".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_get_enum() {
        // get pattern 1 speed:1x
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("speed:1x".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::Enum("speed".to_string(), Some("1x".to_string())),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_track_get_identifier() {
        // get pattern 1 0 deftrignote
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Symbol("deftrignote".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::TrackIndex(0),
                ParsedValue::Identifier("deftrignote".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_trig_get_identifier() {
        // get pattern 1 0 5 note
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Int(5),
            RytmValue::Symbol("note".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::TrackIndex(0),
                ParsedValue::TrigIndex(5),
                ParsedValue::Identifier("note".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_trig_plockget() {
        // get pattern 1 0 5 plockget amplev
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Int(5),
            RytmValue::Symbol("plockget".to_string()),
            RytmValue::Symbol("amplev".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::TrackIndex(0),
                ParsedValue::TrigIndex(5),
                ParsedValue::PlockOperation(PlockOperation::from_str("plockget").unwrap()),
                ParsedValue::Identifier("amplev".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_kit_get_identifier() {
        // get kit 10 name
        let values = vec![
            RytmValue::Symbol("kit".to_string()),
            RytmValue::Int(10),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Kit(10)),
                ParsedValue::Identifier("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_kit_element_get() {
        // get kit 10 tracklevel 0
        let values = vec![
            RytmValue::Symbol("kit".to_string()),
            RytmValue::Int(10),
            RytmValue::Symbol("tracklevel".to_string()),
            RytmValue::Int(0),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Kit(10)),
                ParsedValue::Element("tracklevel".to_string()),
                ParsedValue::ElementIndex(0),
            ]
        );
    }

    #[test]
    fn test_valid_kit_sound_get() {
        // get kit 10 sound 2 amplev
        let values = vec![
            RytmValue::Symbol("kit".to_string()),
            RytmValue::Int(10),
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(2),
            RytmValue::Symbol("amplev".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Kit(10)),
                ParsedValue::Element("sound".to_string()),
                ParsedValue::SoundIndex(2),
                ParsedValue::Identifier("amplev".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_sound_get_identifier() {
        // get sound 5 amplev
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(5),
            RytmValue::Symbol("amplev".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(5)),
                ParsedValue::Identifier("amplev".to_string()),
            ]
        );
    }

    #[test]
    fn test_invalid_object_type() {
        // get invalidobject 1 name
        let values = vec![
            RytmValue::Symbol("invalidobject".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_query_selector_missing_index() {
        // get pattern name
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::QuerySelectorIndexMissingOrInvalid) = result {
            // Expected error
        } else {
            panic!("Expected QuerySelectorIndexMissingOrInvalid error");
        }
    }

    #[test]
    fn test_invalid_identifier() {
        // get pattern 1 invalididentifier
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("invalididentifier".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::InvalidToken(_)) = result {
            // Expected error
        } else {
            panic!("Expected InvalidToken error");
        }
    }

    #[test]
    fn test_valid_settings_get_enum() {
        // get settings sequencermode:
        let values = vec![
            RytmValue::Symbol("settings".to_string()),
            RytmValue::Symbol("sequencermode:".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Settings),
                ParsedValue::Enum("sequencermode".to_string(), None),
            ]
        );
    }

    #[test]
    fn test_valid_settings_get_enum_with_value() {
        // get settings sequencermode:normal
        let values = vec![
            RytmValue::Symbol("settings".to_string()),
            RytmValue::Symbol("sequencermode:normal".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Settings),
                ParsedValue::Enum("sequencermode".to_string(), Some("normal".to_string())),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_set_parameter() {
        // set pattern 1 masterlen 64
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("masterlen".to_string()),
            RytmValue::Int(64),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::Identifier("masterlen".to_string()),
                ParsedValue::Parameter(Number::Int(64)),
            ]
        );
    }

    #[test]
    fn test_valid_sound_set_parameter_float() {
        // set sound 0 lfodepth -12.5
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("lfodepth".to_string()),
            RytmValue::Float(-12.5),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(0)),
                ParsedValue::Identifier("lfodepth".to_string()),
                ParsedValue::Parameter(Number::Float(-12.5)),
            ]
        );
    }

    #[test]
    fn test_invalid_track_index() {
        // get pattern 1 13 deftrignote
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(13),
            RytmValue::Symbol("deftrignote".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::IndexOutOfRange(_)) = result {
            // Expected error
        } else {
            panic!("Expected IndexOutOfRange error");
        }
    }

    #[test]
    fn test_invalid_trig_index() {
        // get pattern 1 0 64 note
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Int(64),
            RytmValue::Symbol("note".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::IndexOutOfRange(_)) = result {
            // Expected error
        } else {
            panic!("Expected IndexOutOfRange error");
        }
    }

    #[test]
    fn test_missing_object_type() {
        // get
        let values = vec![];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::QuerySelectorMissing) = result {
            // Expected error
        } else {
            panic!("Expected QuerySelectorMissing error");
        }
    }

    #[test]
    fn test_invalid_enum_format() {
        // get pattern 1 speed
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Symbol("speed".to_string()), // Missing ':'
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::InvalidFormat(_)) = result {
            // Expected error
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_valid_pattern_min_index() {
        // get pattern 0 name
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(0)),
                ParsedValue::Identifier("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_max_index() {
        // get pattern 127 name
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(127),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(127)),
                ParsedValue::Identifier("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_invalid_pattern_index_out_of_range() {
        // get pattern 128 name
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(128),
            RytmValue::Symbol("name".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());

        if let Err(ParseError::InvalidIndexRange { .. }) = result {
            // Expected error
        } else {
            panic!("Expected InvalidIndexRange error");
        }
    }

    #[test]
    fn test_valid_pattern_enum_with_track() {
        // get pattern 1 0 rootnote:c
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Symbol("rootnote:c".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::TrackIndex(0),
                ParsedValue::Enum("rootnote".to_string(), Some("c".to_string())),
            ]
        );
    }

    #[test]
    fn test_valid_enum_without_value() {
        // get settings sequencermode:
        let values = vec![
            RytmValue::Symbol("settings".to_string()),
            RytmValue::Symbol("sequencermode:".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Settings),
                ParsedValue::Enum("sequencermode".to_string(), None),
            ]
        );
    }

    #[test]
    fn test_invalid_command() {
        // foo bar baz
        let values = vec![
            RytmValue::Symbol("foo".to_string()),
            RytmValue::Symbol("bar".to_string()),
            RytmValue::Symbol("baz".to_string()),
        ];
        let result = parse(values.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_sound_set_float_parameter() {
        // set sound 0 sampstart 12.34
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("sampstart".to_string()),
            RytmValue::Float(12.34),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(0)),
                ParsedValue::Identifier("sampstart".to_string()),
                ParsedValue::Parameter(Number::Float(12.34)),
            ]
        );
    }

    #[test]
    fn test_valid_sound_set_negative_parameter() {
        // set sound 0 sampfinetune -64
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("sampfinetune".to_string()),
            RytmValue::Int(-64),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(0)),
                ParsedValue::Identifier("sampfinetune".to_string()),
                ParsedValue::Parameter(Number::Int(-64)),
            ]
        );
    }

    #[test]
    fn test_valid_pattern_trig_plockset_enum() {
        // set pattern 1 0 5 plockset filtertype:lp2
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            RytmValue::Int(5),
            RytmValue::Symbol("plockset".to_string()),
            RytmValue::Symbol("filtertype:lp2".to_string()),
        ];
        let result = parse(values.into()).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Pattern(1)),
                ParsedValue::TrackIndex(0),
                ParsedValue::TrigIndex(5),
                ParsedValue::PlockOperation(PlockOperation::from_str("plockset").unwrap()),
                ParsedValue::Enum("filtertype".to_string(), Some("lp2".to_string())),
            ]
        );
    }

    #[test]
    fn test_unexpected_end_of_input() {
        // get pattern 1 0
        let values = vec![
            RytmValue::Symbol("pattern".to_string()),
            RytmValue::Int(1),
            RytmValue::Int(0),
            // Missing identifier or enum
        ];
        let result = parse(values.into());
        assert!(result.is_err());
        if let Err(ParseError::UnexpectedEnd) = result {
            // Expected error
        } else {
            panic!("Expected UnexpectedEnd error");
        }
    }

    #[test]

    fn test_set_focused_parse_requires_enum_values() {
        // set focused sequencermode
        let values = vec![
            RytmValue::Symbol("focused:".to_string()),
            RytmValue::Symbol("sequencermode".to_string()),
        ];

        let v: RytmValueList = values.into();
        let result = parse_command(&v, CommandType::Set);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_two_parameters() {
        // set kit 10 distamount 64 127
        let values = vec![
            RytmValue::Symbol("kit".to_string()),
            RytmValue::Int(10),
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("velmodamt".to_string()),
            RytmValue::Int(2),
            RytmValue::Int(2),
        ];
        let v: RytmValueList = values.into();
        let result = parse_command(&v, CommandType::Set).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Kit(10)),
                ParsedValue::Element("sound".to_string()),
                ParsedValue::SoundIndex(0),
                ParsedValue::Identifier("velmodamt".to_string()),
                ParsedValue::Parameter(Number::Int(2)),
                ParsedValue::Parameter(Number::Int(2)),
            ]
        );
    }

    #[test]
    fn test_sound_naming() {
        // set sound 0 name
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("name".to_string()),
            RytmValue::Symbol("hello".to_string()),
        ];
        let v: RytmValueList = values.into();
        let result = parse_command(&v, CommandType::Set).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(0)),
                ParsedValue::Identifier("name".to_string()),
                ParsedValue::ParameterString("hello".to_string()),
            ]
        );

        // get sound 0 name
        let values = vec![
            RytmValue::Symbol("sound".to_string()),
            RytmValue::Int(0),
            RytmValue::Symbol("name".to_string()),
        ];
        let v: RytmValueList = values.into();
        let result = parse_command(&v, CommandType::Get).unwrap();
        assert_eq!(
            result,
            vec![
                ParsedValue::ObjectType(ObjectTypeSelector::Sound(0)),
                ParsedValue::Identifier("name".to_string()),
            ]
        );
    }
}
