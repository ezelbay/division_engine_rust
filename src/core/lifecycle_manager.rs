use crate::EngineState;

pub trait LifecycleManagerBuilder {
    type LifecycleManager: LifecycleManager + 'static;

    fn build(&mut self, state: &mut EngineState) -> Self::LifecycleManager;
}

pub trait LifecycleManager: Sized {
    fn update(&mut self, state: &mut EngineState);
    fn error(&mut self, state: &mut EngineState, error_code: i32, message: &str);
    fn cleanup(&mut self, state: &mut EngineState);
}
