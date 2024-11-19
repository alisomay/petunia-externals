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
pub mod tracing_setup;
pub mod traits;
pub mod trampoline;

use std::sync::Arc;

use crate::error::RytmExternalError;
use crate::traits::Post;
use error_logger_macro::log_errors;
use median::atom::AtomValue;
use median::outlet::{OutAnything, SendError};
use median::{atom::Atom, max_sys::t_atom_long, object::MaxObj, outlet::OutInt, symbol::SymbolRef};
use rytm_object::api::Response;
use rytm_object::types::CommandType;
use rytm_object::value::RytmValue;
use rytm_object::RytmObject;
use tracing::error;
use tracing::{info, instrument, warn};
use tracing_setup::{get_default_env_filter, LoggingState};
use tracing_subscriber::{reload, EnvFilter};
use traits::SerialSend;

// Should be only set through the debug 1 or debug 0 messages.
// Should be only set from one place in the code, no other functions or threads.
// Make sure that no other code is accessing this variable while it is being set.
// Anything other than that is undefined behavior.
pub static mut RYTM_EXTERNAL_DEBUG: bool = true;

// This is the entry point for the Max external
#[no_mangle]
pub unsafe extern "C" fn ext_main(_r: *mut ::std::ffi::c_void) {
    // Register your wrapped class with Max
    if std::panic::catch_unwind(|| RytmExternal::register()).is_err() {
        std::process::exit(1);
    }
}

pub type ReloadHandle = reload::Handle<EnvFilter, tracing_subscriber::Registry>;

// This is the actual object (external)
pub struct RytmExternal {
    pub sysex_out: OutInt,
    pub query_out: OutAnything,
    pub inner: rytm_object::RytmObject,
    pub logging_state: Arc<LoggingState>,
}

// The main trait for your object
impl median::wrapper::ObjWrapped<Self> for RytmExternal {
    fn class_name() -> &'static str {
        "rytm"
    }

    // You can modify the object here such as adding assists etc.
    // TODO: Maybe add notification handling.
}

impl RytmExternal {
    /// Utility to register your wrapped class with Max
    pub(crate) unsafe fn register() {
        median::wrapper::MaxObjWrapper::<Self>::register(false);
    }

    const SELECTOR_QUERY: &'static str = "query";
    const SELECTOR_SEND: &'static str = "send";
    const SELECTOR_SET: &'static str = "set";
    const SELECTOR_GET: &'static str = "get";
    const SELECTOR_DEBUG: &'static str = "debug";
    const SELECTOR_LOG_LEVEL: &'static str = "loglevel";

    #[instrument(skip_all)]
    #[log_errors]
    fn debug_mode(_sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        if let Some(atom) = atoms.first() {
            if let Some(AtomValue::Int(value)) = atom.get_value() {
                // Check lib.rs for safety.
                // In addition debug post should be never used in this function.
                unsafe {
                    if value == 1 {
                        crate::RYTM_EXTERNAL_DEBUG = true;
                        return Ok(());
                    } else if value == 0 {
                        crate::RYTM_EXTERNAL_DEBUG = false;
                        return Ok(());
                    }
                    return Err(RytmExternalError::from(
                        "Invalid value: Only 0 or 1 are allowed for setting the debug mode.",
                    ));
                }
            }
            return Err(RytmExternalError::from(
                "Invalid value: Only 0 or 1 are allowed for setting the debug mode.",
            ));
        }
        Err(RytmExternalError::from(
            "Invalid format: 0 or 1 should follow the debug keyword.",
        ))
    }

    #[instrument(skip(self))]
    pub fn int(&self, value: t_atom_long) -> Result<(), RytmExternalError> {
        let _inlet_index = median::inlet::Proxy::get_inlet(self.max_obj());
        let byte = u8::try_from(value).map_err(|_| {
            RytmExternalError::from(
                "Invalid input: rytm only understands sysex messages. Please connect sysexin object to the rytm inlet to make sure you pass in only sysex messages.",
            )
        }).inspect_err(
            |err| {
                error!("{}", err);
            }
        )?;

        // This one already logs errors in the object.
        Ok(self.inner.handle_sysex_byte(byte)?)
    }

    #[instrument(skip_all)]
    pub fn anything_with_selector(
        &self,
        sel: &SymbolRef,
        atoms: &[Atom],
    ) -> Result<(), RytmExternalError> {
        let selector = sel
            .to_string()
            .map_err(|err| RytmExternalError::Custom(err.to_string()))?;

        match selector.as_str() {
            Self::SELECTOR_QUERY => self.query(atoms),
            Self::SELECTOR_SEND => self.send(atoms),
            Self::SELECTOR_SET => self.set(atoms),
            Self::SELECTOR_GET => self.get(atoms),
            Self::SELECTOR_DEBUG => Self::debug_mode(sel, atoms),
            Self::SELECTOR_LOG_LEVEL => self.change_log_level(atoms),
            _ => Err(format!("Invalid selector: {selector}. Possible selectors are query, send, set, get, debug.").into()),
        }
    }

    #[instrument(skip_all)]
    #[log_errors]
    pub fn change_log_level(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let values = Self::get_rytm_values(atoms)?;
        if values.len() != 1 {
            return Err(RytmExternalError::from(
                "Invalid format: Only one symbol is allowed for changing the log level.",
            ));
        }
        let Some(RytmValue::Symbol(maybe_level)) = values.first() else {
            return Err(RytmExternalError::from(
                "Invalid format: Only one symbol is allowed for changing the log level.",
            ));
        };

        let new_level = match maybe_level.as_str() {
            "error" => tracing::Level::ERROR,
            "warn" => tracing::Level::WARN,
            "info" => tracing::Level::INFO,
            "debug" => tracing::Level::DEBUG,
            "trace" => tracing::Level::TRACE,
            _ => {
                return Err(RytmExternalError::from(
                    "Invalid format: Only one symbol is allowed for changing the log level. It needs to be either error, warn, info, debug or trace.",
                ));
            }
        };

        let mut active_log_level = self.logging_state.active_level.lock();

        if *active_log_level != new_level {
            let mut new_filter = get_default_env_filter();
            new_filter = new_filter.add_directive(new_level.into());

            self.logging_state
                .reload_handle
                .reload(new_filter)
                .inspect_err(|err| {
                    warn!(
                        "Failed to change log level from {} to {}: {:?}",
                        active_log_level, new_level, err
                    );
                })
                .ok();

            *active_log_level = new_level;
            info!(
                "Default log level {} is successfully changed to: {}",
                active_log_level, new_level
            );
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn query(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let sysex = RytmObject::prepare_query(Self::get_rytm_values(atoms)?)?;
        sysex.serial_send_int(&self.sysex_out);
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn send(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let sysex = self.inner.prepare_sysex(Self::get_rytm_values(atoms)?)?;
        sysex.serial_send_int(&self.sysex_out);
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn set(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        self.response_to_outlet(
            self.inner
                .command(CommandType::Set, Self::get_rytm_values(atoms)?)?,
        )
        .ok();

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn get(&self, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        self.response_to_outlet(
            self.inner
                .command(CommandType::Get, Self::get_rytm_values(atoms)?)?,
        )
        .ok();

        Ok(())
    }

    #[instrument(skip_all)]
    #[log_errors]
    fn get_rytm_values(
        atoms: &[Atom],
    ) -> Result<rytm_object::value::RytmValueList, RytmExternalError> {
        atoms.try_into().map_err(|()| {
            RytmExternalError::from(
                "Invalid format: Rytm object only accepts a list of integers floats or symbols",
            )
        })
    }

    #[instrument(skip(self))]
    fn response_to_outlet(&self, res: Response) -> Result<(), SendError> {
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
            "Error sending to outlet due to stack overflow.".error();
            warn!("Error sending to outlet due to stack overflow.");
        })
    }
}
