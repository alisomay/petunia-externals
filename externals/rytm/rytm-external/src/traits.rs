#![allow(clippy::not_unsafe_ptr_arg_deref)]

use median::{max_sys, outlet::SendValue, symbol::SymbolRef};
use std::ffi::CString;
use tracing::warn;

/// For flushing data from an outlet serially.
pub trait SerialSend {
    #[allow(clippy::borrowed_box)]
    fn serial_send_int(&self, out: &Box<dyn SendValue<isize> + Sync>);
}

impl SerialSend for Vec<u8> {
    fn serial_send_int(&self, outlet: &Box<dyn SendValue<isize> + Sync>) {
        for byte in self {
            outlet
                .send(*byte as isize)
                .inspect_err(|_| {
                    median::error!("Error sending to status outlet due to stack overflow.");
                    warn!("Error sending to status outlet due to stack overflow.");
                })
                .ok();
        }
    }
}

// Post trait for posting to the max console.
pub trait Post {
    fn obj_post(&self, obj: *mut max_sys::t_object);
    fn obj_error(&self, obj: *mut max_sys::t_object);
    fn obj_warn(&self, obj: *mut max_sys::t_object);
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

    fn obj_warn(&self, obj: *mut max_sys::t_object) {
        if let Ok(message) = CString::new(*self) {
            unsafe { max_sys::object_warn(obj, message.as_ptr()) };
        }
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

    fn obj_warn(&self, obj: *mut max_sys::t_object) {
        if let Ok(message) = CString::new(self.as_bytes()) {
            unsafe { max_sys::object_warn(obj, message.as_ptr()) };
        }
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

    fn obj_warn(&self, obj: *mut max_sys::t_object) {
        if let Ok(message) = CString::new(self.as_bytes()) {
            unsafe { max_sys::object_warn(obj, message.as_ptr()) };
        }
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

    fn obj_warn(&self, obj: *mut max_sys::t_object) {
        if let Ok(s) = self.to_string() {
            if let Ok(message) = CString::new(s) {
                unsafe { max_sys::object_warn(obj, message.as_ptr()) };
            }
        }
    }

    fn post(&self) {
        median::post(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }

    fn error(&self) {
        median::error(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }
}
