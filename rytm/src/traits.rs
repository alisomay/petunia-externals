use median::{
    atom::{Atom, AtomValue},
    max_sys,
    outlet::SendValue,
    symbol::SymbolRef,
};

/// For flushing data from an outlet serially.
pub trait SerialSend {
    #[allow(clippy::borrowed_box)]
    fn serial_send_int(&self, out: &Box<dyn SendValue<isize> + Sync>);
}

impl SerialSend for Vec<u8> {
    fn serial_send_int(&self, outlet: &Box<dyn SendValue<isize> + Sync>) {
        for byte in self {
            // This is what I could think of :) Since it needs a long :).
            outlet
                .send(max_sys::t_atom_long::from(*byte))
                .inspect_err(|_| median::error!("Error sending to outlet due to stack overflow."))
                .ok();
        }
    }
}

// Post trait for posting to the max console.
pub trait Post {
    fn obj_post(&self, obj: *mut max_sys::t_object);
    fn obj_error(&self, obj: *mut max_sys::t_object);
    fn post(&self);
    fn error(&self);
}

impl Post for &str {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for &String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for SymbolRef {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn post(&self) {
        median::post(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }

    fn error(&self) {
        median::error(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }
}

use crate::RYTM_EXTERNAL_DEBUG;

// TODO: Revise debugging with right files maybe in a macro.
// This is a WIP for now.
pub trait DebugPost {
    fn dbg_post(&self);
}

impl DebugPost for &str {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for String {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for &String {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for SymbolRef {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for &SymbolRef {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for AtomValue {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                match self {
                    Self::Float(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Float: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    Self::Int(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Int: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    Self::Symbol(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Symbol: {}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    Self::Object(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Object Pointer: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                }
            }
        }
    }
}

impl DebugPost for Atom {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                self.get_value().map_or_else(
                    || {
                        median::post!("Debug Atom [{}:{}]: None", std::file!(), std::line!());
                    },
                    |value| value.dbg_post(),
                );
            }
        }
    }
}

impl DebugPost for &Atom {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                self.get_value().map_or_else(
                    || {
                        median::post!("Debug Atom [{}:{}]: None", std::file!(), std::line!());
                    },
                    |value| value.dbg_post(),
                );
            }
        }
    }
}

impl DebugPost for &[Atom] {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                if self.is_empty() {
                    median::post!("Debug Atoms [{}:{}]: []", std::file!(), std::line!());
                } else {
                    median::post!(
                        "Debug List of Atoms Start [{}:{}]",
                        std::file!(),
                        std::line!()
                    );
                    for atom in *self {
                        atom.dbg_post();
                    }
                    median::post!("Debug List of Atoms End");
                }
            }
        }
    }
}
