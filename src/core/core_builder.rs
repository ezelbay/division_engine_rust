use std::{ffi::CString, ptr::null_mut};

use super::{c_interface::settings::DivisionSettings, Core, DivisionError, CoreDelegate};

pub struct CoreBuilder {
    _title: CString,
    _settings: DivisionSettings,
}

impl CoreBuilder {
    pub fn new() -> Self {
        let builder = Self {
            _title: CString::new("New window").unwrap(),
            _settings: DivisionSettings {
                window_width: 512,
                window_height: 512,
                window_title: null_mut(),
                error_callback: Core::error_callback,
                init_callback: Core::init_callback,
                update_callback: Core::update_callback,
            },
        };
        builder
    }

    pub fn window_size(mut self, width: usize, height: usize) -> Self {
        self._settings.window_width = width as u32;
        self._settings.window_height = height as u32;
        self
    }

    pub fn window_title(mut self, title: &str) -> Self {
        self._title = CString::new(title).unwrap();
        self._settings.window_title = self._title.as_ptr();
        self
    }

    pub fn build(self, delegate: Box<dyn CoreDelegate>) -> Result<Box<Core>, DivisionError> {
        Core::new(self._title, self._settings, delegate)
    }
}