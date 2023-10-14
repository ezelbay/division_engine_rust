use super::context::DivisionContext;

extern "C" {
    pub fn division_engine_renderer_run_loop(ctx: *mut DivisionContext);
}
