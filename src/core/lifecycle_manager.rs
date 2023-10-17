use super::core_state::CoreState;

pub trait LifecycleManagerBuilder {
    type LifecycleManager: LifecycleManager + 'static;

    fn build(&mut self, state: &mut CoreState) -> Self::LifecycleManager;
}

pub trait LifecycleManager: Sized {
    fn update(&mut self, state: &mut CoreState);
    fn error(&mut self, state: &mut CoreState, error_code: i32, message: &str);
    fn cleanup(&mut self, state: &mut CoreState);
}
