use std::sync::{atomic::AtomicBool, Arc};

use median::{
    builder::MaxWrappedBuilder,
    class::Class,
    wrapper::{MaxObjWrapped, MaxObjWrapper},
};
use rytm_rs::RytmProject;

use super::RytmExternal;
use median::method::*;
use parking_lot::Mutex;
pub use tracing::{self, error, info, instrument, span, trace, warn, Level};

// This trait is for Max specific objects, there is another one for MSP objects.
impl MaxObjWrapped<Self> for RytmExternal {
    // The constructor for your object
    fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
        // You can also add inlets/outlets here modifying the builder
        builder.with_default_inlet_assist("sysex input (connect sysexin)");

        let (registry, logging_state) = crate::tracing_setup::setup_logging();
        tracing::subscriber::set_global_default(registry).ok();

        let project = RytmProject::try_default()
            .inspect_err(|err| error!("Error creating RytmProject: {}", err))
            .unwrap();

        Self {
            sysex_out: builder.add_int_outlet_with_assist("sysex output (connect to midiout)"),
            query_out: builder.add_anything_outlet_with_assist("get query results (list)"),
            inner: rytm_object::RytmObject {
                project: Arc::new(Mutex::new(project)),
                sysex_in_buffer: Arc::new(Mutex::new(Vec::new())),
                buffering_sysex: AtomicBool::new(false),
            },
            logging_state,
        }
    }

    // Setup your class here
    fn class_setup(class: &mut Class<MaxObjWrapper<Self>>) {
        // TODO: Add attribute for setting the device id
        class.add_method(Method::Int(Self::int_tramp)).unwrap();
        class
            .add_method(Method::Anything(Self::anything_with_selector_tramp))
            .unwrap();
    }
}
