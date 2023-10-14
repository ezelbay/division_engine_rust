use super::core_state::CoreState;

pub trait LifecycleManager: Sized {
    fn init(&mut self, context: &mut CoreState);
    fn update(&mut self, context: &mut CoreState);
    fn error(&mut self, context: &mut CoreState, error_code: i32, message: &str);
    fn cleanup(&mut self, context: &mut CoreState);
}
