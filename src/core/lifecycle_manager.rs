use std::ffi::{c_char, c_void, CStr};

use super::{c_interface::{
        context::{division_engine_context_register_lifecycle, DivisionContext},
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
    }, Context};

pub trait LifecycleManager: Sized {
    fn init(&mut self);
    fn update(&mut self);
    fn error(&mut self, error_code: i32, message: &str);

    fn context_mut(&mut self) -> &mut Context;
    
    fn run(&mut self) {
        self.context_mut().user_data = self as *const Self as *const c_void;

        unsafe {
            division_engine_context_register_lifecycle(
                self.context_mut(),
                &DivisionLifecycle {
                    init_callback: init_callback::<Self>,
                    update_callback: update_callback::<Self>,
                    error_callback: error_callback::<Self>,
                },
            );

            division_engine_renderer_run_loop(
                self.context_mut(),
            );
        }
    }
}

unsafe extern "C" fn init_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.init();
}

unsafe extern "C" fn update_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.update();
}

unsafe extern "C" fn error_callback<T: LifecycleManager>(
    ctx: *mut DivisionContext,
    error_code: i32,
    message: *const c_char,
) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.error(
        error_code,
        CStr::from_ptr(message)
            .to_str()
            .expect("Failed to read error message"),
    );
}

#[inline(always)]
fn get_delegate_mut<T: LifecycleManager>(ctx: &mut DivisionContext) -> &mut T {
    unsafe { &mut *(ctx.user_data as *mut T) }
}
