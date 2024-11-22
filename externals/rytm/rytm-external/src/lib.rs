// Currently for the initial version we're working in a relatively relaxed way, later on we may want to be more strict.
// When the stabilization increases.
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::similar_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::missing_safety_doc,
    clippy::significant_drop_tightening,
    clippy::too_many_lines
)]
#![allow(clippy::must_use_candidate)]

pub mod class;
pub mod error;
pub mod file;
pub mod load_save;
pub mod tracing_setup;
pub mod traits;
pub mod trampoline;
pub mod types;
pub mod utils;

use crate::{error::RytmExternalError, traits::Post};
use error_logger_macro::log_errors;
use median::{
    atom::Atom,
    max_sys::t_atom_long,
    object::MaxObj,
    outlet::{OutAnything, OutInt, SendError},
    symbol::SymbolRef,
    wrapper::MaxObjWrapper,
};
use rytm_object::{api::Response, types::CommandType, value::RytmValue, RytmObject};
use std::sync::{
    atomic::{AtomicIsize, Ordering},
    Arc,
};
use tracing::{error, info, info_span, instrument, span::EnteredSpan, warn};
use tracing_setup::{get_default_env_filter, LoggingState};
use traits::SerialSend;

// This is the entry point for the Max external
#[no_mangle]
pub unsafe extern "C" fn ext_main(_r: *mut ::std::ffi::c_void) {
    // Call order 1

    // Register wrapped class with Max
    if std::panic::catch_unwind(|| RytmExternal::register()).is_err() {
        std::process::exit(1);
    }
}

// This is the actual object (external)
pub struct RytmExternal {
    /// Sysex device id
    pub target_device_id: AtomicIsize,
    pub root_span: EnteredSpan,
    pub subscriber: Arc<dyn tracing::Subscriber + Send + Sync + 'static>,
    pub sysex_out: OutInt,
    pub query_out: OutAnything,
    pub status_out: OutInt,
    pub inner: rytm_object::RytmObject,
    pub logging_state: Arc<LoggingState>,
}

impl RytmExternal {
    /// Utility to register your wrapped class with Max
    pub(crate) unsafe fn register() {
        // Call order 2
        MaxObjWrapper::<Self>::register(false);
    }

    const SELECTOR_QUERY: &'static str = "query";
    const SELECTOR_SEND: &'static str = "send";
    const SELECTOR_SET: &'static str = "set";
    const SELECTOR_GET: &'static str = "get";
    const SELECTOR_LOG_LEVEL: &'static str = "loglevel";

    // TODO: Implementations for these are sketches.
    // For proper impl move some of the logic to the RytmObject.
    // Make nice interfaces with proper error handling management here.

    const SELECTOR_LOAD: &'static str = "load";
    const SELECTOR_SAVE: &'static str = "save";

    pub fn int(&self, value: t_atom_long) -> Result<(), RytmExternalError> {
        tracing::subscriber::with_default(Arc::clone(&self.subscriber), || {
            self.root_span.in_scope(|| {
                let _function_span = info_span!("int", "value" = value).entered();

                let _inlet_index = median::inlet::Proxy::get_inlet(self.max_obj());
                let byte = u8::try_from(value).map_err(|_| {
                    RytmExternalError::from(
                        "Rytm Error: Invalid input. rytm only understands sysex messages. Please connect sysexin object to the rytm inlet to make sure you pass in only sysex messages.",
                    )}).inspect_err(
                        |err| {
                            error!("{}", err);
                        }
                    )?;
                // This one already logs errors in the object.
                Ok(self.inner.handle_sysex_byte(byte)?)
            })
        })
    }

    pub fn anything_with_selector(
        &self,
        sel: &SymbolRef,
        atoms: &[Atom],
    ) -> Result<(), RytmExternalError> {
        tracing::subscriber::with_default(Arc::clone(&self.subscriber), || {
            self.root_span.in_scope(|| {
                let _function_span = info_span!("anything_with_selector").entered();
                let selector = sel
                    .to_string()
                    .map_err(|err|{
                         self.send_status_error();
                         RytmExternalError::Custom(err.to_string())
                    })?;

                let possible_selectors = [
                    Self::SELECTOR_QUERY,
                    Self::SELECTOR_SEND,
                    Self::SELECTOR_SET,
                    Self::SELECTOR_GET,
                    Self::SELECTOR_LOG_LEVEL,
                    Self::SELECTOR_LOAD,
                    Self::SELECTOR_SAVE,
                ].join(", ");
                match selector.as_str() {
                    Self::SELECTOR_QUERY => self.query(atoms),
                    Self::SELECTOR_SEND => self.send(atoms),
                    Self::SELECTOR_SET => self.set(atoms),
                    Self::SELECTOR_GET => self.get(atoms),
                    Self::SELECTOR_LOG_LEVEL => self.change_log_level(atoms),
                    Self::SELECTOR_LOAD => self.load(atoms),
                    Self::SELECTOR_SAVE => self.save(atoms),
                    _ => Err(format!("Parse Error: Invalid command type {selector}. Possible commands are {possible_selectors}.").into()),
                }.inspect_err(|_| {
                    if selector.as_str() != Self::SELECTOR_LOG_LEVEL {
                        self.send_status_error();
                    }
                })
            })
        })
    }

    #[instrument(skip_all)]
    #[log_errors]
    pub fn change_log_level(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let values = self.get_rytm_values(atoms)?;
        if values.len() != 1 {
            self.send_status_error();
            return Err(RytmExternalError::from(
                "Command Error: Invalid format. Only one symbol is allowed for changing the log level.",
            ));
        }
        let Some(RytmValue::Symbol(maybe_level)) = values.first() else {
            self.send_status_error();
            return Err(RytmExternalError::from(
                "Command Error: Invalid format. Only one symbol is allowed for changing the log level.",
            ));
        };

        let new_level = match maybe_level.as_str() {
            "error" => tracing::Level::ERROR,
            "warn" => tracing::Level::WARN,
            "info" => tracing::Level::INFO,
            "debug" => tracing::Level::DEBUG,
            "trace" => tracing::Level::TRACE,
            _ => {
                self.send_status_error();
                return Err(RytmExternalError::from(
                    "Command Error: Invalid format. Only one symbol is allowed for changing the log level. It needs to be either error, warn, info, debug or trace.",
                ));
            }
        };

        let (changed, info) = apply_new_log_level_if_necessary(new_level, &self.logging_state);

        if changed {
            self.send_status_success();
            info.obj_post(self.max_obj());
        } else {
            self.send_status_warning();
            info.obj_warn(self.max_obj());
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn query(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        // Actually the attribute which sets this will is clipped to 0-127 but just in case:

        let device_id =
            u8::try_from(self.target_device_id.load(Ordering::SeqCst)).map_err(|_| {
                RytmExternalError::from(
                    "Query Error: Invalid device id. Device id should be between 0 and 127.",
                )
            })?;

        if device_id > 127 {
            return Err(RytmExternalError::from(
                "Query Error: Invalid device id. Device id should be between 0 and 127.",
            ));
        }

        let sysex = RytmObject::prepare_query(self.get_rytm_values(atoms)?, Some(device_id))?;

        sysex.serial_send_int(&self.sysex_out);
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn send(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let sysex = self.inner.prepare_sysex(self.get_rytm_values(atoms)?)?;
        sysex.serial_send_int(&self.sysex_out);

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn set(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        self.response_to_outlet(
            self.inner
                .command(CommandType::Set, self.get_rytm_values(atoms)?)?,
        )
        .ok();

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn get(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        self.response_to_outlet(
            self.inner
                .command(CommandType::Get, self.get_rytm_values(atoms)?)?,
        )
        .ok();

        Ok(())
    }

    #[instrument(skip_all)]
    #[log_errors]
    fn get_rytm_values(
        &self,
        atoms: &[Atom],
    ) -> Result<rytm_object::value::RytmValueList, RytmExternalError> {
        atoms.try_into().map_err(|()| {
            RytmExternalError::from(
                "Rytm Error: Invalid data type. Rytm object only accepts a list of integers floats or symbols",
            )
        })
    }

    #[instrument(skip(self))]
    fn response_to_outlet(&self, res: Response) -> Result<(), SendError> {
        self.send_status_success();
        match res {
            Response::Common { index, key, value } => self
                .query_out
                .send(&[Atom::from(index as isize), key.as_atom(), value.as_atom()][..]),
            Response::KitElement {
                kit_index,
                element_index,
                element_type,
                value,
            } => self.query_out.send(
                &[
                    Atom::from(kit_index as isize),
                    Atom::from(element_index as isize),
                    element_type.as_atom(),
                    value.as_atom(),
                ][..],
            ),
            Response::Track {
                pattern_index,
                track_index,
                key,
                value,
            } => self.query_out.send(
                &[
                    Atom::from(pattern_index as isize),
                    Atom::from(track_index as isize),
                    key.as_atom(),
                    value.as_atom(),
                ][..],
            ),
            Response::Trig {
                pattern_index,
                track_index,
                trig_index,
                key,
                value,
            } => self.query_out.send(
                &[
                    Atom::from(pattern_index as isize),
                    Atom::from(track_index as isize),
                    Atom::from(trig_index as isize),
                    key.as_atom(),
                    value.as_atom(),
                ][..],
            ),
            Response::Ok => Ok(()),
        }
        .inspect_err(|_| {
            "Error sending to results outlet due to stack overflow.".obj_warn(self.max_obj());
            warn!("Error sending to results outlet due to stack overflow.");
        })
    }

    fn send_status(&self, code: isize) {
        self.status_out
            .send(code)
            .inspect_err(|_| {
                "Error sending to status outlet due to stack overflow.".obj_warn(self.max_obj());
                warn!("Error sending to status outlet due to stack overflow.");
            })
            .ok();
    }

    fn send_status_success(&self) {
        self.send_status(0);
    }

    fn send_status_error(&self) {
        self.send_status(1);
    }

    fn send_status_warning(&self) {
        self.send_status(2);
    }
}

#[instrument(skip(logging_state))]
pub fn apply_new_log_level_if_necessary(
    new_level: tracing::Level,
    logging_state: &LoggingState,
) -> (bool, String) {
    let mut active_log_level = logging_state.active_level.lock();
    let mut is_changed: bool = true;
    let mut information: String = format!(
        "Previous logging level was already set to: {new_level}. Log level was not changed.",
    );

    if *active_log_level == new_level {
        (false, information)
    } else {
        let previous_level = *active_log_level;
        let new_filter = get_default_env_filter().add_directive(new_level.into());

        logging_state
            .reload_handle
            .reload(new_filter)
            .inspect_err(|err| {
                is_changed = false;
                information = format!(
                    "Failed to change log level from {previous_level} to {new_level}: {err:?}"
                );
                warn!("{}", information);
            })
            .ok();

        *active_log_level = new_level;

        information =
            format!("Default log level {previous_level} is successfully changed to: {new_level}");

        info!("{}", information);

        (is_changed, information)
    }
}
