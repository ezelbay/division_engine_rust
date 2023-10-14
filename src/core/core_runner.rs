use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::null_mut,
};

use super::{
    ffi::{
        context::{
            division_engine_context_finalize, division_engine_context_register_lifecycle,
            DivisionContext,
        },
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
        settings::DivisionSettings,
    },
    context::Context,
    context::Error,
    core_state::CoreState,
    LifecycleManager,
};

pub struct CoreRunner {
    title: CString,
    settings: DivisionSettings,
}

struct ContextBridgeData<T: LifecycleManager> {
    pub core_state: CoreState,
    pub lifecycle_manager: T,
}

impl CoreRunner {
    pub fn new() -> Self {
        let builder = Self {
            title: CString::new("New window").unwrap(),
            settings: DivisionSettings {
                window_width: 512,
                window_height: 512,
                window_title: null_mut(),
            },
        };
        builder
    }

    pub fn window_size(mut self, width: usize, height: usize) -> Self {
        self.settings.window_width = width as u32;
        self.settings.window_height = height as u32;
        self
    }

    pub fn window_title(mut self, title: &str) -> Self {
        self.title = CString::new(title).unwrap();
        self.settings.window_title = self.title.as_ptr();
        self
    }

    pub fn run<TManager: LifecycleManager>(
        self,
        lifecycle_manager: TManager,
    ) -> Result<(), Error> {
        let context = Context::new(self.title, self.settings)?;
        let core_state = CoreState { context };

        run(core_state, lifecycle_manager);

        Ok(())
    }
}

fn run<T: LifecycleManager>(core_state: CoreState, lifecycle_manager: T) {
    unsafe {
        let mut context_data = ContextBridgeData {
            core_state,
            lifecycle_manager,
        };

        context_data.core_state.context.user_data =
            &context_data as *const ContextBridgeData<T> as *const c_void;

        division_engine_context_register_lifecycle(
            context_data.core_state.context.as_mut(),
            &DivisionLifecycle {
                init_callback: init_callback::<T>,
                update_callback: update_callback::<T>,
                free_callback: free_callback::<T>,
                error_callback: error_callback::<T>,
            },
        );

        division_engine_renderer_run_loop(context_data.core_state.context.as_mut());
    }
}

unsafe extern "C" fn init_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<ContextBridgeData<T>>(&mut *ctx);
    owner.lifecycle_manager.init(&mut owner.core_state);
}

unsafe extern "C" fn update_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<ContextBridgeData<T>>(&mut *ctx);
    owner.lifecycle_manager.update(&mut owner.core_state);
}

unsafe extern "C" fn free_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<ContextBridgeData<T>>(&mut *ctx);
    owner.lifecycle_manager.cleanup(&mut owner.core_state);
    division_engine_context_finalize(ctx);
}

unsafe extern "C" fn error_callback<T: LifecycleManager>(
    ctx: *mut DivisionContext,
    error_code: i32,
    message: *const c_char,
) {
    let user_data = get_delegate_mut::<ContextBridgeData<T>>(&mut *ctx);
    user_data.lifecycle_manager.error(
        &mut user_data.core_state,
        error_code,
        CStr::from_ptr(message)
            .to_str()
            .expect("Failed to read error message"),
    );
}

#[inline(always)]
fn get_delegate_mut<T>(ctx: &mut DivisionContext) -> &mut T {
    unsafe { &mut *(ctx.user_data as *mut T) }
}
