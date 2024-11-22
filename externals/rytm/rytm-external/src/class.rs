use super::RytmExternal;
use crate::traits::Post;
use median::{
    attr::{AttrBuilder, AttrClip, AttrType, AttrValClip},
    builder::MaxWrappedBuilder,
    class::Class,
    max_sys,
    method::*,
    notify::Notification,
    object::MaxObj,
    symbol::SymbolRef,
    wrapper::{MaxObjWrapped, MaxObjWrapper, ObjWrapped},
};
use parking_lot::Mutex;
use rytm_rs::RytmProject;
use std::{
    ffi::CString,
    sync::{
        atomic::{AtomicBool, AtomicIsize},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::info_span;
pub use tracing::{self, error, info, instrument, span, trace, warn, Level};

// The main trait for the object
impl ObjWrapped<Self> for RytmExternal {
    fn class_name() -> &'static str {
        // Call order 3
        "rytm"
    }

    fn class_type() -> median::class::ClassType {
        // Call order 5
        median::class::ClassType::Box
    }

    fn handle_notification(&self, notification: &Notification) {
        tracing::subscriber::with_default(Arc::clone(&self.subscriber), || {
            self.root_span.in_scope(|| {
                let sender_name = notification.sender_name().to_string();
                let message = notification.message().to_string();
                let _function_span = info_span!("handle_notification", sender_name = ?sender_name, message = ?message).entered();

                // Handle notifications here
                "Currently rytm does not handle any notifications.".obj_warn(self.max_obj());
                warn!("Currently rytm does not handle any notifications.");
                self.send_status_warning();
            });
        });
    }
}

// This trait is for Max specific objects, there is another one for MSP objects.
impl MaxObjWrapped<Self> for RytmExternal {
    // The constructor for your object
    fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
        // Call order 6 (end instantiation)

        let (registry, logging_state) = crate::tracing_setup::setup_logging();

        tracing::subscriber::with_default(Arc::clone(&registry), || {
            let cwd = std::env::current_dir().map_or_else(
                |_| "unknown".to_string(),
                |p| p.to_string_lossy().to_string(),
            );

            let args = builder.creation_args();
            let args = args
                .iter()
                .map(|arg| arg.get_symbol().to_cstring().to_string_lossy().to_string())
                .collect::<Vec<String>>();

            let root_span = tracing::info_span!(
                "rytm_external",
                args = ?args,
                version = %env!("CARGO_PKG_VERSION"),
                os = %std::env::consts::OS,
                arch = %std::env::consts::ARCH,
                pid = %std::process::id(),
                cwd = %cwd,
                exe_path = %std::env::current_exe().map_or_else(|_| "unknown".to_string(), |p| p.to_string_lossy().to_string()),
                timestamp = %SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            ).entered();

            // Inlets
            builder.with_default_inlet_assist("sysex input (connect sysexin)");

            let project = RytmProject::try_default()
                .inspect_err(|err| error!("Error creating RytmProject: {}", err))
                .expect("Failed to create RytmProject");

            let instance = Self {
                target_device_id: AtomicIsize::new(0),
                root_span,
                subscriber: registry,
                sysex_out: builder.add_int_outlet_with_assist("sysex output (connect to midiout)"),
                query_out: builder.add_anything_outlet_with_assist("get query results (list)"),
                status_out: builder.add_int_outlet_with_assist(
                    "command status: 0 for success, 1 and 2 for error and warning (int)",
                ),
                inner: rytm_object::RytmObject {
                    project: Arc::new(Mutex::new(project)),
                    sysex_in_buffer: Arc::new(Mutex::new(Vec::new())),
                    buffering_sysex: AtomicBool::new(false),
                },
                logging_state,
            };

            info!("Rytm is instantiated ({:p}).", &instance.max_obj());

            instance
        })
    }

    fn class_setup(class: &mut Class<MaxObjWrapper<Self>>) {
        // Call order 4

        // Attributes

        class
            .add_attribute(
                AttrBuilder::new_accessors(
                    "sysex_id",
                    AttrType::Int64,
                    Self::attr_get_sysex_id_tramp,
                    Self::attr_set_sysex_id_tramp,
                )
                .clip(AttrClip::Set(AttrValClip::MinMax(0.0, 127.0)))
                .build()
                .expect("Failed to build sysex_id attribute"),
            )
            .expect("Failed to add sysex_id attribute");

        // Adding the save flag to the attribute
        // Currently this is not possible with median so it is saved with the patcher.

        // c74_max_object.h is the source for this info.
        // Mimicing this CLASS_ATTR_ATTR_PARSE(c,attrname,"save", c74::max::gensym("long"),flags,"1")

        let attrname = CString::new("sysex_id").unwrap();
        let attrname2 = CString::new("save").unwrap();
        let parsestring = CString::new("1").unwrap();
        let type_ = SymbolRef::try_from("long").unwrap();

        unsafe {
            max_sys::class_attr_addattr_parse(
                class.inner(),
                attrname.as_ptr(),
                attrname2.as_ptr(),
                type_.inner(),
                (max_sys::e_max_attrflags::ATTR_GET_DEFER_LOW
                    | max_sys::e_max_attrflags::ATTR_SET_DEFER_LOW)
                    .into(),
                parsestring.as_ptr(),
            );

            // Forget these, since they're heap allocations and we don't know how long they should live. It is in Max's hands now.
            std::mem::forget(attrname);
            std::mem::forget(attrname2);
            std::mem::forget(parsestring);
        }

        // Methods

        class
            .add_method(Method::Int(Self::int_tramp))
            .expect("Failed to add class method int_tramp");
        class
            .add_method(Method::Anything(Self::anything_with_selector_tramp))
            .expect("Failed to add class method anything_with_selector_tramp");
    }
}

// impl FilePath {
//     /// Get the full pathname using basic Max path formatting
//     pub fn to_full_path(&self) -> Option<CString> {
//         let mut full_path = [0 as c_char; MAX_PATH_CHARS];
//         unsafe {
//             if max_sys::path_topathname(
//                 self.vol,
//                 self.file_name.as_ptr(),
//                 full_path.as_mut_ptr()
//             ) == 0 {
//                 Some(CStr::from_ptr(full_path.as_ptr()).to_owned())
//             } else {
//                 None
//             }
//         }
//     }

//     /// Get the absolute system path (e.g. POSIX style on Mac)
//     /// This is preferred when passing paths to external libraries or system calls
//     pub fn to_absolute_system_path(&self) -> Option<CString> {
//         let mut full_path = [0 as c_char; MAX_PATH_CHARS];
//         unsafe {
//             if max_sys::path_toabsolutesystempath(
//                 self.vol,
//                 self.file_name.as_ptr(),
//                 full_path.as_mut_ptr()
//             ) == 0 {
//                 Some(CStr::from_ptr(full_path.as_ptr()).to_owned())
//             } else {
//                 None
//             }
//         }
//     }
// }
