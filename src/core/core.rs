use std::{
    ffi::CString,
    ptr::null_mut,
};

pub type DivisionId = u32;

use super::{
    c_interface::{
        context::{
            division_engine_context_alloc, division_engine_context_free,
            DivisionContext,
        },
        renderer::division_engine_renderer_run_loop,
        settings::DivisionSettings,
    },
    core_builder::CoreBuilder,
};

pub struct Core {
    pub(crate) ctx: *mut DivisionContext,
    settings: DivisionSettings,
    window_title: CString,
}

#[derive(Debug)]
pub enum Error {
    Core(String),
    CInterface { error_code: i32, message: String },
}

impl Core {
    pub fn builder() -> CoreBuilder {
        CoreBuilder::new()
    }

    pub(crate) fn new(
        window_title: CString,
        settings: DivisionSettings,
    ) -> Result<Core, Error> {
        let mut core = Core {
            ctx: null_mut(),
            settings,
            window_title,
        };

        core.settings.window_title = core.window_title.as_ptr();

        unsafe {
            if !division_engine_context_alloc(&core.settings, &mut core.ctx) {
                return Err(Error::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }
        }

        Ok(core)
    }

    pub(crate) fn run(&self) {
        unsafe {
            division_engine_renderer_run_loop(self.ctx);
        }
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe {
            division_engine_context_free(self.ctx);
        }
    }
}
