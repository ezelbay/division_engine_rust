use std::ffi::{c_char, c_void, CStr};

use super::{
    c_interface::{
        context::{division_engine_context_register_lifecycle, DivisionContext},
        lifecycle::DivisionLifecycle,
    },
    Core,
};

pub trait CoreDelegate: Sized {
    fn init(&mut self);
    fn update(&mut self);
    fn error(&mut self, error_code: i32, message: &str);

    fn core(&self) -> &Core;
    fn core_mut(&mut self) -> &mut Core;

    fn run(&mut self) {
        unsafe {
            let ctx = {
                let core = self.core_mut();
                &mut *(core.ctx)
            };

            ctx.user_data = self as *mut Self as *mut c_void;
            division_engine_context_register_lifecycle(
                ctx,
                &DivisionLifecycle {
                    init_callback: Self::init_callback,
                    update_callback: Self::update_callback,
                    error_callback: Self::error_callback,
                },
            )
        }

        let core = self.core();
        core.run();
    }

    unsafe extern "C" fn init_callback(ctx: *mut DivisionContext) {
        let delegate = Self::get_delegate_mut(&*ctx);
        delegate.init();
    }

    unsafe extern "C" fn update_callback(ctx: *mut DivisionContext) {
        let delegate = Self::get_delegate_mut(&*ctx);
        delegate.update();
    }

    unsafe extern "C" fn error_callback(
        ctx: *mut DivisionContext,
        error_code: i32,
        message: *const c_char,
    ) {
        let delegate = Self::get_delegate_mut(&*ctx);
        delegate.error(
            error_code,
            CStr::from_ptr(message)
                .to_str()
                .expect("Failed to read error message"),
        );
    }

    #[inline(always)]
    fn get_delegate_mut<'a>(ctx: &'a DivisionContext) -> &'a mut Self {
        unsafe { &mut *((*ctx).user_data as *mut Self) }
    }
}