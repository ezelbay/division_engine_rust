use std::{ffi::CString, ptr::null_mut};

use super::{c_interface::settings::DivisionSettings, Context, Error, LifecycleManager};

pub struct ContextBuilder {
    title: CString,
    settings: DivisionSettings,
}

impl ContextBuilder {
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

    pub fn build<T: LifecycleManager>(
        self,
        lifecycle_manager: &T,
    ) -> Result<Box<Context>, Error> {
        let mut context = Context::new(self.title, self.settings)?;
        context.register_lifecycle_manager(lifecycle_manager);

        Ok(context)
    }
}
