use crate::EngineContext;

use super::Context;

pub trait LifecycleManagerBuilder {
    type LifecycleManager: LifecycleManager + 'static;

    fn build(&mut self, context: &mut Context) -> Self::LifecycleManager;
}

pub trait LifecycleManager: Sized {
    type LifecycleState;

    fn create_state(&self, context: &mut Context) -> Self::LifecycleState;
    fn update(&mut self, state: &mut EngineContext<Self::LifecycleState>);
    fn error(
        &mut self,
        state: &mut EngineContext<Self::LifecycleState>,
        error_code: i32,
        message: &str,
    );
    fn cleanup(&mut self, state: &mut EngineContext<Self::LifecycleState>);
}
