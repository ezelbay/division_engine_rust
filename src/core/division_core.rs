use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::null_mut,
};

pub type DivisionId = u32;

use super::{
    division_core_builder::DivisionCoreBuilder,
    interface::{
        context::{division_engine_context_alloc, division_engine_context_free, DivisionContext},
        renderer::division_engine_renderer_run_loop,
        settings::DivisionSettings,
    },
    DivisionCoreDelegate,
};

pub struct DivisionCore {
    pub(crate) ctx: *mut DivisionContext,
    settings: DivisionSettings,
    window_title: CString,
    delegate: Box<dyn DivisionCoreDelegate>,
}

#[derive(Debug)]
pub enum DivisionError {
    Core(String),
    Internal { error_code: i32, message: String },
}

impl DivisionCore {
    pub fn builder() -> DivisionCoreBuilder {
        DivisionCoreBuilder::new()
    }

    pub(crate) fn new(
        window_title: CString,
        settings: DivisionSettings,
        delegate: Box<dyn DivisionCoreDelegate>,
    ) -> Result<Box<DivisionCore>, DivisionError> {
        let mut core = Box::new(DivisionCore {
            ctx: null_mut(),
            settings,
            window_title,
            delegate,
        });

        core.settings.window_title = core.window_title.as_ptr();

        unsafe {
            if !division_engine_context_alloc(&core.settings, &mut core.ctx) {
                return Err(DivisionError::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }
        }

        unsafe {
            (*core.ctx).user_data = &*core as *const DivisionCore as *const c_void;
        }

        Ok(core)
    }

    pub fn run(&self) {
        unsafe {
            division_engine_renderer_run_loop(self.ctx, &self.settings);
        }
    }

    pub(crate) unsafe extern "C" fn init_callback(ctx: *mut DivisionContext) {
        let core = (*ctx).user_data as *mut DivisionCore;
        (*core).delegate.init(&mut *core);
    }

    pub(crate) unsafe extern "C" fn update_callback(ctx: *mut DivisionContext) {
        let core = (*ctx).user_data as *mut DivisionCore;
        (*core).delegate.update(&mut *core);
    }

    pub(crate) unsafe extern "C" fn error_callback(error_code: i32, message: *const c_char) {
        let msg = CStr::from_ptr(message);
        eprintln!(
            "Error code: {}, error_message: {}",
            error_code,
            msg.to_str().unwrap()
        );
    }
}

impl Drop for DivisionCore {
    fn drop(&mut self) {
        unsafe {
            division_engine_context_free(self.ctx);
        }
    }
}
