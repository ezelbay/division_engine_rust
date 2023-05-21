use super::context::DivisionContext;
use super::settings::DivisionSettings;

extern "C" {
    pub fn division_engine_renderer_run_loop(
        ctx: *mut DivisionContext, settings: *const DivisionSettings);
}