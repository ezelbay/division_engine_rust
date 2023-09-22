use std::ffi::{c_char, c_void, CStr};

use super::{c_interface::{
        context::{division_engine_context_register_lifecycle, DivisionContext, division_engine_context_finalize},
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
    }, Context};

pub trait LifecycleManager: Sized {
    fn init(&mut self, context: &mut DivisionContext);
    fn update(&mut self, context: &mut DivisionContext);
    fn error(&mut self, context: &mut DivisionContext, error_code: i32, message: &str);
    fn cleanup(&mut self, context: &mut DivisionContext);
}

impl Context {
    pub(crate) fn register_lifecycle_manager<T: LifecycleManager>(&mut self, lifecycle_manager: &T) {
        self.user_data = lifecycle_manager as *const T as *const c_void;
        
        unsafe {
            division_engine_context_register_lifecycle(
                self,
                &DivisionLifecycle {
                    init_callback: init_callback::<T>,
                    update_callback: update_callback::<T>,
                    free_callback: free_callback::<T>,
                    error_callback: error_callback::<T>,
                },
            );
        }
    }

    pub fn run(&mut self) {
        unsafe {
            division_engine_renderer_run_loop(self);
        }
    }
}

unsafe extern "C" fn init_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.init(&mut *ctx);
}

unsafe extern "C" fn update_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.update(&mut *ctx);
}

unsafe extern "C" fn free_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.cleanup(&mut *ctx);
    division_engine_context_finalize(ctx);
}

unsafe extern "C" fn error_callback<T: LifecycleManager>(
    ctx: *mut DivisionContext,
    error_code: i32,
    message: *const c_char,
) {
    let owner = get_delegate_mut::<T>(&mut *ctx);
    owner.error(
        &mut *ctx,
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
