use std::ffi::c_char;

use super::context::DivisionContext;

pub type DivisionErrorFunc = unsafe extern "C" fn(*mut DivisionContext, i32, *const c_char);
pub type DivisionLifecycleFunc = unsafe extern "C" fn(*mut DivisionContext);

#[repr(C)]
pub struct DivisionLifecycle {
    pub init_callback: DivisionLifecycleFunc,
    pub update_callback: DivisionLifecycleFunc,
    pub error_callback: DivisionErrorFunc,
}