use std::{
    ffi::{c_char, c_void, CStr, CString},
    mem::ManuallyDrop,
    ptr::null_mut,
};

use super::{
    context::Context,
    context::Error,
    core_state::CoreState,
    ffi::{
        context::{
            division_engine_context_finalize, division_engine_context_register_lifecycle,
            DivisionContext,
        },
        lifecycle::DivisionLifecycle,
        renderer::division_engine_renderer_run_loop,
        settings::DivisionSettings,
    },
    LifecycleManager, LifecycleManagerBuilder,
};

pub struct CoreRunner {
    title: CString,
    settings: DivisionSettings,
}

struct ContextPreInitBridgeData<T: LifecycleManagerBuilder> {
    pub lifecycle_manager_builder: T,
}

struct ContextPostInitBridgeData<T: LifecycleManager> {
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

    pub fn run<TManager: LifecycleManagerBuilder>(
        self,
        lifecycle_manager_builder: TManager,
    ) -> Result<(), Error> {
        let context = Context::new(self.title, self.settings)?;
        run(context, lifecycle_manager_builder);

        Ok(())
    }
}

fn run<T: LifecycleManagerBuilder>(
    context_ptr: *mut Context,
    lifecycle_manager_builder: T,
) {
    unsafe {
        let preinit_data = ManuallyDrop::new(Box::new(ContextPreInitBridgeData {
            lifecycle_manager_builder,
        }));

        (*context_ptr).user_data =
            preinit_data.as_ref() as *const ContextPreInitBridgeData<T> as *const c_void;

        division_engine_context_register_lifecycle(
            context_ptr,
            &DivisionLifecycle {
                init_callback: init_callback::<T>,
                update_callback: update_callback::<T::LifecycleManager>,
                free_callback: free_callback::<T::LifecycleManager>,
                error_callback: error_callback::<T::LifecycleManager>,
            },
        );

        division_engine_renderer_run_loop(context_ptr);
    }
}

unsafe extern "C" fn init_callback<T: LifecycleManagerBuilder>(
    ctx_ptr: *mut DivisionContext,
) {
    let mut ctx = ManuallyDrop::new(Box::from_raw(ctx_ptr));
    let mut core_state = CoreState {
        context: ManuallyDrop::new(Box::from_raw(ctx_ptr))
    };

    let mut pre_init = Box::from_raw(ctx.user_data as *mut ContextPreInitBridgeData<T>);
    let mut lifecycle_manager = pre_init.lifecycle_manager_builder.build(&mut core_state);
    lifecycle_manager.init(&mut core_state);

    let post_init_data_ptr = ManuallyDrop::new(Box::new(ContextPostInitBridgeData {
        core_state,
        lifecycle_manager
    }));

    ctx.user_data = post_init_data_ptr.as_ref()
        as *const ContextPostInitBridgeData<T::LifecycleManager>
        as *const c_void;
}

unsafe extern "C" fn update_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let owner = get_delegate_mut::<ContextPostInitBridgeData<T>>(&mut *ctx);
    owner.lifecycle_manager.update(&mut owner.core_state);
}

unsafe extern "C" fn free_callback<T: LifecycleManager>(ctx: *mut DivisionContext) {
    let mut ctx = Box::from_raw(ctx);
    let mut owner = Box::from_raw(ctx.user_data as *mut ContextPostInitBridgeData<T>);
    
    owner.lifecycle_manager.cleanup(&mut owner.core_state);
    division_engine_context_finalize(ctx.as_mut() as *mut DivisionContext);
}

unsafe extern "C" fn error_callback<T: LifecycleManager>(
    ctx: *mut DivisionContext,
    error_code: i32,
    message: *const c_char,
) {
    let user_data = get_delegate_mut::<ContextPostInitBridgeData<T>>(&mut *ctx);
    user_data.lifecycle_manager.error(
        &mut user_data.core_state,
        error_code,
        CStr::from_ptr(message)
            .to_str()
            .expect("Failed to read error message"),
    );
}

#[inline(always)]
fn get_delegate_mut<'a, 'b, T>(ctx: &'a mut DivisionContext) -> &'b mut T {
    unsafe { &mut *(ctx.user_data as *mut T) }
}

impl<T: LifecycleManager> Drop for ContextPostInitBridgeData<T> {
    fn drop(&mut self) {
        println!("Post data was dropped");
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Context was dropped")
    }
}