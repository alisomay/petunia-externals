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
use rytm_rs::object::Global;
use tracing::{error, instrument};

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
            let object =
                index.map_or_else(|| guard.work_buffer().global(), |i| &guard.globals()[i]);
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
            let object = if let Some(i) = index {
                &mut guard.globals_mut()[i]
            } else {
                guard.work_buffer_mut().global_mut()
            };

            match next_token {
                Some(ParsedValue::Enum(variant, value)) => {
                    set_enum(object, &mut tokens, variant, value)
                }
                Some(ParsedValue::Identifier(action)) => set_action(object, &mut tokens, action),
                _ => {
                    unreachable!("Parser should take care of this. Invalid setter format.")
                }
            }
        }
        CommandType::Copy => Ok(Response::Unsupported(
            "Currently copy command is not supported for global object. If you need this badly please open an issue and implementation will be considered.".into(),
        )),
    }
}

#[instrument(skip(object))]
#[log_errors]
fn get_enum(
    object: &Global,
    variant: &str,
    value: &Option<String>,
) -> Result<RytmValue, RytmObjectError> {
    use crate::api::global_enum_type::*;
    let result: &str = match variant {
        METRONOME_TIME_SIGNATURE => object.metronome_settings().time_signature().into(),

        ROUTING_USB_IN_OPTIONS => object.routing().usb_in().into(),
        ROUTING_USB_OUT_OPTIONS => object.routing().usb_out().into(),
        ROUTING_USB_TO_MAIN_DB => object.routing().usb_to_main_db().into(),

        OUT_PORT_FUNCTION => object
            .midi_config()
            .port_config()
            .output_port_function()
            .into(),
        THRU_PORT_FUNCTION => object
            .midi_config()
            .port_config()
            .thru_port_function()
            .into(),
        INPUT_FROM => object.midi_config().port_config().input_transport().into(),
        OUTPUT_TO => object.midi_config().port_config().output_transport().into(),
        PARAM_OUTPUT => object
            .midi_config()
            .port_config()
            .parameter_output_type()
            .into(),
        PAD_DEST => object
            .midi_config()
            .port_config()
            .pad_parameter_destination()
            .into(),
        PRESSURE_DEST => object
            .midi_config()
            .port_config()
            .pressure_parameter_destination()
            .into(),
        ENCODER_DEST => object
            .midi_config()
            .port_config()
            .encoder_parameter_destination()
            .into(),
        MUTE_DEST => object
            .midi_config()
            .port_config()
            .mute_parameter_destination()
            .into(),
        PORTS_OUTPUT_CHANNEL => object
            .midi_config()
            .port_config()
            .ports_output_channel()
            .into(),

        AUTO_CHANNEL => object.midi_config().channels().auto_channel().into(),

        TRACK_CHANNELS => {
            let Some(track_index) = value else {
                return Err(GetError::InvalidFormat("trackchannels: should include an integer track index. Example: trackchannels:1".to_owned()).into());
            };
            let track_index = track_index.parse::<usize>().map_err(|_| GetError::InvalidFormat("trackchannels: should include an integer track index. Example: trackchannels:1".to_owned()))?;

            object
                .midi_config()
                .channels()
                .track_channel(track_index)?
                .into()
        }
        TRACK_FX_CHANNEL => object.midi_config().channels().track_fx_channel().into(),
        PROGRAM_CHANGE_IN_CHANNEL => object
            .midi_config()
            .channels()
            .program_change_in_channel()
            .into(),
        PROGRAM_CHANGE_OUT_CHANNEL => object
            .midi_config()
            .channels()
            .program_change_out_channel()
            .into(),
        PERFORMANCE_CHANNEL => object.midi_config().channels().performance_channel().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn get_action(
    object: &Global,
    tokens: &mut std::slice::Iter<ParsedValue>,
    action: &str,
) -> Result<RytmValue, RytmObjectError> {
    use crate::api::global_action_type::*;
    let result = match action {
        VERSION => object.structure_version() as isize,
        INDEX => object.index() as isize,
        IS_WORK_BUFFER => isize::from(object.is_work_buffer()),

        KIT_RELOAD_ON_CHANGE => object.sequencer_config().kit_reload_on_chg().into(),
        QUANTIZE_LIVE_REC => object.sequencer_config().quantize_live_rec().into(),
        AUTO_TRACK_SWITCH => object.sequencer_config().auto_trk_switch().into(),

        ROUTE_TO_MAIN => {
            let Some(ParsedValue::Parameter(Number::Int(param))) = tokens.next() else {
                return Err(GetError::InvalidFormat(
                    "routetomain should be followed by an integer track index.".to_owned(),
                )
                .into());
            };

            object
                .routing()
                .is_track_routed_to_main(*param as usize)
                .into()
        }

        SEND_TO_FX => {
            let Some(ParsedValue::Parameter(Number::Int(param))) = tokens.next() else {
                return Err(GetError::InvalidFormat(
                    "sendtofx should be followed by a track index (integer)".to_owned(),
                )
                .into());
            };

            object.routing().is_track_sent_to_fx(*param as usize).into()
        }

        CLOCK_RECEIVE => object.midi_config().sync().clock_receive().into(),
        CLOCK_SEND => object.midi_config().sync().clock_send().into(),
        TRANSPORT_RECEIVE => object.midi_config().sync().transport_receive().into(),
        TRANSPORT_SEND => object.midi_config().sync().transport_send().into(),
        PROGRAM_CHANGE_RECEIVE => object.midi_config().sync().program_change_receive().into(),
        PROGRAM_CHANGE_SEND => object.midi_config().sync().program_change_send().into(),

        RECEIVE_NOTES => object.midi_config().port_config().receive_notes().into(),
        RECEIVE_CC_NRPN => object.midi_config().port_config().receive_cc_nrpn().into(),
        TURBO_SPEED => object.midi_config().port_config().turbo_speed().into(),

        METRONOME_ACTIVE => object.metronome_settings().is_active().into(),
        METRONOME_PRE_ROLL_BARS => object.metronome_settings().pre_roll_bars() as isize,
        METRONOME_VOLUME => object.metronome_settings().volume() as isize,

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    Ok(result.into())
}

#[instrument(skip(object))]
#[log_errors]
fn set_enum(
    object: &mut Global,
    tokens: &mut std::slice::Iter<ParsedValue>,
    variant: &str,
    value: &Option<String>,
) -> Result<Response, RytmObjectError> {
    let enum_value = value
        .clone()
        .ok_or_else(|| SetError::InvalidFormat("Enum value not provided".into()))?;

    use crate::api::global_enum_type::*;
    match variant {
        METRONOME_TIME_SIGNATURE => object
            .metronome_settings_mut()
            .set_time_signature(enum_value.as_str().try_into()?),

        ROUTING_USB_IN_OPTIONS => object
            .routing_mut()
            .set_usb_in(enum_value.as_str().try_into()?),
        ROUTING_USB_OUT_OPTIONS => object
            .routing_mut()
            .set_usb_out(enum_value.as_str().try_into()?),
        ROUTING_USB_TO_MAIN_DB => object
            .routing_mut()
            .set_usb_to_main_db(enum_value.as_str().try_into()?),

        OUT_PORT_FUNCTION => object
            .midi_config_mut()
            .port_config_mut()
            .set_output_port_function(enum_value.as_str().try_into()?),
        THRU_PORT_FUNCTION => object
            .midi_config_mut()
            .port_config_mut()
            .set_thru_port_function(enum_value.as_str().try_into()?),
        INPUT_FROM => object
            .midi_config_mut()
            .port_config_mut()
            .set_input_transport(enum_value.as_str().try_into()?),
        OUTPUT_TO => object
            .midi_config_mut()
            .port_config_mut()
            .set_output_transport(enum_value.as_str().try_into()?),
        PARAM_OUTPUT => object
            .midi_config_mut()
            .port_config_mut()
            .set_parameter_output_type(enum_value.as_str().try_into()?),
        PAD_DEST => object
            .midi_config_mut()
            .port_config_mut()
            .set_pad_parameter_destination(enum_value.as_str().try_into()?),
        PRESSURE_DEST => object
            .midi_config_mut()
            .port_config_mut()
            .set_pressure_parameter_destination(enum_value.as_str().try_into()?),
        ENCODER_DEST => object
            .midi_config_mut()
            .port_config_mut()
            .set_encoder_parameter_destination(enum_value.as_str().try_into()?),
        MUTE_DEST => object
            .midi_config_mut()
            .port_config_mut()
            .set_mute_parameter_destination(enum_value.as_str().try_into()?),
        PORTS_OUTPUT_CHANNEL => object
            .midi_config_mut()
            .port_config_mut()
            .set_ports_output_channel(enum_value.as_str().try_into()?),

        AUTO_CHANNEL => object
            .midi_config_mut()
            .channels_mut()
            .set_auto_channel(enum_value.as_str().try_into()?)?,

        TRACK_CHANNELS => {
            let Some(ParsedValue::Parameter(Number::Int(param))) = tokens.next() else {
                return Err(
                    SetError::InvalidFormat("trackchannels should be followed by an integer track index. Format: trackchannels:<channel> <track index>. Example: trackchannels:1 2".to_owned()).into(),
                );
            };

            object
                .midi_config_mut()
                .channels_mut()
                .set_track_channel(*param as usize, enum_value.as_str().try_into()?)?
        }
        TRACK_FX_CHANNEL => object
            .midi_config_mut()
            .channels_mut()
            .set_track_fx_channel(enum_value.as_str().try_into()?)?,
        PROGRAM_CHANGE_IN_CHANNEL => object
            .midi_config_mut()
            .channels_mut()
            .set_program_change_in_channel(enum_value.as_str().try_into()?)?,
        PROGRAM_CHANGE_OUT_CHANNEL => object
            .midi_config_mut()
            .channels_mut()
            .set_program_change_out_channel(enum_value.as_str().try_into()?)?,
        PERFORMANCE_CHANNEL => object
            .midi_config_mut()
            .channels_mut()
            .set_performance_channel(enum_value.as_str().try_into()?)?,

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(Response::Ok)
}

#[instrument(skip(object))]
#[log_errors]
fn set_action(
    object: &mut Global,
    tokens: &mut std::slice::Iter<ParsedValue>,
    maybe_action: &String,
) -> Result<Response, RytmObjectError> {
    use crate::api::global_action_type::*;

    let param = number_or_set_error(tokens)?;

    match maybe_action.as_str() {
        KIT_RELOAD_ON_CHANGE => {
            object
                .sequencer_config_mut()
                .set_kit_reload_on_chg(param.get_bool_from_0_or_1(KIT_RELOAD_ON_CHANGE)?);
        }
        QUANTIZE_LIVE_REC => {
            object
                .sequencer_config_mut()
                .set_quantize_live_rec(param.get_bool_from_0_or_1(QUANTIZE_LIVE_REC)?);
        }
        AUTO_TRACK_SWITCH => {
            object
                .sequencer_config_mut()
                .set_auto_trk_switch(param.get_bool_from_0_or_1(AUTO_TRACK_SWITCH)?);
        }

        ROUTE_TO_MAIN => {
            object
                .routing_mut()
                .route_track_to_main(param.get_int() as usize)?;
        }
        SEND_TO_FX => {
            object
                .routing_mut()
                .send_track_to_fx(param.get_int() as usize)?;
        }

        CLOCK_RECEIVE => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_clock_receive(param.get_bool_from_0_or_1(CLOCK_RECEIVE)?);
        }
        CLOCK_SEND => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_clock_send(param.get_bool_from_0_or_1(CLOCK_SEND)?);
        }
        TRANSPORT_RECEIVE => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_transport_receive(param.get_bool_from_0_or_1(TRANSPORT_RECEIVE)?);
        }
        TRANSPORT_SEND => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_transport_send(param.get_bool_from_0_or_1(TRANSPORT_SEND)?);
        }
        PROGRAM_CHANGE_RECEIVE => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_program_change_receive(param.get_bool_from_0_or_1(PROGRAM_CHANGE_RECEIVE)?);
        }
        PROGRAM_CHANGE_SEND => {
            object
                .midi_config_mut()
                .sync_mut()
                .set_program_change_send(param.get_bool_from_0_or_1(PROGRAM_CHANGE_SEND)?);
        }

        RECEIVE_NOTES => {
            object
                .midi_config_mut()
                .port_config_mut()
                .set_receive_notes(param.get_bool_from_0_or_1(RECEIVE_NOTES)?);
        }
        RECEIVE_CC_NRPN => {
            object
                .midi_config_mut()
                .port_config_mut()
                .set_receive_cc_nrpn(param.get_bool_from_0_or_1(RECEIVE_CC_NRPN)?);
        }

        _ => return Err(IdentifierError::InvalidType(maybe_action.to_owned()).into()),
    }

    Ok(Response::Ok)
}
