use median::max_sys::{t_atom, t_atom_long, t_symbol};
use median::object::MaxObj;
use median::wrapper::MaxObjWrapper;

use std::os::raw::c_long;

use median::method;
use median::wrapper::WrapperWrapped;

use super::RytmExternal;

impl RytmExternal {
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
}
