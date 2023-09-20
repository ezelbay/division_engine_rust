use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::null_mut,
};

pub type DivisionId = u32;

use super::{
    c_interface::{
        context::{
            division_engine_context_alloc, division_engine_context_free,
            division_engine_context_register_lifecycle, DivisionContext,
        },
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
        settings::DivisionSettings,
    },
    core_builder::CoreBuilder,
    CoreDelegate,
};

pub struct Core {
    pub(crate) ctx: *mut DivisionContext,
    settings: DivisionSettings,
    window_title: CString,
    delegate: Box<dyn CoreDelegate>,
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
        delegate: Box<dyn CoreDelegate>,
    ) -> Result<Box<Core>, Error> {
        let mut core = Box::new(Core {
            ctx: null_mut(),
            settings,
            window_title,
            delegate,
        });

        core.settings.window_title = core.window_title.as_ptr();

        unsafe {
            if !division_engine_context_alloc(&core.settings, &mut core.ctx) {
                return Err(Error::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }

            division_engine_context_register_lifecycle(
                core.ctx,
                &DivisionLifecycle {
                    init_callback: Core::init_callback,
                    update_callback: Core::update_callback,
                    error_callback: Core::error_callback,
                },
            )
        }

        unsafe {
            (*core.ctx).user_data = &*core as *const Core as *const c_void;
        }

        Ok(core)
    }

    pub fn run(&self) {
        unsafe {
            division_engine_renderer_run_loop(self.ctx);
        }
    }

    pub(crate) unsafe extern "C" fn init_callback(ctx: *mut DivisionContext) {
        let core = (*ctx).user_data as *mut Core;
        (*core).delegate.init(&mut *core);
    }

    pub(crate) unsafe extern "C" fn update_callback(ctx: *mut DivisionContext) {
        let core = (*ctx).user_data as *mut Core;
        (*core).delegate.update(&mut *core);
    }

    pub(crate) unsafe extern "C" fn error_callback(
        _ctx: *mut DivisionContext,
        error_code: i32,
        message: *const c_char,
    ) {
        let msg = CStr::from_ptr(message);
        eprintln!(
            "Error code: {}, error_message: {}",
            error_code,
            msg.to_str().unwrap()
        );
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe {
            division_engine_context_free(self.ctx);
        }
    }
}
