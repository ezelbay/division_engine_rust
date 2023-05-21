use std::ffi::c_void;
use crate::division_engine::core_interface::settings::DivisionEngineErrorFunc;
use super::settings::DivisionSettings;
use super::state::DivisionState;

#[repr(C)]
pub struct DivisionContext {
    state: DivisionState,

    error_callback: DivisionEngineErrorFunc,
    render_context: *const c_void,
    shader_context: *const c_void,
    vertex_buffer_context: *const c_void,
    uniform_buffer_context: *const c_void,
    texture_context: *const c_void,
    render_pass_context: *const c_void,

    user_data: *const c_void
}

extern "C" {
    pub fn division_engine_context_alloc(
        settings: *const DivisionSettings, out_context: *mut *mut DivisionContext);

    pub fn division_engine_context_free(ctx: *mut DivisionContext);
}