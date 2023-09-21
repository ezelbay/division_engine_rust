use std::{ffi::CString, marker::PhantomPinned, pin::Pin};

pub type DivisionId = u32;

use super::{
    c_interface::{
        context::{
            division_engine_context_finalize, division_engine_context_initialize,
            DivisionContext,
        },
        settings::DivisionSettings,
    },
    context_builder::ContextBuilder,
};

pub struct Context {
    pub(crate) c_context: DivisionContext,
    _pin: PhantomPinned,
}

pub type PinnedContext = Pin<Box<Context>>;

#[derive(Debug)]
pub enum Error {
    Core(String),
    CInterface { error_code: i32, message: String },
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub(crate) fn new(
        window_title: CString,
        settings: DivisionSettings,
    ) -> Result<PinnedContext, Error> {
        let mut settings = settings;

        unsafe {
            let ctx =
                std::alloc::alloc(std::alloc::Layout::new::<Context>()) as *mut Context;
            let mut ctx = Box::into_pin(Box::from_raw(ctx));

            settings.window_title = window_title.as_ptr();
            if !division_engine_context_initialize(
                &settings,
                ctx.c_context_ptr_mut(),
            ) {
                return Err(Error::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }

            Ok(ctx)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            division_engine_context_finalize(&mut self.c_context);
        }
    }
}

pub trait PinnedContextGetter {
    unsafe fn context_mut(&mut self) -> &mut Context;
    unsafe fn c_context_ptr_mut(&mut self) -> *mut DivisionContext;
    unsafe fn c_context_ref_mut(&mut self) -> &mut DivisionContext;
}

impl PinnedContextGetter for PinnedContext {
    unsafe fn context_mut(&mut self) -> &mut Context {
        self.as_mut().get_unchecked_mut()
    }

    unsafe fn c_context_ptr_mut(&mut self) -> *mut DivisionContext {
        &mut self.context_mut().c_context as *mut DivisionContext
    }

    unsafe fn c_context_ref_mut(&mut self) -> &mut DivisionContext {
        &mut self.context_mut().c_context
    }
}
