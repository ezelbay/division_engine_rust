use super::Context;

pub trait LifecycleManagerBuilder {
    type LifecycleManager: LifecycleManager + 'static;

    fn build(&mut self, context: &mut Context) -> Self::LifecycleManager;
}

pub trait LifecycleManager: Sized {
    fn update(&mut self, context: &mut Context);
    fn error(&mut self, context: &mut Context, error_code: i32, message: &str);
    fn cleanup(&mut self, context: &mut Context);
}
