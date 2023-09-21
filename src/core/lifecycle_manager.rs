use std::ffi::{c_char, c_void, CStr};

use super::{
    c_interface::{
        context::{division_engine_context_register_lifecycle, DivisionContext},
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
    },
    Context, PinnedContext,
};

pub trait LifecycleManager: Sized {
    fn init(&mut self);
    fn update(&mut self);
    fn error(&mut self, error_code: i32, message: &str);

    fn pinned_context_mut(&mut self) -> &mut PinnedContext;

    unsafe fn context_mut(&mut self) -> &mut Context {
        self.pinned_context_mut().as_mut().get_unchecked_mut()
    }

    unsafe fn c_context_ptr_mut(&mut self) -> *mut DivisionContext {
        &mut self
            .pinned_context_mut()
            .as_mut()
            .get_unchecked_mut()
            .c_context
    }

    fn run(&mut self) {
        unsafe {
            let c_context = &mut *self.c_context_ptr_mut();

            c_context.user_data = &*self as *const Self as *mut c_void;

            division_engine_context_register_lifecycle(
                c_context,
                &DivisionLifecycle {
                    init_callback: init_callback::<Self>,
                    update_callback: update_callback::<Self>,
                    error_callback: error_callback::<Self>,
                },
            );

            division_engine_renderer_run_loop(c_context);
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
