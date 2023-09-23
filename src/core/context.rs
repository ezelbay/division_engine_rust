use std::ffi::CString;

pub type DivisionId = u32;

use division_math::{Vector2, Vector4};

use super::{
    c_interface::{
        context::{division_engine_context_initialize, DivisionContext, DivisionColor},
        settings::DivisionSettings,
    },
    context_builder::ContextBuilder,
};

pub type Context = DivisionContext;

#[derive(Debug)]
pub enum Error {
    Core(String),
    CInterface { error_code: i32, message: String },
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub(crate) fn new(
        window_title: CString,
        settings: DivisionSettings,
    ) -> Result<Box<Context>, Error> {
        let mut settings = settings;

        unsafe {
            let ctx =
                std::alloc::alloc(std::alloc::Layout::new::<Context>()) as *mut Context;

            settings.window_title = window_title.as_ptr();
            if !division_engine_context_initialize(&settings, ctx) {
                return Err(Error::Core(String::from(
                    "Failed to create new division engine context",
                )));
            }

            Ok(Box::from_raw(ctx))
        }
    }

    #[inline]
    pub fn get_window_size(&self) -> Vector2 {
        let ctx = self.render_context;
        unsafe {
            Vector2::new(
                (*ctx).frame_buffer_width as f32,
                (*ctx).frame_buffer_height as f32,
            )
        }
    }

    #[inline]
    pub fn get_clear_color(&self) -> Vector4 {
        let ctx = self.render_context;
        unsafe {
            (*ctx).clear_color.into()
        }
    }

    #[inline]
    pub fn set_clear_color(&mut self, color: Vector4) {
        let ctx = self.render_context;
        unsafe {
            (*ctx).clear_color = color.into();
        }
    }
}

impl From<DivisionColor> for Vector4 {
    fn from(value: DivisionColor) -> Self {
        unsafe {
            std::mem::transmute(value)
        }
    }
}

impl From<Vector4> for DivisionColor {
    fn from(value: Vector4) -> Self {
        unsafe {
            std::mem::transmute(value)
        }
    }   
}
