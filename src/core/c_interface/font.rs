use std::ffi::c_char;

use super::context::DivisionContext;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DivisionFontGlyph {
    pub glyph_id: u32,
    pub width: u32,
    pub height: u32,
}

extern "C" {
    pub fn division_engine_font_alloc(
        ctx: *mut DivisionContext,
        font_file_path: *const c_char,
        font_height: u32,
        out_font_id: *mut u32,
    ) -> bool;

    pub fn division_engine_font_free(ctx: *mut DivisionContext, font_id: u32);

    pub fn division_engine_font_get_glyph(
        ctx: *mut DivisionContext, 
        font_id: u32, 
        character: i32, 
        out_glyph: *mut DivisionFontGlyph
    ) -> bool;

    pub fn division_engine_font_rasterize_glyph(
        ctx: *mut DivisionContext,
        font_id: u32,
        glyph: *const DivisionFontGlyph,
        bitmap: *mut u8
    ) -> bool;
}
