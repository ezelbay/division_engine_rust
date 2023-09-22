use std::ffi::CString;

pub type DivisionId = u32;

use super::{
    c_interface::{
        context::{
            division_engine_context_initialize,
            DivisionContext,
        },
        settings::DivisionSettings,
    },
    context_builder::ContextBuilder,
};

pub type Context = DivisionContext;

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
    ) -> Result<Box<Context>, Error> {
        let mut settings = settings;

        unsafe {
            let ctx =
                std::alloc::alloc(std::alloc::Layout::new::<Context>()) as *mut Context;

            settings.window_title = window_title.as_ptr();
            if !division_engine_context_initialize(
                &settings,
                ctx,
            ) {
                return Err(Error::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }

            Ok(Box::from_raw(ctx))
        }
    }
}