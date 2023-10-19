use crate::core::Context;

pub struct EngineContext<T> {
    pub ffi_context: Box<Context>,
    pub state: T,
}