use crate::EngineContext;

use super::Context;

pub trait LifecycleManagerBuilder {
    type LifecycleManager: LifecycleManager;

    fn build(
        &mut self,
        ffi_context: &mut Context,
    ) -> (
        Self::LifecycleManager,
        <Self::LifecycleManager as LifecycleManager>::LifecycleState,
    );
}

pub trait LifecycleManager: Sized {
    type LifecycleState;

    fn update(&mut self, state: &mut EngineContext<Self::LifecycleState>);

    fn error(
        &mut self,
        state: &mut EngineContext<Self::LifecycleState>,
        error_code: i32,
        message: &str,
    );

    fn cleanup(&mut self, state: &mut EngineContext<Self::LifecycleState>);
}
