use super::Response;
use crate::{
    error::{
        number_or_set_error, EnumError::InvalidEnumType, GetError, IdentifierError,
        RytmObjectError, SetError,
    },
    parse::types::{Number, ParsedValue},
    types::CommandType,
    value::RytmValue,
    RytmObject,
};
use error_logger_macro::log_errors;
use rytm_rs::object::Settings;
use tracing::{error, instrument};

#[instrument(skip(rytm))]
pub fn handle(
    rytm: &RytmObject,
    tokens: Vec<ParsedValue>,
    command_type: CommandType,
) -> Result<Response, RytmObjectError> {
    let mut guard = rytm.project.lock();
    let mut tokens = tokens[1..].iter();
    let next_token = tokens.next();

    match command_type {
        CommandType::Get => {
            let object = guard.settings();
            match next_token {
                Some(ParsedValue::Enum(variant, _)) => Ok(Response::Common {
                    index: 0,
                    key: variant.into(),
                    value: get_enum(object, variant)?,
                }),
                Some(ParsedValue::Identifier(action)) => Ok(Response::Common {
                    index: 0,
                    key: action.into(),
                    value: get_action(object, &mut tokens, action)?,
                }),
                _ => {
                    unreachable!("Parser should take care of this. Invalid getter format.")
                }
            }
        }
        CommandType::Set => {
            let object = guard.settings_mut();
            match next_token {
                Some(ParsedValue::Enum(variant, value)) => set_enum(object, variant, value),
                Some(ParsedValue::Identifier(action)) => set_action(object, &mut tokens, action),
                _ => {
                    unreachable!("Parser should take care of this. Invalid setter format.")
                }
            }
        }
    }
}

#[instrument(skip(object))]
#[log_errors]
fn get_enum(object: &Settings, variant: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::settings_enum_type::*;
    let result: &str = match variant {
        PARAMETER_MENU_ITEM => object.selected_parameter_menu_item().into(),
        FX_PARAMETER_MENU_ITEM => object.selected_fx_menu_item().into(),
        SEQUENCER_MODE => object.selected_mode().into(),
        PATTERN_MODE => object.selected_pattern_mode().into(),
        SAMPLE_RECORDER_SOURCE => object.sample_recorder_source().into(),
        SAMPLE_RECORDER_RECORDING_LENGTH => object.sample_recorder_recording_length().into(),
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn get_action(
    object: &Settings,
    tokens: &mut std::slice::Iter<ParsedValue>,
    action: &str,
) -> Result<RytmValue, RytmObjectError> {
    use crate::api::settings_action_type::*;
    let result: RytmValue = match action {
        BPM_PROJECT => RytmValue::from(f64::from(object.bpm())),
        SELECTED_TRACK => RytmValue::from(object.selected_track() as isize),
        SELECTED_PAGE => RytmValue::from(object.selected_page() as isize),
        MUTE => {
            let Some(ParsedValue::Parameter(Number::Int(param))) = tokens.next() else {
                return Err(GetError::InvalidFormat(
                    "mute should be followed by an integer sound index.".into(),
                )
                .into());
            };
            RytmValue::from(isize::from(object.is_sound_muted(*param as usize)?))
        }
        FIXED_VELOCITY_ENABLE => RytmValue::from(isize::from(object.fixed_velocity_enabled())),
        FIXED_VELOCITY_AMOUNT => RytmValue::from(object.fixed_velocity_amount() as isize),
        SAMPLE_RECORDER_THR => RytmValue::from(object.sample_recorder_threshold() as isize),
        SAMPLE_RECORDER_MONITOR_ENABLE => {
            RytmValue::from(isize::from(object.sample_recorder_monitor_enabled()))
        }
        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result)
}

#[instrument(skip(object))]
#[log_errors]
fn set_enum(
    object: &mut Settings,
    variant: &str,
    value: &Option<String>,
) -> Result<Response, RytmObjectError> {
    let enum_value = value
        .clone()
        .ok_or_else(|| SetError::InvalidFormat("Enum value not provided".into()))?;

    use crate::api::settings_enum_type::*;
    match variant {
        PARAMETER_MENU_ITEM => {
            object.set_selected_parameter_menu_item(enum_value.as_str().try_into()?);
        }
        FX_PARAMETER_MENU_ITEM => {
            object.set_selected_fx_menu_item(enum_value.as_str().try_into()?);
        }
        SEQUENCER_MODE => {
            object.set_selected_mode(enum_value.as_str().try_into()?);
        }
        PATTERN_MODE => {
            object.set_selected_pattern_mode(enum_value.as_str().try_into()?);
        }
        SAMPLE_RECORDER_SOURCE => {
            object.set_sample_recorder_source(enum_value.as_str().try_into()?);
        }
        SAMPLE_RECORDER_RECORDING_LENGTH => {
            object.set_sample_recorder_recording_length(enum_value.as_str().try_into()?);
        }
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn set_action(
    object: &mut Settings,
    tokens: &mut std::slice::Iter<ParsedValue>,
    maybe_action: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::settings_action_type::*;

    let param = number_or_set_error(tokens)?;

    match maybe_action {
        BPM_PROJECT => {
            object.set_bpm(param.get_float() as f32)?;
        }
        SELECTED_TRACK => {
            object.set_selected_track(param.get_int() as usize)?;
        }
        SELECTED_PAGE => {
            object.set_selected_page(param.get_int() as usize)?;
        }
        MUTE => {
            object.mute_sound(param.get_int() as usize)?;
        }
        UNMUTE => {
            object.unmute_sound(param.get_int() as usize)?;
        }
        FIXED_VELOCITY_ENABLE => {
            object.set_fixed_velocity_enable(param.get_bool_from_0_or_1(FIXED_VELOCITY_ENABLE)?);
        }
        FIXED_VELOCITY_AMOUNT => {
            object.set_fixed_velocity_amount(param.get_int() as usize)?;
        }
        SAMPLE_RECORDER_THR => {
            object.set_sample_recorder_threshold(param.get_int() as usize)?;
        }
        SAMPLE_RECORDER_MONITOR_ENABLE => {
            object.set_sample_recorder_monitor_enable(
                param.get_bool_from_0_or_1(SAMPLE_RECORDER_MONITOR_ENABLE)?,
            );
        }
        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}
