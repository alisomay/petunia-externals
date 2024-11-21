use api::{
    global::{self},
    kit, pattern, settings, sound, Response,
};
use error::{QueryError, RytmObjectError, SendError};

use error_logger_macro::log_errors;
use parking_lot::Mutex;
use parse::{
    parse_command,
    types::{ObjectTypeSelector, ParsedValue},
};
use rytm_rs::{
    query::{GlobalQuery, KitQuery, PatternQuery, SettingsQuery, SoundQuery},
    RytmProject, SysexCompatible,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tracing::error;
use tracing::instrument;
use types::CommandType;
use value::RytmValueList;

pub mod api;
pub mod error;
pub mod parse;
pub mod types;
pub mod value;

pub struct RytmObject {
    pub project: Arc<Mutex<RytmProject>>,
    pub sysex_in_buffer: Arc<Mutex<Vec<u8>>>,
    pub buffering_sysex: AtomicBool,
}

impl RytmObject {
    // Constants for MIDI SysEx messages
    const SYSEX_START: u8 = 0xF0;
    const SYSEX_END: u8 = 0xF7;

    // TODO: This is going to be called a lot is this fine to instrument?
    #[instrument(skip(self))]
    #[log_errors]
    pub fn handle_sysex_byte(&self, byte: u8) -> Result<(), RytmObjectError> {
        if byte != Self::SYSEX_START && !self.buffering_sysex.load(Ordering::Acquire) {
            return Err(RytmObjectError::from(
                "Invalid input: Rytm only understands SysEx messages. Please connect sysexin object to the Rytm inlet.",
            ));
        }

        let mut sysex_buffer = self.sysex_in_buffer.lock();

        if byte == Self::SYSEX_START {
            self.buffering_sysex.store(true, Ordering::Release);
            sysex_buffer.clear(); // Clear any previous incomplete message
        }

        sysex_buffer.push(byte);

        // Process complete message
        if byte == Self::SYSEX_END {
            self.buffering_sysex.store(false, Ordering::Release);

            // Process the complete message
            let mut project = self.project.lock();

            project.update_from_sysex_response(&sysex_buffer)?;
            sysex_buffer.clear();
        }

        Ok(())
    }

    #[instrument]
    #[log_errors]
    pub fn prepare_query(
        query: RytmValueList,
        device_id: Option<u8>,
    ) -> Result<Vec<u8>, RytmObjectError> {
        let pair = match (query.first(), query.get(1)) {
            (None, Some(_) | None) => Err(QueryError::InvalidFormat),
            (Some(object_type), other) => Ok((object_type, other)),
        }?;

        let device_id = device_id.unwrap_or(0x00);

        Ok(match ObjectTypeSelector::try_from(pair)? {
            ObjectTypeSelector::Pattern(index) => {
                PatternQuery::new_with_device_id(index, device_id)
                    .unwrap()
                    .as_sysex()
            }
            ObjectTypeSelector::PatternWorkBuffer => {
                PatternQuery::new_targeting_work_buffer_with_device_id(device_id).as_sysex()
            }
            ObjectTypeSelector::Kit(index) => KitQuery::new_with_device_id(index, device_id)
                .unwrap()
                .as_sysex(),
            ObjectTypeSelector::KitWorkBuffer => {
                KitQuery::new_targeting_work_buffer_with_device_id(device_id).as_sysex()
            }
            ObjectTypeSelector::Sound(index) => SoundQuery::new_with_device_id(index, device_id)
                .unwrap()
                .as_sysex(),
            ObjectTypeSelector::SoundWorkBuffer(index) => {
                SoundQuery::new_targeting_work_buffer_with_device_id(index, device_id)
                    .unwrap()
                    .as_sysex()
            }
            ObjectTypeSelector::Global(index) => GlobalQuery::new_with_device_id(index, device_id)
                .unwrap()
                .as_sysex(),
            ObjectTypeSelector::GlobalWorkBuffer => {
                GlobalQuery::new_targeting_work_buffer_with_device_id(device_id).as_sysex()
            }
            ObjectTypeSelector::Settings => SettingsQuery::new_with_device_id(device_id).as_sysex(),
        }?)
    }

    #[instrument(skip(self))]
    #[log_errors]
    pub fn prepare_sysex(&self, selector: RytmValueList) -> Result<Vec<u8>, RytmObjectError> {
        let pair = match (selector.first(), selector.get(1)) {
            (None, Some(_) | None) => Err(SendError::InvalidFormat),
            (Some(object_type), other) => Ok((object_type, other)),
        }?;

        let project = self.project.lock();
        let work_buffer = project.work_buffer();
        Ok(match ObjectTypeSelector::try_from(pair)? {
            ObjectTypeSelector::Pattern(index) => project.patterns()[index].as_sysex(),
            ObjectTypeSelector::PatternWorkBuffer => work_buffer.pattern().as_sysex(),
            ObjectTypeSelector::Kit(index) => project.kits()[index].as_sysex(),
            ObjectTypeSelector::KitWorkBuffer => work_buffer.kit().as_sysex(),
            ObjectTypeSelector::Sound(index) => project.pool_sounds()[index].as_sysex(),
            ObjectTypeSelector::SoundWorkBuffer(index) => work_buffer.sounds()[index].as_sysex(),
            ObjectTypeSelector::Global(index) => project.globals()[index].as_sysex(),
            ObjectTypeSelector::GlobalWorkBuffer => work_buffer.global().as_sysex(),
            ObjectTypeSelector::Settings => project.settings().as_sysex(),
        }?)
    }

    #[instrument(skip(self))]
    pub fn command(
        &self,
        selector: CommandType,
        values: RytmValueList,
    ) -> Result<Response, RytmObjectError> {
        let tokens = parse_command(&values, selector)?;
        let Some(ParsedValue::ObjectType(kind)) = tokens.first().cloned() else {
            unreachable!("Parser should have caught this.");
        };
        match kind {
            ObjectTypeSelector::Pattern(index) => {
                pattern::handle(self, tokens, Some(index), selector)
            }
            ObjectTypeSelector::PatternWorkBuffer => pattern::handle(self, tokens, None, selector),
            ObjectTypeSelector::Kit(index) => kit::handle(self, tokens, Some(index), selector),
            ObjectTypeSelector::KitWorkBuffer => kit::handle(self, tokens, None, selector),
            ObjectTypeSelector::Sound(index) => {
                sound::handle(self, tokens, index, sound::SoundSource::Pool, selector)
            }
            ObjectTypeSelector::SoundWorkBuffer(index) => sound::handle(
                self,
                tokens,
                index,
                sound::SoundSource::WorkBuffer,
                selector,
            ),
            ObjectTypeSelector::Global(index) => {
                global::handle(self, tokens, Some(index), selector)
            }
            ObjectTypeSelector::GlobalWorkBuffer => global::handle(self, tokens, None, selector),
            ObjectTypeSelector::Settings => settings::handle(self, tokens, selector),
        }
    }
}
