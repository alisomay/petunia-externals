use error_logger_macro::log_errors;
use rytm_rs::object::pattern::track::trig::HoldsTrigFlags;
use rytm_rs::object::pattern::track::Track;
use rytm_rs::object::pattern::Trig;
use rytm_rs::object::Pattern;
use tracing::instrument;

use crate::error::EnumError::InvalidEnumType;
use crate::error::{number_or_set_error, GetError, IdentifierError, RytmObjectError};
use crate::parse::types::ParsedValue;
use crate::types::CommandType;
use crate::value::RytmValue;
use crate::RytmObject;
use tracing::error;

use super::plock::handle_plock_commands;
use super::Response;

#[instrument(skip(rytm))]
pub fn handle(
    rytm: &RytmObject,
    tokens: Vec<ParsedValue>,
    index: Option<usize>,
    command_type: CommandType,
) -> Result<Response, RytmObjectError> {
    let mut guard = rytm.project.lock();

    let mut tokens = tokens[1..].iter();

    match tokens.next() {
        Some(ParsedValue::TrackIndex(track_index)) => match tokens.next() {
            Some(ParsedValue::TrigIndex(trig_index)) => match tokens.next() {
                Some(ParsedValue::PlockOperation(op)) => {
                    // Treat as plock
                    let object = if let Some(i) = index {
                        &mut guard.patterns_mut()[i].tracks_mut()[*track_index].trigs_mut()
                            [*trig_index]
                    } else {
                        &mut guard.work_buffer_mut().pattern_mut().tracks_mut()[*track_index]
                            .trigs_mut()[*trig_index]
                    };

                    // TODO: Maybe plockget commands can also return parent indexes.
                    handle_plock_commands(object, &mut tokens, *trig_index, *op, command_type)
                }
                Some(ident_or_enum) => {
                    // Treat as trig and apply the command.
                    match command_type {
                        CommandType::Get => {
                            let object = index.map_or_else(
                                || {
                                    &guard.work_buffer().pattern().tracks()[*track_index].trigs()
                                        [*trig_index]
                                },
                                |i| {
                                    &guard.patterns()[i].tracks()[*track_index].trigs()[*trig_index]
                                },
                            );
                            match ident_or_enum {
                                ParsedValue::Enum(variant, _) => Ok(Response::Trig {
                                    pattern_index: index.unwrap_or(0),
                                    track_index: *track_index,
                                    trig_index: object.index(),
                                    key: variant.into(),
                                    value: trig_get_enum(object, variant)?,
                                }),
                                ParsedValue::Identifier(action) => Ok(Response::Trig {
                                    pattern_index: index.unwrap_or(0),
                                    track_index: *track_index,
                                    trig_index: object.index(),
                                    key: action.into(),
                                    value: trig_get_action(object, action)?,
                                }),
                                _ => {
                                    unreachable!(
                                        "Parser should take care of this. Invalid getter format."
                                    )
                                }
                            }
                        }
                        CommandType::Set => {
                            let object = if let Some(i) = index {
                                &mut guard.patterns_mut()[i].tracks_mut()[*track_index].trigs_mut()
                                    [*trig_index]
                            } else {
                                &mut guard.work_buffer_mut().pattern_mut().tracks_mut()
                                    [*track_index]
                                    .trigs_mut()[*trig_index]
                            };

                            match ident_or_enum {
                                ParsedValue::Enum(variant, Some(value)) => {
                                    trig_set_enum(object, variant, value)
                                }
                                ParsedValue::Identifier(action) => {
                                    trig_set_action(object, &mut tokens, action)
                                }
                                _ => {
                                    unreachable!(
                                        "Parser should take care of this. Invalid setter format."
                                    )
                                }
                            }
                        }
                    }
                }
                None => Err(GetError::InvalidFormat(
                    "A trig index should be followed by an identifier enum or a plock command."
                        .into(),
                )
                .into()),
            },
            Some(ident_or_enum) => {
                // Treat as track and apply the command.
                match command_type {
                    CommandType::Get => {
                        let object = index.map_or_else(
                            || &guard.work_buffer().pattern().tracks()[*track_index],
                            |i| &guard.patterns()[i].tracks()[*track_index],
                        );
                        match ident_or_enum {
                            ParsedValue::Enum(variant, _) => Ok(Response::Track {
                                pattern_index: index.unwrap_or(0),
                                track_index: object.index(),
                                key: variant.into(),
                                value: track_get_enum(object, variant)?,
                            }),
                            ParsedValue::Identifier(action) => Ok(Response::Track {
                                pattern_index: index.unwrap_or(0),
                                track_index: object.index(),
                                key: action.into(),
                                value: track_get_action(object, action)?,
                            }),
                            _ => {
                                unreachable!(
                                    "Parser should take care of this. Invalid getter format."
                                )
                            }
                        }
                    }
                    CommandType::Set => {
                        let object = if let Some(i) = index {
                            &mut guard.patterns_mut()[i].tracks_mut()[*track_index]
                        } else {
                            &mut guard.work_buffer_mut().pattern_mut().tracks_mut()[*track_index]
                        };

                        match ident_or_enum {
                            ParsedValue::Enum(variant, Some(value)) => {
                                track_set_enum(object, variant, value)
                            }
                            ParsedValue::Identifier(action) => {
                                track_set_action(object, &mut tokens, action)
                            }
                            _ => {
                                unreachable!(
                                    "Parser should take care of this. Invalid setter format."
                                )
                            }
                        }
                    }
                }
            }
            None => Err(GetError::InvalidFormat(
                "A track index should be followed by a identifier enum or trig index.".into(),
            )
            .into()),
        },
        Some(ident_or_enum) => {
            // Treat as pattern and apply the command.
            match command_type {
                CommandType::Get => {
                    let object = index
                        .map_or_else(|| guard.work_buffer().pattern(), |i| &guard.patterns()[i]);
                    match ident_or_enum {
                        ParsedValue::Enum(variant, _) => Ok(Response::Common {
                            index: object.index(),
                            key: variant.into(),
                            value: pattern_get_enum(object, variant)?,
                        }),
                        ParsedValue::Identifier(action) => Ok(Response::Common {
                            index: object.index(),
                            key: action.into(),
                            value: pattern_get_action(object, action)?,
                        }),
                        _ => {
                            unreachable!("Parser should take care of this. Invalid getter format.")
                        }
                    }
                }
                CommandType::Set => {
                    let object = if let Some(i) = index {
                        &mut guard.patterns_mut()[i]
                    } else {
                        guard.work_buffer_mut().pattern_mut()
                    };

                    match ident_or_enum {
                        ParsedValue::Enum(variant, Some(value)) => {
                            pattern_set_enum(object, variant, value)
                        }
                        ParsedValue::Identifier(action) => {
                            pattern_set_action(object, &mut tokens, action)
                        }
                        _ => {
                            unreachable!("Parser should take care of this. Invalid setter format.")
                        }
                    }
                }
            }
        }
        None => Err(GetError::InvalidFormat(
            "A pattern index should be followed by a identifier enum or track index.".into(),
        )
        .into()),
    }
}

#[instrument(skip(object))]
#[log_errors]
fn pattern_get_enum(object: &Pattern, variant: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::pattern_enum_type::*;
    let result: &str = match variant {
        SPEED => object.speed().into(),
        TIME_MODE => object.time_mode().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };
    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn pattern_get_action(object: &Pattern, action: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::pattern_action_type::*;
    let result = match action {
        IS_WORK_BUFFER => isize::from(object.is_work_buffer_pattern()),
        VERSION => object.structure_version() as isize,
        INDEX => object.index() as isize,
        MASTER_LENGTH => object.master_length() as isize,
        MASTER_CHANGE => object.master_change() as isize,
        KIT_NUMBER => object.kit_number() as isize,
        SWING_AMOUNT => object.swing_amount() as isize,
        GLOBAL_QUANTIZE => object.global_quantize() as isize,
        BPM => object.bpm() as isize,

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn track_get_enum(object: &Track, variant: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::track_enum_type::*;
    let result: &str = match variant {
        ROOT_NOTE => object.root_note().into(),
        PAD_SCALE => object.pad_scale().into(),
        DEFAULT_NOTE_LENGTH => object.default_trig_note_length().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn track_get_action(object: &Track, action: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::track_action_type::*;
    let result = match action {
        INDEX => object.index(),
        OWNER_INDEX => object.owner_pattern_index(),
        DEF_TRIG_NOTE => object.default_trig_note(),
        DEF_TRIG_VELOCITY => object.default_trig_velocity(),
        DEF_TRIG_PROB => object.default_trig_probability(),
        NUMBER_OF_STEPS => object.number_of_steps(),
        QUANTIZE_AMOUNT => object.quantize_amount(),
        SENDS_MIDI => object.sends_midi().into(),
        EUCLIDEAN_MODE => object.euclidean_mode().into(),
        EUCLIDEAN_PL1 => object.euclidean_pl1(),
        EUCLIDEAN_PL2 => object.euclidean_pl2(),
        EUCLIDEAN_RO1 => object.euclidean_ro1(),
        EUCLIDEAN_RO2 => object.euclidean_ro2(),
        EUCLIDEAN_TRO => object.euclidean_tro(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok((result as isize).into())
}

#[instrument(skip(object))]
#[log_errors]
fn trig_get_enum(object: &Trig, variant: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::trig_enum_type::*;
    let result: &str = match variant {
        MICRO_TIME => object.micro_timing().into(),
        NOTE_LENGTH => object.note_length().into(),
        RETRIG_LENGTH => object.retrig_length().into(),
        RETRIG_RATE => object.retrig_rate().into(),
        TRIG_CONDITION => object.trig_condition().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn trig_get_action(object: &Trig, action: &str) -> Result<RytmValue, RytmObjectError> {
    use crate::api::trig_action_type::*;
    let result = match action {
        ENABLE => object.enabled_trig().into(),
        RETRIG => object.enabled_retrig().into(),
        MUTE => object.enabled_mute().into(),
        ACCENT => object.enabled_accent().into(),
        SWING => object.enabled_swing().into(),
        SLIDE => object.enabled_slide().into(),
        // TODO: Do the rest of the flags..
        NOTE => object.note() as isize,
        VELOCITY => object.velocity() as isize,
        RETRIG_VELOCITY_OFFSET => object.retrig_velocity_offset(),
        SOUND_LOCK => object.sound_lock() as isize,

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn pattern_set_enum(
    object: &mut Pattern,
    maybe_enum: &str,
    value: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::pattern_enum_type::*;

    match maybe_enum {
        SPEED => object.set_speed(value.try_into()?),
        TIME_MODE => object.set_time_mode(value.try_into()?),
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn pattern_set_action(
    object: &mut Pattern,
    tokens: &mut std::slice::Iter<ParsedValue>,
    maybe_action: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::pattern_action_type::*;

    let param = number_or_set_error(tokens)?;

    match maybe_action {
        MASTER_LENGTH => {
            object.set_master_length(param.get_int() as usize)?;
        }
        MASTER_CHANGE => {
            object.set_master_change(param.get_int() as usize)?;
        }
        KIT_NUMBER => {
            object.set_kit_number(param.get_int() as usize)?;
        }
        SWING_AMOUNT => {
            object.set_swing_amount(param.get_int() as usize)?;
        }
        GLOBAL_QUANTIZE => {
            object.set_global_quantize(param.get_int() as usize)?;
        }
        BPM => {
            object.set_bpm(param.get_float() as f32)?;
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn track_set_enum(
    object: &mut Track,
    maybe_enum: &str,
    value: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::track_enum_type::*;

    match maybe_enum {
        ROOT_NOTE => object.set_root_note(value.try_into()?),
        PAD_SCALE => object.set_pad_scale(value.try_into()?),
        DEFAULT_NOTE_LENGTH => object.set_default_trig_note_length(value.try_into()?),
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn track_set_action(
    object: &mut Track,
    tokens: &mut std::slice::Iter<ParsedValue>,
    maybe_action: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::track_action_type::*;

    let param = number_or_set_error(tokens)?;

    match maybe_action {
        DEF_TRIG_NOTE => {
            object.set_default_trig_note(param.get_int() as usize)?;
        }
        DEF_TRIG_VELOCITY => {
            object.set_default_trig_velocity(param.get_int() as usize)?;
        }
        DEF_TRIG_PROB => {
            object.set_default_trig_probability(param.get_int() as usize)?;
        }
        NUMBER_OF_STEPS => {
            object.set_number_of_steps(param.get_int() as usize)?;
        }
        QUANTIZE_AMOUNT => {
            object.set_quantize_amount(param.get_int() as usize)?;
        }
        SENDS_MIDI => {
            object.set_sends_midi(param.get_bool_from_0_or_1(SENDS_MIDI)?);
        }
        EUCLIDEAN_MODE => {
            object.set_euclidean_mode(param.get_bool_from_0_or_1(EUCLIDEAN_MODE)?);
        }
        EUCLIDEAN_PL1 => {
            object.set_euclidean_pl1(param.get_int() as usize)?;
        }
        EUCLIDEAN_PL2 => {
            object.set_euclidean_pl2(param.get_int() as usize)?;
        }
        EUCLIDEAN_RO1 => {
            object.set_euclidean_ro1(param.get_int() as usize)?;
        }
        EUCLIDEAN_RO2 => {
            object.set_euclidean_ro2(param.get_int() as usize)?;
        }
        EUCLIDEAN_TRO => {
            object.set_euclidean_tro(param.get_int() as usize)?;
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn trig_set_enum(
    object: &mut Trig,
    maybe_enum: &str,
    value: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::trig_enum_type::*;

    match maybe_enum {
        MICRO_TIME => object.set_micro_timing(value.try_into()?),
        NOTE_LENGTH => object.set_note_length(value.try_into()?),
        RETRIG_LENGTH => object.set_retrig_length(value.try_into()?),
        RETRIG_RATE => object.set_retrig_rate(value.try_into()?),
        TRIG_CONDITION => object.set_trig_condition(value.try_into()?),
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn trig_set_action(
    object: &mut Trig,
    tokens: &mut std::slice::Iter<ParsedValue>,
    maybe_action: &str,
) -> Result<Response, RytmObjectError> {
    use crate::api::trig_action_type::*;

    let param = number_or_set_error(tokens)?;

    match maybe_action {
        ENABLE => {
            object.set_trig_enable(param.get_bool_from_0_or_1(ENABLE)?);
        }
        RETRIG => {
            object.set_retrig(param.get_bool_from_0_or_1(RETRIG)?);
        }
        MUTE => {
            object.set_mute(param.get_bool_from_0_or_1(MUTE)?);
        }
        ACCENT => {
            object.set_accent(param.get_bool_from_0_or_1(ACCENT)?);
        }
        SWING => {
            object.set_swing(param.get_bool_from_0_or_1(SWING)?);
        }
        SLIDE => {
            object.set_slide(param.get_bool_from_0_or_1(SLIDE)?);
        }
        NOTE => object.set_note(param.get_int() as usize)?,
        VELOCITY => object.set_velocity(param.get_int() as usize)?,
        RETRIG_VELOCITY_OFFSET => object.set_retrig_velocity_offset(param.get_int())?,
        SOUND_LOCK => object.set_sound_lock(param.get_int() as usize)?,

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}
