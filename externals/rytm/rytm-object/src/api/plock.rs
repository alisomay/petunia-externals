use crate::api::kit_action_type;
use crate::api::kit_enum_type;
use crate::api::sound_action_type;
use crate::api::sound_enum_type;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmObjectError;
use crate::parse::types::{ParsedValue, PlockOperation};
use crate::types::CommandType;
use crate::value::RytmValue;
use error_logger_macro::log_errors;
use rytm_rs::object::pattern::Trig;
use tracing::error;
use tracing::instrument;

use super::Response;

#[instrument(skip(object))]
#[log_errors]
pub fn handle_plock_commands(
    object: &mut Trig,
    tokens: &mut std::slice::Iter<ParsedValue>,
    trig_index: usize,
    op: PlockOperation,
    command_type: CommandType,
) -> Result<Response, RytmObjectError> {
    // Validation
    match op {
        PlockOperation::Clear => {
            if command_type == CommandType::Get {
                return Err(
               format!("Invalid format: A get command can not be followed by a {op} command. Please try {} or use a set command.", PlockOperation::Get).into()
            );
            }
        }
        PlockOperation::Get => {
            if command_type == CommandType::Set {
                return Err(
                format!("Invalid format: A set command can not be followed by a {op} command. Please try {} or {} or use a get command.", PlockOperation::Set, PlockOperation::Clear).into()
            );
            }
        }
        PlockOperation::Set => {
            if command_type == CommandType::Get {
                return Err(
                format!("Invalid format: A get command can not be followed by a {op} command. Please try {} or use a set command.", PlockOperation::Get).into()
            );
            }
        }
    }

    match op {
        PlockOperation::Get => {
            let (key, maybe_value): (RytmValue, Option<RytmValue>) = match tokens.next() {
                Some(ParsedValue::Identifier(ident)) => (
                    ident.into(),
                    match ident.as_str() {
                        kit_action_type::FX_DELAY_TIME => object
                            .plock_get_fx_delay_time()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_DELAY_PING_PONG => object
                            .plock_get_fx_delay_ping_pong()?
                            .map(|val| RytmValue::from(isize::from(val))),
                        kit_action_type::FX_DELAY_STEREO_WIDTH => object
                            .plock_get_fx_delay_stereo_width()?
                            .map(RytmValue::from),
                        kit_action_type::FX_DELAY_FEEDBACK => object
                            .plock_get_fx_delay_feedback()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_DELAY_HPF => object
                            .plock_get_fx_delay_hpf()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_DELAY_LPF => object
                            .plock_get_fx_delay_lpf()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_DELAY_REVERB_SEND => object
                            .plock_get_fx_delay_reverb_send()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_DELAY_VOLUME => object
                            .plock_get_fx_delay_volume()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_PRE_DELAY => object
                            .plock_get_fx_reverb_pre_delay()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_DECAY => object
                            .plock_get_fx_reverb_decay()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_FREQ => object
                            .plock_get_fx_reverb_freq()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_GAIN => object
                            .plock_get_fx_reverb_gain()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_HPF => object
                            .plock_get_fx_reverb_hpf()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_LPF => object
                            .plock_get_fx_reverb_lpf()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_REVERB_VOLUME => object
                            .plock_get_fx_reverb_volume()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_COMP_THRESHOLD => object
                            .plock_get_fx_compressor_threshold()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_COMP_GAIN => object
                            .plock_get_fx_compressor_gain()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_COMP_MIX => object
                            .plock_get_fx_compressor_mix()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_COMP_VOLUME => object
                            .plock_get_fx_compressor_volume()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_LFO_SPEED => {
                            object.plock_get_fx_lfo_speed()?.map(RytmValue::from)
                        }
                        kit_action_type::FX_LFO_FADE => {
                            object.plock_get_fx_lfo_fade()?.map(RytmValue::from)
                        }
                        kit_action_type::FX_LFO_START_PHASE_OR_SLEW => object
                            .plock_get_fx_lfo_start_phase()?
                            .map(|val| RytmValue::from(val as isize)),
                        kit_action_type::FX_LFO_DEPTH => object
                            .plock_get_fx_lfo_depth()?
                            .map(|val| RytmValue::from(f64::from(val))),
                        //
                        // TODO: Do the dist setters after fixing the dist in the SDK
                        // TODO: MACHINE Do Machine plocks
                        //
                        sound_action_type::AMP_ATTACK => object
                            .plock_get_amplitude_attack()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_HOLD => object
                            .plock_get_amplitude_hold()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_DECAY => object
                            .plock_get_amplitude_decay()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_OVERDRIVE => object
                            .plock_get_amplitude_overdrive()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_DELAY_SEND => object
                            .plock_get_amplitude_delay_send()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_REVERB_SEND => object
                            .plock_get_amplitude_reverb_send()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::AMP_PAN => {
                            object.plock_get_amplitude_pan()?.map(RytmValue::from)
                        }
                        sound_action_type::AMP_VOLUME => object
                            .plock_get_amplitude_volume()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_ATTACK => object
                            .plock_get_filter_attack()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_HOLD => object
                            .plock_get_filter_sustain()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_DECAY => object
                            .plock_get_filter_decay()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_RELEASE => object
                            .plock_get_filter_release()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_CUTOFF => object
                            .plock_get_filter_cutoff()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_RESONANCE => object
                            .plock_get_filter_resonance()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::FILT_ENVELOPE_AMOUNT => object
                            .plock_get_filter_envelope_amount()?
                            .map(RytmValue::from),
                        sound_action_type::LFO_SPEED => {
                            object.plock_get_lfo_speed()?.map(RytmValue::from)
                        }
                        sound_action_type::LFO_FADE => {
                            object.plock_get_lfo_fade()?.map(RytmValue::from)
                        }
                        sound_action_type::LFO_START_PHASE_OR_SLEW => object
                            .plock_get_lfo_start_phase()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::LFO_DEPTH => object
                            .plock_get_lfo_depth()?
                            .map(|val| RytmValue::from(f64::from(val))),
                        sound_action_type::SAMP_TUNE => {
                            object.plock_get_sample_tune()?.map(RytmValue::from)
                        }
                        sound_action_type::SAMP_FINE_TUNE => {
                            object.plock_get_sample_fine_tune()?.map(RytmValue::from)
                        }
                        sound_action_type::SAMP_NUMBER => object
                            .plock_get_sample_number()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::SAMP_BIT_REDUCTION => object
                            .plock_get_sample_bit_reduction()?
                            .map(|val| RytmValue::from(val as isize)),
                        sound_action_type::SAMP_START => object
                            .plock_get_sample_start()?
                            .map(|val| RytmValue::from(f64::from(val))),
                        sound_action_type::SAMP_END => object
                            .plock_get_sample_end()?
                            .map(|val| RytmValue::from(f64::from(val))),
                        sound_action_type::SAMP_LOOP_FLAG => object
                            .plock_get_sample_loop_flag()?
                            .map(|val| RytmValue::from(isize::from(val))),
                        sound_action_type::SAMP_VOLUME => object
                            .plock_get_sample_volume()?
                            .map(|val| RytmValue::from(val as isize)),

                        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
                    },
                ),
                Some(ParsedValue::Enum(enum_type, _)) => (
                    enum_type.into(),
                    match enum_type.as_str() {
                        kit_enum_type::FX_COMP_ATTACK => object
                            .plock_get_fx_compressor_attack()?
                            .map(|v| <&str>::from(v).into()),
                        kit_enum_type::FX_COMP_RELEASE => object
                            .plock_get_fx_compressor_release()?
                            .map(|v| <&str>::from(v).into()),
                        kit_enum_type::FX_COMP_RATIO => object
                            .plock_get_fx_compressor_ratio()?
                            .map(|v| <&str>::from(v).into()),
                        kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => object
                            .plock_get_fx_compressor_side_chain_eq()?
                            .map(|v| <&str>::from(v).into()),
                        kit_enum_type::FX_LFO_DESTINATION => object
                            .plock_get_fx_lfo_destination()?
                            .map(|v| <&str>::from(v).into()),
                        // TODO:
                        // sound_enum_type::MACHINE_PARAMETERS => todo!(),
                        sound_enum_type::LFO_DESTINATION => object
                            .plock_get_lfo_destination()?
                            .map(|v| <&str>::from(v).into()),
                        sound_enum_type::FILTER_TYPE => object
                            .plock_get_filter_type()?
                            .map(|v| <&str>::from(v).into()),
                        sound_enum_type::LFO_MULTIPLIER => object
                            .plock_get_lfo_multiplier()?
                            .map(|v| <&str>::from(v).into()),
                        sound_enum_type::LFO_WAVEFORM => object
                            .plock_get_lfo_waveform()?
                            .map(|v| <&str>::from(v).into()),
                        sound_enum_type::LFO_MODE => {
                            object.plock_get_lfo_mode()?.map(|v| <&str>::from(v).into())
                        }
                        other => return Err(InvalidEnumType(other.to_owned()).into()),
                    },
                ),
                _ => {
                    unreachable!("Parser should take care of this. Invalid plock clear format.")
                }
            };

            if let Some(value) = maybe_value {
                Ok(Response::Common {
                    index: trig_index,
                    key,
                    value,
                })
            } else {
                // Send the value as "unset" for a plock which is not set.
                Ok(Response::Common {
                    index: trig_index,
                    key,
                    value: RytmValue::from("unset"),
                })
            }
        }

        PlockOperation::Set => {
            match tokens.next() {
                Some(ParsedValue::Identifier(ident)) => {
                    let Some(ParsedValue::Parameter(param)) = tokens.next() else {
                        return Err(
                            "Invalid format: A parameter or enum should follow a plockset action."
                                .into(),
                        );
                    };

                    match ident.as_str() {
                        kit_action_type::FX_DELAY_TIME => {
                            Ok(object.plock_set_fx_delay_time(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_DELAY_PING_PONG => Ok(object
                            .plock_set_fx_delay_ping_pong(
                                param.get_bool_from_0_or_1(kit_action_type::FX_DELAY_PING_PONG)?,
                            )?),
                        kit_action_type::FX_DELAY_STEREO_WIDTH => {
                            Ok(object.plock_set_fx_delay_stereo_width(param.get_int())?)
                        }
                        kit_action_type::FX_DELAY_FEEDBACK => {
                            Ok(object.plock_set_fx_delay_feedback(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_DELAY_HPF => {
                            Ok(object.plock_set_fx_delay_hpf(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_DELAY_LPF => {
                            Ok(object.plock_set_fx_delay_lpf(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_DELAY_REVERB_SEND => {
                            Ok(object.plock_set_fx_delay_reverb_send(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_DELAY_VOLUME => {
                            Ok(object.plock_set_fx_delay_volume(param.get_int() as usize)?)
                        }

                        kit_action_type::FX_REVERB_PRE_DELAY => {
                            Ok(object.plock_set_fx_reverb_pre_delay(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_DECAY => {
                            Ok(object.plock_set_fx_reverb_decay(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_FREQ => {
                            Ok(object.plock_set_fx_reverb_freq(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_GAIN => {
                            Ok(object.plock_set_fx_reverb_gain(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_HPF => {
                            Ok(object.plock_set_fx_reverb_hpf(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_LPF => {
                            Ok(object.plock_set_fx_reverb_lpf(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_REVERB_VOLUME => {
                            Ok(object.plock_set_fx_reverb_volume(param.get_int() as usize)?)
                        }

                        kit_action_type::FX_COMP_THRESHOLD => Ok(
                            object.plock_set_fx_compressor_threshold(param.get_int() as usize)?
                        ),
                        kit_action_type::FX_COMP_GAIN => {
                            Ok(object.plock_set_fx_compressor_gain(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_COMP_MIX => {
                            Ok(object.plock_set_fx_compressor_mix(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_COMP_VOLUME => {
                            Ok(object.plock_set_fx_compressor_volume(param.get_int() as usize)?)
                        }

                        kit_action_type::FX_LFO_SPEED => {
                            Ok(object.plock_set_fx_lfo_speed(param.get_int())?)
                        }
                        kit_action_type::FX_LFO_FADE => {
                            Ok(object.plock_set_fx_lfo_fade(param.get_int())?)
                        }
                        kit_action_type::FX_LFO_START_PHASE_OR_SLEW => {
                            Ok(object.plock_set_fx_lfo_start_phase(param.get_int() as usize)?)
                        }
                        kit_action_type::FX_LFO_DEPTH => {
                            Ok(object.plock_set_fx_lfo_depth(param.get_float() as f32)?)
                        }
                        //
                        // TODO: Do the dist setters after fixing the dist in the SDK
                        // TODO: Do Machine plocks
                        //
                        sound_action_type::AMP_ATTACK => {
                            Ok(object.plock_set_amplitude_attack(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_HOLD => {
                            Ok(object.plock_set_amplitude_hold(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_DECAY => {
                            Ok(object.plock_set_amplitude_decay(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_OVERDRIVE => {
                            Ok(object.plock_set_amplitude_overdrive(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_DELAY_SEND => {
                            Ok(object.plock_set_amplitude_delay_send(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_REVERB_SEND => {
                            Ok(object.plock_set_amplitude_reverb_send(param.get_int() as usize)?)
                        }
                        sound_action_type::AMP_PAN => {
                            Ok(object.plock_set_amplitude_pan(param.get_int())?)
                        }
                        sound_action_type::AMP_VOLUME => {
                            Ok(object.plock_set_amplitude_volume(param.get_int() as usize)?)
                        }

                        sound_action_type::FILT_ATTACK => {
                            Ok(object.plock_set_filter_attack(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_HOLD => {
                            Ok(object.plock_set_filter_sustain(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_DECAY => {
                            Ok(object.plock_set_filter_decay(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_RELEASE => {
                            Ok(object.plock_set_filter_release(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_CUTOFF => {
                            Ok(object.plock_set_filter_cutoff(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_RESONANCE => {
                            Ok(object.plock_set_filter_resonance(param.get_int() as usize)?)
                        }
                        sound_action_type::FILT_ENVELOPE_AMOUNT => {
                            Ok(object.plock_set_filter_envelope_amount(param.get_int())?)
                        }

                        sound_action_type::LFO_SPEED => {
                            Ok(object.plock_set_lfo_speed(param.get_int())?)
                        }
                        sound_action_type::LFO_FADE => {
                            Ok(object.plock_set_lfo_fade(param.get_int())?)
                        }
                        sound_action_type::LFO_START_PHASE_OR_SLEW => {
                            Ok(object.plock_set_lfo_start_phase(param.get_int() as usize)?)
                        }
                        sound_action_type::LFO_DEPTH => {
                            Ok(object.plock_set_lfo_depth(param.get_float() as f32)?)
                        }

                        sound_action_type::SAMP_TUNE => {
                            Ok(object.plock_set_sample_tune(param.get_int())?)
                        }
                        sound_action_type::SAMP_FINE_TUNE => {
                            Ok(object.plock_set_sample_fine_tune(param.get_int())?)
                        }
                        sound_action_type::SAMP_NUMBER => {
                            Ok(object.plock_set_sample_number(param.get_int() as usize)?)
                        }
                        sound_action_type::SAMP_BIT_REDUCTION => {
                            Ok(object.plock_set_sample_bit_reduction(param.get_int() as usize)?)
                        }
                        sound_action_type::SAMP_START => {
                            Ok(object.plock_set_sample_start(param.get_float() as f32)?)
                        }
                        sound_action_type::SAMP_END => {
                            Ok(object.plock_set_sample_end(param.get_float() as f32)?)
                        }
                        sound_action_type::SAMP_LOOP_FLAG => Ok(object
                            .plock_set_sample_loop_flag(
                                param.get_bool_from_0_or_1(sound_action_type::SAMP_LOOP_FLAG)?,
                            )?),
                        sound_action_type::SAMP_VOLUME => {
                            Ok(object.plock_set_sample_volume(param.get_int() as usize)?)
                        }

                        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
                    }
                    .map(|_| Response::Ok)
                }
                Some(ParsedValue::Enum(enum_type, Some(value))) => {
                    match enum_type.as_str() {
                        kit_enum_type::FX_COMP_ATTACK => Ok(
                            object.plock_set_fx_compressor_attack(value.as_str().try_into()?)?
                        ),
                        kit_enum_type::FX_COMP_RELEASE => {
                            Ok(object
                                .plock_set_fx_compressor_release(value.as_str().try_into()?)?)
                        }
                        kit_enum_type::FX_COMP_RATIO => {
                            Ok(object.plock_set_fx_compressor_ratio(value.as_str().try_into()?)?)
                        }
                        kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => Ok(object
                            .plock_set_fx_compressor_side_chain_eq(value.as_str().try_into()?)?),
                        kit_enum_type::FX_LFO_DESTINATION => {
                            Ok(object.plock_set_fx_lfo_destination(value.as_str().try_into()?)?)
                        }
                        // TODO:
                        // sound_enum_type::MACHINE_PARAMETERS => todo!(),
                        sound_enum_type::LFO_DESTINATION => {
                            Ok(object.plock_set_lfo_destination(value.as_str().try_into()?)?)
                        }
                        sound_enum_type::FILTER_TYPE => {
                            Ok(object.plock_set_filter_type(value.as_str().try_into()?)?)
                        }
                        sound_enum_type::LFO_MULTIPLIER => {
                            Ok(object.plock_set_lfo_multiplier(value.as_str().try_into()?)?)
                        }
                        sound_enum_type::LFO_WAVEFORM => {
                            Ok(object.plock_set_lfo_waveform(value.as_str().try_into()?)?)
                        }
                        sound_enum_type::LFO_MODE => {
                            Ok(object.plock_set_lfo_mode(value.as_str().try_into()?)?)
                        }

                        other => Err(InvalidEnumType(other.to_owned()).into()),
                    }
                    .map(|_| Response::Ok)
                }
                _ => {
                    unreachable!("Parser should take care of this. Invalid plock clear format.")
                }
            }
        }

        PlockOperation::Clear => match tokens.next() {
            Some(ParsedValue::Identifier(ident)) => {
                match ident.as_str() {
                    kit_action_type::FX_DELAY_TIME => Ok(object.plock_clear_fx_delay_time()?),
                    kit_action_type::FX_DELAY_PING_PONG => {
                        Ok(object.plock_clear_fx_delay_ping_pong()?)
                    }
                    kit_action_type::FX_DELAY_STEREO_WIDTH => {
                        Ok(object.plock_clear_fx_delay_stereo_width()?)
                    }
                    kit_action_type::FX_DELAY_FEEDBACK => {
                        Ok(object.plock_clear_fx_delay_feedback()?)
                    }
                    kit_action_type::FX_DELAY_HPF => Ok(object.plock_clear_fx_delay_hpf()?),
                    kit_action_type::FX_DELAY_LPF => Ok(object.plock_clear_fx_delay_lpf()?),
                    kit_action_type::FX_DELAY_REVERB_SEND => {
                        Ok(object.plock_clear_fx_delay_reverb_send()?)
                    }
                    kit_action_type::FX_DELAY_VOLUME => Ok(object.plock_clear_fx_delay_volume()?),

                    kit_action_type::FX_REVERB_PRE_DELAY => {
                        Ok(object.plock_clear_fx_reverb_pre_delay()?)
                    }
                    kit_action_type::FX_REVERB_DECAY => Ok(object.plock_clear_fx_reverb_decay()?),
                    kit_action_type::FX_REVERB_FREQ => Ok(object.plock_clear_fx_reverb_freq()?),
                    kit_action_type::FX_REVERB_GAIN => Ok(object.plock_clear_fx_reverb_gain()?),
                    kit_action_type::FX_REVERB_HPF => Ok(object.plock_clear_fx_reverb_hpf()?),
                    kit_action_type::FX_REVERB_LPF => Ok(object.plock_clear_fx_reverb_lpf()?),
                    kit_action_type::FX_REVERB_VOLUME => Ok(object.plock_clear_fx_reverb_volume()?),

                    kit_action_type::FX_COMP_THRESHOLD => {
                        Ok(object.plock_clear_fx_compressor_threshold()?)
                    }
                    kit_action_type::FX_COMP_GAIN => Ok(object.plock_clear_fx_compressor_gain()?),
                    kit_action_type::FX_COMP_MIX => Ok(object.plock_clear_fx_compressor_mix()?),
                    kit_action_type::FX_COMP_VOLUME => {
                        Ok(object.plock_clear_fx_compressor_volume()?)
                    }

                    kit_action_type::FX_LFO_SPEED => Ok(object.plock_clear_fx_lfo_speed()?),
                    kit_action_type::FX_LFO_FADE => Ok(object.plock_clear_fx_lfo_fade()?),
                    kit_action_type::FX_LFO_START_PHASE_OR_SLEW => {
                        Ok(object.plock_clear_fx_lfo_start_phase()?)
                    }
                    kit_action_type::FX_LFO_DEPTH => Ok(object.plock_clear_fx_lfo_depth()?),
                    //
                    // TODO: Do the dist setters after fixing the dist in the SDK
                    // TODO: MACHINE Do Machine plocks
                    //
                    sound_action_type::AMP_ATTACK => Ok(object.plock_clear_amplitude_attack()?),
                    sound_action_type::AMP_HOLD => Ok(object.plock_clear_amplitude_hold()?),
                    sound_action_type::AMP_DECAY => Ok(object.plock_clear_amplitude_decay()?),
                    sound_action_type::AMP_OVERDRIVE => {
                        Ok(object.plock_clear_amplitude_overdrive()?)
                    }
                    sound_action_type::AMP_DELAY_SEND => {
                        Ok(object.plock_clear_amplitude_delay_send()?)
                    }
                    sound_action_type::AMP_REVERB_SEND => {
                        Ok(object.plock_clear_amplitude_reverb_send()?)
                    }
                    sound_action_type::AMP_PAN => Ok(object.plock_clear_amplitude_pan()?),
                    sound_action_type::AMP_VOLUME => Ok(object.plock_clear_amplitude_volume()?),

                    sound_action_type::FILT_ATTACK => Ok(object.plock_clear_filter_attack()?),
                    sound_action_type::FILT_HOLD => Ok(object.plock_clear_filter_sustain()?),
                    sound_action_type::FILT_DECAY => Ok(object.plock_clear_filter_decay()?),
                    sound_action_type::FILT_RELEASE => Ok(object.plock_clear_filter_release()?),
                    sound_action_type::FILT_CUTOFF => Ok(object.plock_clear_filter_cutoff()?),
                    sound_action_type::FILT_RESONANCE => Ok(object.plock_clear_filter_resonance()?),
                    sound_action_type::FILT_ENVELOPE_AMOUNT => {
                        Ok(object.plock_clear_filter_envelope_amount()?)
                    }

                    sound_action_type::LFO_SPEED => Ok(object.plock_clear_lfo_speed()?),
                    sound_action_type::LFO_FADE => Ok(object.plock_clear_lfo_fade()?),
                    sound_action_type::LFO_START_PHASE_OR_SLEW => {
                        Ok(object.plock_clear_lfo_start_phase()?)
                    }
                    sound_action_type::LFO_DEPTH => Ok(object.plock_clear_lfo_depth()?),

                    sound_action_type::SAMP_TUNE => Ok(object.plock_clear_sample_tune()?),
                    sound_action_type::SAMP_FINE_TUNE => Ok(object.plock_clear_sample_fine_tune()?),
                    sound_action_type::SAMP_NUMBER => Ok(object.plock_clear_sample_number()?),
                    sound_action_type::SAMP_BIT_REDUCTION => {
                        Ok(object.plock_clear_sample_bit_reduction()?)
                    }
                    sound_action_type::SAMP_START => Ok(object.plock_clear_sample_start()?),
                    sound_action_type::SAMP_END => Ok(object.plock_clear_sample_end()?),
                    sound_action_type::SAMP_LOOP_FLAG => Ok(object.plock_clear_sample_loop_flag()?),
                    sound_action_type::SAMP_VOLUME => Ok(object.plock_clear_sample_volume()?),

                    other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
                }
                .map(|_| Response::Ok)
            }
            Some(ParsedValue::Enum(enum_type, _)) => {
                match enum_type.as_str() {
                    kit_enum_type::FX_COMP_ATTACK => Ok(object.plock_clear_fx_compressor_attack()?),
                    kit_enum_type::FX_COMP_RELEASE => {
                        Ok(object.plock_clear_fx_compressor_release()?)
                    }
                    kit_enum_type::FX_COMP_RATIO => Ok(object.plock_clear_fx_compressor_ratio()?),
                    kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => {
                        Ok(object.plock_clear_fx_compressor_side_chain_eq()?)
                    }
                    kit_enum_type::FX_LFO_DESTINATION => {
                        Ok(object.plock_clear_fx_lfo_destination()?)
                    }

                    // TODO: MACHINE
                    // sound_enum_type::MACHINE_PARAMETERS => todo!(),
                    sound_enum_type::LFO_DESTINATION => Ok(object.plock_clear_lfo_destination()?),
                    sound_enum_type::FILTER_TYPE => Ok(object.plock_clear_filter_type()?),
                    sound_enum_type::LFO_MULTIPLIER => Ok(object.plock_clear_lfo_multiplier()?),
                    sound_enum_type::LFO_WAVEFORM => Ok(object.plock_clear_lfo_waveform()?),
                    sound_enum_type::LFO_MODE => Ok(object.plock_clear_lfo_mode()?),

                    other => Err(InvalidEnumType(other.to_owned()).into()),
                }
                .map(|_| Response::Ok)
            }
            _ => {
                unreachable!("Parser should take care of this. Invalid plock clear format.")
            }
        },
    }
}
