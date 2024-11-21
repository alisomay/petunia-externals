use error_logger_macro::log_errors;
use rytm_rs::object::{Kit, Sound};
use tracing::instrument;

use crate::error::EnumError::InvalidEnumType;
use crate::error::{GetError, IdentifierError, RytmObjectError, SetError};
use crate::parse::types::{Number, ParsedValue};
use crate::types::CommandType;
use crate::value::RytmValue;
use crate::RytmObject;
use tracing::error;

use super::Response;

#[derive(Debug)]
pub enum SoundSource<'a> {
    Pool,
    WorkBuffer,
    Kit(&'a Kit),
    KitMut(&'a mut Kit),
}

impl std::fmt::Display for SoundSource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundSource::Pool => write!(f, "pool"),
            SoundSource::WorkBuffer => write!(f, "workbuffer"),
            SoundSource::Kit(_) => write!(f, "kit"),
            SoundSource::KitMut(_) => write!(f, "kit"),
        }
    }
}

#[instrument(skip(rytm, source), fields(source = %source, tokens = ?tokens, index = %index, command_type = ?command_type))]
pub fn handle(
    rytm: &RytmObject,
    tokens: Vec<ParsedValue>,
    index: usize,
    source: SoundSource,
    command_type: CommandType,
) -> Result<Response, RytmObjectError> {
    let mut tokens = match source {
        SoundSource::Kit(_) | SoundSource::KitMut(_) => tokens.iter(),
        _ => tokens[1..].iter(),
    };

    let next_token = tokens.next();

    let mut guard = None;

    match command_type {
        CommandType::Get => {
            let object = match source {
                SoundSource::Pool => {
                    let g = rytm.project.lock();
                    guard.replace(g);
                    &guard.as_ref().unwrap().pool_sounds()[index]
                }
                SoundSource::WorkBuffer => {
                    let g = rytm.project.lock();
                    guard.replace(g);
                    &guard.as_ref().unwrap().work_buffer().sounds()[index]
                }
                SoundSource::Kit(kit) => &kit.sounds()[index],
                _ => panic!("Do not use SoundSource::KitMut for get."),
            };

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
                _ => {
                    unreachable!("Parser should take care of this. Invalid getter format.")
                }
            }
        }
        CommandType::Set => {
            let object = match source {
                SoundSource::Pool => {
                    let g = rytm.project.lock();
                    guard.replace(g);
                    &mut guard.as_mut().unwrap().pool_sounds_mut()[index]
                }
                SoundSource::WorkBuffer => {
                    let g = rytm.project.lock();
                    guard.replace(g);
                    &mut guard.as_mut().unwrap().work_buffer_mut().sounds_mut()[index]
                }
                SoundSource::KitMut(kit) => &mut kit.sounds_mut()[index],
                _ => panic!("Do not use SoundSource::Kit for set."),
            };

            match next_token {
                Some(ParsedValue::Enum(variant, value)) => {
                    set_enum(object, variant, value, tokens.next())
                }
                Some(ParsedValue::Identifier(action)) => set_action(object, action, &mut tokens),
                _ => {
                    unreachable!("Parser should take care of this. Invalid setter format.")
                }
            }
        }
    }
}

#[instrument(skip(object))]
#[log_errors]
fn get_enum(
    object: &Sound,
    variant: &str,
    value: &Option<String>,
) -> Result<RytmValue, RytmObjectError> {
    use crate::api::sound_enum_type::*;
    let result: &str = match variant {
        MACHINE_TYPE => object.machine_type().into(),
        LFO_DESTINATION => object.lfo().destination().into(),

        VELOCITY_MOD_TARGET => {
            let value = value.as_ref().ok_or_else(|| {
                GetError::InvalidFormat(
                    "velmodtarget:<integer> is the correct format. Example: velmodtarget:2".into(),
                )
            })?;
            let index = value.parse::<usize>().map_err(|_| {
                GetError::InvalidFormat(
                    "velmodtarget:<integer> is the correct format. Example: velmodtarget:2".into(),
                )
            })?;
            match index {
                0 => object.settings().velocity_modulation_target_1().into(),
                1 => object.settings().velocity_modulation_target_2().into(),
                2 => object.settings().velocity_modulation_target_3().into(),
                3 => object.settings().velocity_modulation_target_4().into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for velmodtarget."
                    )
                    .into())
                }
            }
        }
        AFTER_TOUCH_MOD_TARGET => {
            let value = value.as_ref().ok_or_else(|| {
                GetError::InvalidFormat(
                    "atmodtarget:<integer> is the correct format. Example: atmodtarget:2".into(),
                )
            })?;
            let index = value.parse::<usize>().map_err(|_| {
                GetError::InvalidFormat(
                    "atmodtarget:<integer> is the correct format. Example: atmodtarget:2".into(),
                )
            })?;
            match index {
                0 => object.settings().after_touch_modulation_target_1().into(),
                1 => object.settings().after_touch_modulation_target_2().into(),
                2 => object.settings().after_touch_modulation_target_3().into(),
                3 => object.settings().after_touch_modulation_target_4().into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for atmodtarget."
                    )
                    .into())
                }
            }
        }
        FILTER_TYPE => object.filter().filter_type().into(),
        LFO_MULTIPLIER => object.lfo().multiplier().into(),
        LFO_WAVEFORM => object.lfo().waveform().into(),
        LFO_MODE => object.lfo().mode().into(),
        SOUND_SETTINGS_CHROMATIC_MODE => object.settings().chromatic_mode().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn get_action(
    object: &Sound,
    tokens: &mut std::slice::Iter<ParsedValue>,
    action: &str,
) -> Result<RytmValue, RytmObjectError> {
    use crate::api::sound_action_type::*;
    let result: RytmValue = match action {
        NAME => return Ok(object.name().into()),
        ACCENT_LEVEL => (object.accent_level() as isize).into(),
        // TODO: MACHINE
        // MACHINE => (object.machine() as isize).into(),
        AMP_ATTACK => (object.amplitude().attack() as isize).into(),
        AMP_HOLD => (object.amplitude().hold() as isize).into(),
        AMP_DECAY => (object.amplitude().decay() as isize).into(),
        AMP_OVERDRIVE => (object.amplitude().overdrive() as isize).into(),
        AMP_DELAY_SEND => (object.amplitude().delay_send() as isize).into(),
        AMP_REVERB_SEND => (object.amplitude().reverb_send() as isize).into(),
        AMP_PAN => (object.amplitude().pan() as isize).into(),
        AMP_VOLUME => (object.amplitude().volume() as isize).into(),
        FILT_ATTACK => (object.filter().attack() as isize).into(),
        FILT_HOLD => (object.filter().sustain() as isize).into(),
        FILT_DECAY => (object.filter().decay() as isize).into(),
        FILT_RELEASE => (object.filter().release() as isize).into(),
        FILT_CUTOFF => (object.filter().cutoff() as isize).into(),
        FILT_RESONANCE => (object.filter().resonance() as isize).into(),
        FILT_ENVELOPE_AMOUNT => (object.filter().envelope_amount()).into(),
        LFO_SPEED => (object.lfo().speed()).into(),
        LFO_FADE => (object.lfo().fade()).into(),
        LFO_START_PHASE_OR_SLEW => (object.lfo().start_phase_or_slew() as isize).into(),
        LFO_DEPTH => f64::from(object.lfo().depth()).into(),
        SAMP_TUNE => (object.sample().tune()).into(),
        SAMP_FINE_TUNE => (object.sample().fine_tune()).into(),
        SAMP_NUMBER => (object.sample().slice_number() as isize).into(),
        SAMP_BIT_REDUCTION => (object.sample().bit_reduction() as isize).into(),
        SAMP_START => f64::from(object.sample().start()).into(),
        SAMP_END => f64::from(object.sample().end()).into(),
        SAMP_LOOP_FLAG => isize::from(object.sample().loop_flag()).into(),
        SAMP_VOLUME => (object.sample().volume() as isize).into(),

        VEL_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = tokens.next() else {
                return Err("velmodamt should be followed by an index.".into());
            };
            match *index as usize {
                0 => (object.settings().velocity_modulation_amt_1()).into(),
                1 => (object.settings().velocity_modulation_amt_2()).into(),
                2 => (object.settings().velocity_modulation_amt_3()).into(),
                3 => (object.settings().velocity_modulation_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for velmodamt."
                    )
                    .into())
                }
            }
        }

        AT_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = tokens.next() else {
                return Err("atmodamt should be followed by an integer index.".into());
            };
            match *index as usize {
                0 => (object.settings().after_touch_modulation_amt_1()).into(),
                1 => (object.settings().after_touch_modulation_amt_2()).into(),
                2 => (object.settings().after_touch_modulation_amt_3()).into(),
                3 => (object.settings().after_touch_modulation_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for atmodamt."
                    )
                    .into())
                }
            }
        }

        ENV_RESET_FILTER => isize::from(object.settings().env_reset_filter()).into(),
        VELOCITY_TO_VOLUME => isize::from(object.settings().velocity_to_volume()).into(),
        LEGACY_FX_SEND => isize::from(object.settings().legacy_fx_send()).into(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result)
}

#[instrument(skip(object))]
#[log_errors]
fn set_enum(
    object: &mut Sound,
    variant: &str,
    value: &Option<String>,
    next_param: Option<&ParsedValue>,
) -> Result<Response, RytmObjectError> {
    let enum_value = value
        .clone()
        .ok_or_else(|| GetError::InvalidFormat("Enum value not provided".into()))?;

    use crate::api::sound_enum_type::*;
    match variant {
        // TODO:
        // MACHINE_PARAMETERS => {
        //     object.set_selected_machine_parameter(enum_value.try_into()?);
        // }
        MACHINE_TYPE => {
            object.set_machine_type(enum_value.as_str().try_into()?)?;
        }
        LFO_DESTINATION => {
            object
                .lfo_mut()
                .set_destination(enum_value.as_str().try_into()?);
        }
        VELOCITY_MOD_TARGET => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = next_param else {
                return Err(
                        SetError::InvalidFormat( "velmodtarget should be followed by an integer velmod index. Format: velmodtarget:<target> <velmod index>. Example: velmodtarget:lfophase 2".into())
                        .into(),
                );
            };
            match *index as usize {
                0 => object
                    .settings_mut()
                    .set_velocity_modulation_target_1(enum_value.as_str().try_into()?),
                1 => object
                    .settings_mut()
                    .set_velocity_modulation_target_2(enum_value.as_str().try_into()?),
                2 => object
                    .settings_mut()
                    .set_velocity_modulation_target_3(enum_value.as_str().try_into()?),
                3 => object
                    .settings_mut()
                    .set_velocity_modulation_target_4(enum_value.as_str().try_into()?),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for velmodtarget."
                    )
                    .into())
                }
            }
        }
        AFTER_TOUCH_MOD_TARGET => {
            let Some(ParsedValue::Parameter(Number::Int(index))) = next_param else {
                return Err(
                    SetError::InvalidFormat("atmodtarget should be followed by an integer atmod index. Format: atmodtarget:<target> <atmod index>. Example: atmodtarget:lfophase 2".into())
                        .into(),
                );
            };
            match *index as usize {
                0 => object
                    .settings_mut()
                    .set_after_touch_modulation_target_1(enum_value.as_str().try_into()?),
                1 => object
                    .settings_mut()
                    .set_after_touch_modulation_target_2(enum_value.as_str().try_into()?),
                2 => object
                    .settings_mut()
                    .set_after_touch_modulation_target_3(enum_value.as_str().try_into()?),
                3 => object
                    .settings_mut()
                    .set_after_touch_modulation_target_4(enum_value.as_str().try_into()?),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for atmodtarget."
                    )
                    .into())
                }
            }
        }
        FILTER_TYPE => {
            object
                .filter_mut()
                .set_filter_type(enum_value.as_str().try_into()?);
        }
        LFO_MULTIPLIER => {
            object
                .lfo_mut()
                .set_multiplier(enum_value.as_str().try_into()?);
        }
        LFO_WAVEFORM => {
            object
                .lfo_mut()
                .set_waveform(enum_value.as_str().try_into()?);
        }
        LFO_MODE => {
            object.lfo_mut().set_mode(enum_value.as_str().try_into()?);
        }
        SOUND_SETTINGS_CHROMATIC_MODE => {
            object
                .settings_mut()
                .set_chromatic_mode(enum_value.as_str().try_into()?);
        }
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn set_action(
    object: &mut Sound,
    action: &str,
    tokens: &mut std::slice::Iter<ParsedValue>,
) -> Result<Response, RytmObjectError> {
    use crate::api::sound_action_type::*;

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
        return Err("Allowed parameters are integers or floats or a symbol if you'd like to change the name of the sound.".into());
    };

    match action {
        ACCENT_LEVEL => {
            object.set_accent_level(param.get_int() as usize)?;
        }
        // TODO:
        // MACHINE => {
        //     object.set_machine(param.get_int() as usize)?;
        // }
        AMP_ATTACK => {
            object
                .amplitude_mut()
                .set_attack(param.get_int() as usize)?;
        }
        AMP_HOLD => {
            object.amplitude_mut().set_hold(param.get_int() as usize)?;
        }
        AMP_DECAY => {
            object.amplitude_mut().set_decay(param.get_int() as usize)?;
        }
        AMP_OVERDRIVE => {
            object
                .amplitude_mut()
                .set_overdrive(param.get_int() as usize)?;
        }
        AMP_DELAY_SEND => {
            object
                .amplitude_mut()
                .set_delay_send(param.get_int() as usize)?;
        }
        AMP_REVERB_SEND => {
            object
                .amplitude_mut()
                .set_reverb_send(param.get_int() as usize)?;
        }
        AMP_PAN => {
            object.amplitude_mut().set_pan(param.get_int())?;
        }
        AMP_VOLUME => {
            object
                .amplitude_mut()
                .set_volume(param.get_int() as usize)?;
        }
        FILT_ATTACK => {
            object.filter_mut().set_attack(param.get_int() as usize)?;
        }
        FILT_HOLD => {
            object.filter_mut().set_sustain(param.get_int() as usize)?;
        }
        FILT_DECAY => {
            object.filter_mut().set_decay(param.get_int() as usize)?;
        }
        FILT_RELEASE => {
            object.filter_mut().set_release(param.get_int() as usize)?;
        }
        FILT_CUTOFF => {
            object.filter_mut().set_cutoff(param.get_int() as usize)?;
        }
        FILT_RESONANCE => {
            object
                .filter_mut()
                .set_resonance(param.get_int() as usize)?;
        }
        FILT_ENVELOPE_AMOUNT => {
            object.filter_mut().set_envelope_amount(param.get_int())?;
        }
        LFO_SPEED => {
            object.lfo_mut().set_speed(param.get_int())?;
        }
        LFO_FADE => {
            object.lfo_mut().set_fade(param.get_int())?;
        }
        LFO_START_PHASE_OR_SLEW => {
            object.lfo_mut().set_start_phase(param.get_int() as usize)?;
        }
        LFO_DEPTH => {
            object.lfo_mut().set_depth(param.get_float() as f32)?;
        }
        SAMP_TUNE => {
            object.sample_mut().set_tune(param.get_int())?;
        }
        SAMP_FINE_TUNE => {
            object.sample_mut().set_fine_tune(param.get_int())?;
        }
        SAMP_NUMBER => {
            object
                .sample_mut()
                .set_slice_number(param.get_int() as usize)?;
        }
        SAMP_BIT_REDUCTION => {
            object
                .sample_mut()
                .set_bit_reduction(param.get_int() as usize)?;
        }
        SAMP_START => {
            object.sample_mut().set_start(param.get_float() as f32)?;
        }
        SAMP_END => {
            object.sample_mut().set_end(param.get_float() as f32)?;
        }
        SAMP_LOOP_FLAG => {
            object
                .sample_mut()
                .set_loop_flag(param.get_bool_from_0_or_1(SAMP_LOOP_FLAG)?);
        }
        SAMP_VOLUME => {
            object.sample_mut().set_volume(param.get_int() as usize)?;
        }

        VEL_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(amount))) = tokens.next() else {
                return Err(
                         SetError::InvalidFormat("velmodamt should be followed by an integer velmod index. Format: velmodamt <velmod index> <amount>. Example: velmodamt 2 100".into())
                        .into(),
                );
            };
            match param.get_int() as usize {
                0 => object
                    .settings_mut()
                    .set_velocity_modulation_amt_1(*amount)?,
                1 => object
                    .settings_mut()
                    .set_velocity_modulation_amt_2(*amount)?,
                2 => object
                    .settings_mut()
                    .set_velocity_modulation_amt_3(*amount)?,
                3 => object
                    .settings_mut()
                    .set_velocity_modulation_amt_4(*amount)?,
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for velmodamt."
                    )
                    .into())
                }
            }
        }

        AT_MOD_AMT => {
            let Some(ParsedValue::Parameter(Number::Int(amount))) = tokens.next() else {
                return Err(
                        SetError::InvalidFormat( "atmodamt should be followed by an integer atmod index. Format: atmodamt <atmod index> <amount>. Example: atmodamt 2 100".into())
                        .into(),
                );
            };
            match param.get_int() as usize {
                0 => object
                    .settings_mut()
                    .set_after_touch_modulation_amt_1(*amount)?,
                1 => object
                    .settings_mut()
                    .set_after_touch_modulation_amt_2(*amount)?,
                2 => object
                    .settings_mut()
                    .set_after_touch_modulation_amt_3(*amount)?,
                3 => object
                    .settings_mut()
                    .set_after_touch_modulation_amt_4(*amount)?,
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for atmodamt."
                    )
                    .into())
                }
            }
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}
