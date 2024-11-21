use error_logger_macro::log_errors;
use rytm_rs::object::Kit;
use tracing::instrument;

use crate::api::kit_action_type::*;
use crate::api::kit_element_type::*;
use crate::api::kit_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::{GetError, IdentifierError, RytmObjectError, SetError};
use crate::parse::types::{Number, ParsedValue};
use crate::types::CommandType;
use crate::value::RytmValue;
use crate::RytmObject;
use std::convert::TryInto;
use tracing::error;

use super::sound;
use super::sound::SoundSource;
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

    let next_token = tokens.next();

    match command_type {
        CommandType::Get => {
            let object = index.map_or_else(|| guard.work_buffer().kit(), |i| &guard.kits()[i]);

            match next_token {
                Some(ParsedValue::Enum(variant, value)) => Ok(Response::Common {
                    index: object.index(),
                    key: variant.into(),
                    value: get_enum(object, variant, value)?,
                }),
                Some(ParsedValue::Identifier(action)) => Ok(Response::Common {
                    index: object.index(),
                    key: action.into(),
                    value: get_action(object, &mut tokens, action)?,
                }),
                Some(ParsedValue::Element(element)) => {
                    if element == SOUND {
                        if let Some(ParsedValue::SoundIndex(i)) = tokens.next() {
                            let res = sound::handle(
                                rytm,
                                tokens.cloned().collect::<Vec<ParsedValue>>(),
                                *i,
                                SoundSource::Kit(object),
                                command_type,
                            )?;

                            let Response::Common {
                                index: sound_index,
                                key,
                                value,
                            } = res
                            else {
                                unreachable!("Sound should return a common response.");
                            };

                            return Ok(Response::KitElement {
                                kit_index: object.index(),
                                element_index: sound_index,
                                element_type: key,
                                value,
                            });
                        }

                        return Err(GetError::InvalidFormat(
                            "a kit sound must be followed by an index.".into(),
                        )
                        .into());
                    }

                    if let Some(ParsedValue::ElementIndex(element_index)) = tokens.next() {
                        Ok(Response::KitElement {
                            kit_index: object.index(),
                            element_index: *element_index,
                            element_type: element.into(),
                            value: get_kit_element_value(object, element, *element_index)?,
                        })
                    } else {
                        Err(GetError::InvalidFormat(
                            "a kit element must be followed by an index.".into(),
                        )
                        .into())
                    }
                }
                _ => {
                    unreachable!("Parser should take care of this. Invalid getter format.")
                }
            }
        }
        CommandType::Set => {
            let object = if let Some(i) = index {
                &mut guard.kits_mut()[i]
            } else {
                guard.work_buffer_mut().kit_mut()
            };

            match next_token {
                Some(ParsedValue::Enum(variant, value)) => {
                    set_enum(object, variant, value, tokens.next())
                }
                Some(ParsedValue::Identifier(action)) => set_action(object, action, &mut tokens),
                Some(ParsedValue::Element(element)) => {
                    if element == SOUND {
                        if let Some(ParsedValue::SoundIndex(i)) = tokens.next() {
                            let res = sound::handle(
                                rytm,
                                tokens.cloned().collect::<Vec<ParsedValue>>(),
                                *i,
                                SoundSource::KitMut(object),
                                command_type,
                            )?;
                            return Ok(res);
                        }

                        return Err(SetError::InvalidFormat(
                            "a kit sound must be followed by an index.".into(),
                        )
                        .into());
                    }

                    if let Some(ParsedValue::ElementIndex(element_index)) = tokens.next() {
                        set_kit_element_value(object, element, *element_index, &mut tokens)
                    } else {
                        Err(SetError::InvalidFormat(
                            "a kit element must be followed by an index.".into(),
                        )
                        .into())
                    }
                }
                _ => {
                    unreachable!("Parser should take care of this. Invalid getter format.")
                }
            }
        }
    }
}

#[instrument(skip(object))]
#[log_errors]
fn get_kit_element_value(
    object: &Kit,
    element: &str,
    index: usize,
) -> Result<RytmValue, RytmObjectError> {
    match element {
        TRACK_LEVEL => Ok((object.track_level(index)? as isize).into()),
        TRACK_RETRIG_RATE => {
            Ok(Into::<&str>::into(object.track_retrig_settings(index)?.rate()).into())
        }
        TRACK_RETRIG_LENGTH => {
            Ok(Into::<&str>::into(object.track_retrig_settings(index)?.length()).into())
        }
        TRACK_RETRIG_VEL_OFFSET => {
            Ok((object.track_retrig_settings(index)?.velocity_curve() as isize).into())
        }
        TRACK_RETRIG_ALWAYS_ON => {
            Ok(isize::from(object.track_retrig_settings(index)?.always_on()).into())
        }

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

#[instrument(skip(object))]
#[log_errors]
fn set_kit_element_value(
    object: &mut Kit,
    element: &str,
    index: usize,
    tokens: &mut std::slice::Iter<ParsedValue>,
) -> Result<Response, RytmObjectError> {
    let next_token = tokens.next().ok_or_else(|| {
        SetError::InvalidFormat("A parameter or enum must be provided to set a kit element.".into())
    })?;

    match element {
        TRACK_LEVEL => {
            let ParsedValue::Parameter(param) = next_token else {
                return Err(SetError::InvalidFormat(format!(
                    "{TRACK_LEVEL} should have an integer parameter."
                ))
                .into());
            };
            object.set_track_level(index, param.get_int() as usize)?;

            Ok(Response::Ok)
        }
        TRACK_RETRIG_RATE => {
            let err = SetError::InvalidFormat(format!(
                "{TRACK_RETRIG_RATE} should have an enum parameter. E.g. {TRACK_RETRIG_RATE}:1/1"
            ));

            let ParsedValue::Enum(enum_type, Some(value)) = next_token else {
                return Err(err.into());
            };

            if enum_type != TRACK_RETRIG_RATE {
                return Err(err.into());
            }

            object
                .track_retrig_settings_mut(index)?
                .set_rate(value.as_str().try_into()?);

            Ok(Response::Ok)
        }
        TRACK_RETRIG_LENGTH => {
            let err = SetError::InvalidFormat(
                format!("{TRACK_RETRIG_LENGTH} should have an enum parameter. E.g. {TRACK_RETRIG_LENGTH}:3.63"),
            );

            let ParsedValue::Enum(enum_type, Some(value)) = next_token else {
                return Err(err.into());
            };

            if enum_type != TRACK_RETRIG_LENGTH {
                return Err(err.into());
            }

            object
                .track_retrig_settings_mut(index)?
                .set_length(value.as_str().try_into()?);

            Ok(Response::Ok)
        }
        TRACK_RETRIG_VEL_OFFSET => {
            let ParsedValue::Parameter(param) = next_token else {
                return Err(SetError::InvalidFormat(format!(
                    "{TRACK_RETRIG_VEL_OFFSET} should have an integer parameter."
                ))
                .into());
            };

            object
                .track_retrig_settings_mut(index)?
                .set_velocity_curve(param.get_int())?;

            Ok(Response::Ok)
        }
        TRACK_RETRIG_ALWAYS_ON => {
            let ParsedValue::Parameter(param) = next_token else {
                return Err(SetError::InvalidFormat(format!(
                    "{TRACK_RETRIG_ALWAYS_ON} should have an integer parameter."
                ))
                .into());
            };

            object
                .track_retrig_settings_mut(index)?
                .set_always_on(param.get_bool_from_0_or_1(TRACK_RETRIG_ALWAYS_ON)?);

            Ok(Response::Ok)
        }

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

#[instrument(skip(object))]
#[log_errors]
fn get_enum(
    object: &Kit,
    variant: &str,
    value: &Option<String>,
) -> Result<RytmValue, RytmObjectError> {
    let result: &str = match variant {
        CONTROL_IN_1_MOD_TARGET => {
            let value = value.as_ref().ok_or_else(|| {
                GetError::InvalidFormat(
                    "ctrlinmod1target:<integer> is the correct format. Example: ctrlinmod1target:2"
                        .into(),
                )
            })?;
            let index = value.parse::<usize>().map_err(|_| {
                GetError::InvalidFormat(
                    "ctrlinmod1target:<integer> is the correct format. Example: ctrlinmod1target:2"
                        .into(),
                )
            })?;
            match index {
                0 => object.control_in_1_mod_target_1().into(),
                1 => object.control_in_1_mod_target_2().into(),
                2 => object.control_in_1_mod_target_3().into(),
                3 => object.control_in_1_mod_target_4().into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod1target."
                    )
                    .into())
                }
            }
        }
        CONTROL_IN_2_MOD_TARGET => {
            let value = value.as_ref().ok_or_else(|| {
                GetError::InvalidFormat(
                    "ctrlinmod2target:<integer> is the correct format. Example: ctrlinmod2target:2"
                        .into(),
                )
            })?;
            let index = value.parse::<usize>().map_err(|_| {
                GetError::InvalidFormat(
                    "ctrlinmod2target:<integer> is the correct format. Example: ctrlinmod2target:2"
                        .into(),
                )
            })?;
            match index {
                0 => object.control_in_2_mod_target_1().into(),
                1 => object.control_in_2_mod_target_2().into(),
                2 => object.control_in_2_mod_target_3().into(),
                3 => object.control_in_2_mod_target_4().into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod2target."
                    )
                    .into())
                }
            }
        }

        FX_LFO_DESTINATION => (*object.fx_lfo().destination()).into(),
        FX_COMP_ATTACK => (*object.fx_compressor().attack()).into(),
        FX_COMP_RELEASE => (*object.fx_compressor().release()).into(),
        FX_COMP_RATIO => (*object.fx_compressor().ratio()).into(),
        FX_COMP_SIDE_CHAIN_EQ => (*object.fx_compressor().side_chain_eq()).into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn get_action(
    object: &Kit,
    tokens: &mut std::slice::Iter<ParsedValue>,
    action: &str,
) -> Result<RytmValue, RytmObjectError> {
    let result: RytmValue = match action {
        VERSION => (object.structure_version() as isize).into(),
        INDEX => (object.index() as isize).into(),
        NAME => object.name().into(),

        CONTROL_IN_1_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = tokens.next() else {
                return Err("ctrlinmod1amt should be followed by an index.".into());
            };
            match *index as usize {
                0 => (object.control_in_1_mod_amt_1()).into(),
                1 => (object.control_in_1_mod_amt_2()).into(),
                2 => (object.control_in_1_mod_amt_3()).into(),
                3 => (object.control_in_1_mod_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod1amt."
                    )
                    .into())
                }
            }
        }
        CONTROL_IN_2_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = tokens.next() else {
                return Err("ctrlinmod2amt should be followed by an index.".into());
            };
            match *index as usize {
                0 => (object.control_in_2_mod_amt_1()).into(),
                1 => (object.control_in_2_mod_amt_2()).into(),
                2 => (object.control_in_2_mod_amt_3()).into(),
                3 => (object.control_in_2_mod_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod2amt."
                    )
                    .into())
                }
            }
        }

        FX_DELAY_TIME => (object.fx_delay().time() as isize).into(),
        FX_DELAY_PING_PONG => isize::from(object.fx_delay().ping_pong()).into(),
        FX_DELAY_STEREO_WIDTH => (object.fx_delay().stereo_width()).into(),
        FX_DELAY_FEEDBACK => (object.fx_delay().feedback() as isize).into(),
        FX_DELAY_HPF => (object.fx_delay().hpf() as isize).into(),
        FX_DELAY_LPF => (object.fx_delay().lpf() as isize).into(),
        FX_DELAY_REVERB_SEND => (object.fx_delay().reverb_send() as isize).into(),
        FX_DELAY_VOLUME => (object.fx_delay().volume() as isize).into(),

        FX_REVERB_PRE_DELAY => (object.fx_reverb().pre_delay() as isize).into(),
        FX_REVERB_DECAY => (object.fx_reverb().decay() as isize).into(),
        FX_REVERB_FREQ => (object.fx_reverb().freq() as isize).into(),
        FX_REVERB_GAIN => (object.fx_reverb().gain() as isize).into(),
        FX_REVERB_HPF => (object.fx_reverb().hpf() as isize).into(),
        FX_REVERB_LPF => (object.fx_reverb().lpf() as isize).into(),
        FX_REVERB_VOLUME => (object.fx_reverb().volume() as isize).into(),

        FX_COMP_THRESHOLD => (object.fx_compressor().threshold() as isize).into(),
        FX_COMP_GAIN => (object.fx_compressor().gain() as isize).into(),
        FX_COMP_MIX => (object.fx_compressor().mix() as isize).into(),
        FX_COMP_VOLUME => (object.fx_compressor().volume() as isize).into(),

        FX_LFO_SPEED => (object.fx_lfo().speed()).into(),
        FX_LFO_FADE => (object.fx_lfo().fade()).into(),
        FX_LFO_START_PHASE_OR_SLEW => (object.fx_lfo().start_phase_or_slew() as isize).into(),
        FX_LFO_DEPTH => f64::from(object.fx_lfo().depth()).into(),

        FX_DISTORTION_DELAY_OVERDRIVE => (object.fx_distortion().delay_overdrive() as isize).into(),
        FX_DISTORTION_DELAY_POST => isize::from(object.fx_distortion().delay_post()).into(),
        FX_DISTORTION_REVERB_POST => isize::from(object.fx_distortion().reverb_post()).into(),
        FX_DISTORTION_AMOUNT => (object.fx_distortion().amount() as isize).into(),
        FX_DISTORTION_SYMMETRY => object.fx_distortion().symmetry().into(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result)
}

#[instrument(skip(object))]
#[log_errors]
fn set_enum(
    object: &mut Kit,
    variant: &str,
    value: &Option<String>,
    next_param: Option<&ParsedValue>,
) -> Result<Response, RytmObjectError> {
    let enum_value = value
        .clone()
        .ok_or_else(|| GetError::InvalidFormat("Enum value not provided".into()))?;

    match variant {
        CONTROL_IN_1_MOD_TARGET => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = next_param else {
                return Err(
                    SetError::InvalidFormat("ctrlinmod1target should be followed by an integer ctrlinmod1 index. Format: ctrlinmod1target:<target> <ctrlinmod1 index>. Example: ctrlinmod1target:lfophase 2".into())
                    .into(),
                );
            };
            match *index as usize {
                0 => object.set_control_in_1_mod_target_1(enum_value.as_str().try_into()?),
                1 => object.set_control_in_1_mod_target_2(enum_value.as_str().try_into()?),
                2 => object.set_control_in_1_mod_target_3(enum_value.as_str().try_into()?),
                3 => object.set_control_in_1_mod_target_4(enum_value.as_str().try_into()?),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for {CONTROL_IN_1_MOD_TARGET}."
                    )
                    .into())
                }
            }
        }
        CONTROL_IN_2_MOD_TARGET => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = next_param else {
                return Err(
                    SetError::InvalidFormat("ctrlinmod2target should be followed by an integer ctrlinmod2 index. Format: ctrlinmod2target:<target> <ctrlinmod2 index>. Example: ctrlinmod2target:lfophase 2".into())
                    .into(),
                );
            };
            match *index as usize {
                0 => object.set_control_in_2_mod_target_1(enum_value.as_str().try_into()?),
                1 => object.set_control_in_2_mod_target_2(enum_value.as_str().try_into()?),
                2 => object.set_control_in_2_mod_target_3(enum_value.as_str().try_into()?),
                3 => object.set_control_in_2_mod_target_4(enum_value.as_str().try_into()?),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for {CONTROL_IN_2_MOD_TARGET}."
                    )
                    .into())
                }
            }
        }
        // Only set.
        FX_DELAY_TIME_ON_THE_GRID => {
            object
                .fx_delay_mut()
                .set_time_on_grid(enum_value.as_str().try_into()?);
        }
        FX_LFO_DESTINATION => object
            .fx_lfo_mut()
            .set_destination(enum_value.as_str().try_into()?),
        FX_COMP_ATTACK => object
            .fx_compressor_mut()
            .set_attack(enum_value.as_str().try_into()?),
        FX_COMP_RELEASE => object
            .fx_compressor_mut()
            .set_release(enum_value.as_str().try_into()?),
        FX_COMP_RATIO => object
            .fx_compressor_mut()
            .set_ratio(enum_value.as_str().try_into()?),
        FX_COMP_SIDE_CHAIN_EQ => object
            .fx_compressor_mut()
            .set_side_chain_eq(enum_value.as_str().try_into()?),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn set_action(
    object: &mut Kit,
    action: &str,
    tokens: &mut std::slice::Iter<ParsedValue>,
) -> Result<Response, RytmObjectError> {
    if action == NAME {
        if let Some(ParsedValue::ParameterString(name)) = tokens.next() {
            if name.is_empty() {
                return Err("Invalid parameter: name must not be empty.".into());
            }
            object.set_name(name)?;
            return Ok(Response::Ok);
        }
        return Err("Invalid parameter: name must be a symbol with maximum 15 characters long and use only ascii characters.".into());
    }

    let Some(ParsedValue::Parameter(param)) = tokens.next() else {
        return Err("Allowed parameters are integers or floats or a symbol if you'd like to change the name of the kit.".into());
    };

    match action {
        CONTROL_IN_1_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(amount))) = tokens.next() else {
                return Err(
                    SetError::InvalidFormat("ctrlinmod1amt should be followed by an integer ctrlinmod1 index. Format: ctrlinmod1amt <ctrlinmod1 index> <amount>. Example: ctrlinmod1amt 2 100".into())
                    .into(),
                );
            };
            match param.get_int() as usize {
                0 => object.set_control_in_1_mod_amt_1(*amount)?,
                1 => object.set_control_in_1_mod_amt_2(*amount)?,
                2 => object.set_control_in_1_mod_amt_3(*amount)?,
                3 => object.set_control_in_1_mod_amt_4(*amount)?,
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod1amt."
                    )
                    .into())
                }
            }
        }

        CONTROL_IN_2_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(amount))) = tokens.next() else {
                return Err(
                    SetError::InvalidFormat("ctrlinmod2amt should be followed by an integer ctrlinmod2 index. Format: ctrlinmod2amt <ctrlinmod2 index> <amount>. Example: ctrlinmod2amt 2 100".into())
                    .into(),
                );
            };
            match param.get_int() as usize {
                0 => object.set_control_in_2_mod_amt_1(*amount)?,
                1 => object.set_control_in_2_mod_amt_2(*amount)?,
                2 => object.set_control_in_2_mod_amt_3(*amount)?,
                3 => object.set_control_in_2_mod_amt_4(*amount)?,
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for ctrlinmod2amt."
                    )
                    .into())
                }
            }
        }

        FX_DELAY_TIME => {
            object.fx_delay_mut().set_time(param.get_int() as usize)?;
        }
        FX_DELAY_PING_PONG => {
            object
                .fx_delay_mut()
                .set_ping_pong(param.get_bool_from_0_or_1(FX_DELAY_PING_PONG)?);
        }
        FX_DELAY_STEREO_WIDTH => {
            object.fx_delay_mut().set_stereo_width(param.get_int())?;
        }
        FX_DELAY_FEEDBACK => {
            object
                .fx_delay_mut()
                .set_feedback(param.get_int() as usize)?;
        }
        FX_DELAY_HPF => {
            object.fx_delay_mut().set_hpf(param.get_int() as usize)?;
        }
        FX_DELAY_LPF => {
            object.fx_delay_mut().set_lpf(param.get_int() as usize)?;
        }
        FX_DELAY_REVERB_SEND => {
            object
                .fx_delay_mut()
                .set_reverb_send(param.get_int() as usize)?;
        }
        FX_DELAY_VOLUME => {
            object.fx_delay_mut().set_volume(param.get_int() as usize)?;
        }

        FX_REVERB_PRE_DELAY => {
            object
                .fx_reverb_mut()
                .set_pre_delay(param.get_int() as usize)?;
        }
        FX_REVERB_DECAY => {
            object.fx_reverb_mut().set_decay(param.get_int() as usize)?;
        }
        FX_REVERB_FREQ => {
            object.fx_reverb_mut().set_freq(param.get_int() as usize)?;
        }
        FX_REVERB_GAIN => {
            object.fx_reverb_mut().set_gain(param.get_int() as usize)?;
        }
        FX_REVERB_HPF => {
            object.fx_reverb_mut().set_hpf(param.get_int() as usize)?;
        }
        FX_REVERB_LPF => {
            object.fx_reverb_mut().set_lpf(param.get_int() as usize)?;
        }
        FX_REVERB_VOLUME => {
            object
                .fx_reverb_mut()
                .set_volume(param.get_int() as usize)?;
        }

        FX_COMP_THRESHOLD => {
            object
                .fx_compressor_mut()
                .set_threshold(param.get_int() as usize)?;
        }
        FX_COMP_GAIN => {
            object
                .fx_compressor_mut()
                .set_gain(param.get_int() as usize)?;
        }
        FX_COMP_MIX => {
            object
                .fx_compressor_mut()
                .set_mix(param.get_int() as usize)?;
        }
        FX_COMP_VOLUME => {
            object
                .fx_compressor_mut()
                .set_volume(param.get_int() as usize)?;
        }

        FX_LFO_SPEED => {
            object.fx_lfo_mut().set_speed(param.get_int())?;
        }
        FX_LFO_FADE => {
            object.fx_lfo_mut().set_fade(param.get_int())?;
        }
        FX_LFO_START_PHASE_OR_SLEW => {
            object
                .fx_lfo_mut()
                .set_start_phase(param.get_int() as usize)?;
        }
        FX_LFO_DEPTH => {
            object.fx_lfo_mut().set_depth(param.get_float() as f32)?;
        }

        FX_DISTORTION_DELAY_OVERDRIVE => {
            object
                .fx_distortion_mut()
                .set_delay_overdrive(param.get_int() as usize)?;
        }
        FX_DISTORTION_DELAY_POST => {
            object
                .fx_distortion_mut()
                .set_delay_post(param.get_bool_from_0_or_1(FX_DISTORTION_DELAY_POST)?);
        }
        FX_DISTORTION_REVERB_POST => {
            object
                .fx_distortion_mut()
                .set_reverb_post(param.get_bool_from_0_or_1(FX_DISTORTION_REVERB_POST)?);
        }
        FX_DISTORTION_AMOUNT => {
            object
                .fx_distortion_mut()
                .set_amount(param.get_int() as usize)?;
        }
        FX_DISTORTION_SYMMETRY => {
            object.fx_distortion_mut().set_symmetry(param.get_int())?;
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}
