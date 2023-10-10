use std::{
    ffi::CString,
    mem::MaybeUninit,
    path::Path,
};

use super::{
    c_interface::font::{
        division_engine_font_alloc, division_engine_font_free,
        division_engine_font_get_glyph, division_engine_font_rasterize_glyph,
    },
    Context, DivisionId, context::Error,
};

pub use super::c_interface::font::DivisionFontGlyph as FontGlyph;

impl Context {
    pub fn create_font(
        &mut self,
        font_file_path: &Path,
        font_height: u32,
    ) -> Result<DivisionId, Error> {
        let path = font_file_path.to_str();
        let path = match path {
            Some(p) => p,
            None => {
                return Err(Error::Core("Failed to get a font file path".to_string()))
            }
        };

        let path = CString::new(path).unwrap();

        let mut font_id = MaybeUninit::uninit();
        let ok = unsafe {
            division_engine_font_alloc(
                self,
                path.as_ptr(),
                font_height,
                font_id.as_mut_ptr(),
            )
        };

        match ok {
            true => unsafe { Ok(font_id.assume_init()) },
            false => Err(Error::Core("Failed to create a font".to_string())),
        }
    }

    pub fn get_font_glyph(
        &mut self,
        font_id: DivisionId,
        glyph_char: char,
    ) -> Result<FontGlyph, Error> {
        let mut glyph = MaybeUninit::uninit();
        let ok = unsafe {
            let glyph_char: u32 = glyph_char.into();
            division_engine_font_get_glyph(
                self,
                font_id,
                glyph_char as i32,
                glyph.as_mut_ptr(),
            )
        };

        match ok {
            true => unsafe { Ok(glyph.assume_init()) },
            false => Err(Error::Core("Failed to get a glyph".to_string())),
        }
    }

    pub unsafe fn rasterize_glyph_to_buffer(
        &mut self,
        font_id: DivisionId,
        glyph_char: char,
        buffer: *mut u8,
    ) -> Result<(), Error> {
        let ok = unsafe {
            division_engine_font_rasterize_glyph(
                self,
                font_id,
                glyph_char as i32,
                buffer,
            )
        };

        match ok {
            true => Ok(()),
            false => Err(Error::Core("Failed to rasterize glyph".to_string())),
        }
    }

    pub fn delete_font(&mut self, font_id: DivisionId) {
        unsafe { division_engine_font_free(self, font_id) }
    }
}
