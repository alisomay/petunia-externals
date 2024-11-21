use core::ffi::c_void;
use median::max_sys::{t_atom, t_atom_long, t_symbol};
use median::object::MaxObj;
use median::wrapper::MaxObjWrapper;
use std::os::raw::c_long;
use std::sync::atomic::Ordering;

use median::method;
use median::wrapper::WrapperWrapped;

use super::RytmExternal;

impl RytmExternal {
    // Methods:

    pub extern "C" fn int_tramp(wrapper: &::median::wrapper::MaxObjWrapper<Self>, v: t_atom_long) {
        if let Err(err) = WrapperWrapped::wrapped(wrapper).int(v) {
            err.obj_post(wrapper.wrapped().max_obj());
        }
    }

    pub extern "C" fn anything_with_selector_tramp(
        wrapper: &MaxObjWrapper<Self>,
        sel: *mut t_symbol,
        ac: c_long,
        av: *const t_atom,
    ) {
        method::sel_list(sel, ac, av, |sym, atoms| {
            if let Err(err) = WrapperWrapped::wrapped(wrapper).anything_with_selector(&sym, atoms) {
                err.obj_post(wrapper.wrapped().max_obj());
            }
        });
    }

    // Attributes:

    // Trampoline for getting frequency
    #[allow(clippy::needless_pass_by_value)]
    pub extern "C" fn attr_get_sysex_id_tramp(
        wrapper: &MaxObjWrapper<Self>,
        _attr: c_void,
        ac: *mut c_long,
        av: *mut *mut t_atom,
    ) {
        median::attr::get(ac, av, || {
            WrapperWrapped::wrapped(wrapper)
                .target_device_id
                .load(Ordering::SeqCst)
        });
    }

    // Trampoline for setting frequency
    #[allow(clippy::needless_pass_by_value)]
    pub extern "C" fn attr_set_sysex_id_tramp(
        wrapper: &MaxObjWrapper<Self>,
        _attr: c_void,
        ac: c_long,
        av: *mut t_atom,
    ) {
        median::attr::set(ac, av, |val: isize| {
            let external = WrapperWrapped::wrapped(wrapper);
            external.target_device_id.store(val, Ordering::SeqCst);
            // Value is always valid because it is clamped.
            external.inner.project.lock().set_device_id(val as u8);
        });
    }
}
