use super::settings::DivisionEngineErrorFunc;
use super::settings::DivisionSettings;
use super::state::DivisionState;
use std::ffi::c_void;

#[repr(C)]
pub struct DivisionContext {
    pub state: DivisionState,

    pub error_callback: DivisionEngineErrorFunc,
    pub render_context: *const c_void,
    pub shader_context: *const c_void,
    pub vertex_buffer_context: *const c_void,
    pub uniform_buffer_context: *const c_void,
    pub texture_context: *const c_void,
    pub render_pass_context: *const c_void,

    pub user_data: *const c_void,
}

extern "C" {
    pub fn division_engine_context_alloc(
        settings: *const DivisionSettings,
        out_context: *mut *mut DivisionContext,
    ) -> bool;

    pub fn division_engine_context_free(ctx: *mut DivisionContext);
}
