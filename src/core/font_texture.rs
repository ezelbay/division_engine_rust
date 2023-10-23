use division_math::Vector2;
use std::{alloc::Layout, path::Path};

use super::{context, Context, DivisionId, FontGlyph, TextureDescriptor, TextureFormat};

#[derive(Debug)]
pub enum Error {
    Context(context::Error),
    NoSpace,
}

#[derive(Clone, Copy)]
pub struct GlyphPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
struct FreeBlock {
    position: usize,
    width: usize,
}

pub struct FontTexture {
    glyphs: Vec<FontGlyph>,
    glyph_positions: Vec<GlyphPosition>,
    characters: Vec<char>,
    rows_free_space: Vec<FreeBlock>,
    pixel_buffer: *mut u8,
    rasterizer_buffer: *mut u8,
    rasterizer_buffer_capacity: usize,
    width: usize,
    height: usize,
    font_size: usize,
    font_id: DivisionId,
    texture_id: DivisionId,
    texture_was_changed: bool,
}

impl FontTexture {
    pub const DEFAULT_WIDTH: usize = 1024;
    pub const DEFAULT_HEIGHT: usize = 512;

    pub fn new(
        context: &mut Context,
        font_path: &Path,
        font_size: usize,
    ) -> Result<Self, Error> {
        Self::new_with_resolution(
            context,
            font_path,
            font_size,
            Self::DEFAULT_WIDTH,
            Self::DEFAULT_HEIGHT,
        )
    }

    pub fn new_with_resolution(
        context: &mut Context,
        font_path: &Path,
        font_size: usize,
        width: usize,
        height: usize,
    ) -> Result<Self, Error> {
        let font_id = context.create_font(&font_path, font_size as u32)?;

        let tex_desc = TextureDescriptor::new(width, height, TextureFormat::R8Uint);
        let texture_id = context.create_texture_buffer(&tex_desc)?;

        let pixel_buffer = unsafe {
            std::alloc::alloc_zeroed(Layout::from_size_align_unchecked(width * height, 1))
        };

        let row_count = height / font_size;
        let approx_char_count = row_count + (width / font_size);

        let mut rows_free_space = Vec::with_capacity(row_count);
        rows_free_space.resize(row_count, FreeBlock { position: 0, width });

        Ok(FontTexture {
            glyphs: Vec::with_capacity(approx_char_count),
            characters: Vec::with_capacity(approx_char_count),
            glyph_positions: Vec::with_capacity(approx_char_count),
            rasterizer_buffer: std::ptr::null_mut(),
            rasterizer_buffer_capacity: 0,
            pixel_buffer,
            rows_free_space,
            width,
            height,
            font_size,
            font_id,
            texture_id,
            texture_was_changed: false,
        })
    }

    #[inline]
    pub fn texture_id(&self) -> DivisionId {
        self.texture_id
    }

    #[inline]
    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width as f32, self.height as f32)
    }

    #[inline]
    pub fn bytes_len(&self) -> usize {
        self.width * self.height
    }

    pub fn find_glyph_layout(
        &self,
        glyph_char: char,
    ) -> Option<(&FontGlyph, &GlyphPosition)> {
        match self.characters.binary_search(&glyph_char) {
            Ok(i) => Some((&self.glyphs[i], &self.glyph_positions[i])),
            Err(_) => None,
        }
    }

    pub fn delete(&mut self, context: &mut Context) {
        context.delete_texture_buffer(self.texture_id);
        context.delete_font(self.font_id);
    }

    pub fn upload_texture(&mut self, context: &mut Context) {
        if !self.texture_was_changed {
            return;
        }

        unsafe {
            context.set_texture_buffer_data_ptr(self.texture_id, self.pixel_buffer);
        }
        self.texture_was_changed = false;
    }

    pub fn cache_character(
        &mut self,
        context: &mut Context,
        character: char,
    ) -> Result<(&FontGlyph, &GlyphPosition), Error> {
        match self.characters.binary_search(&character) {
            Ok(i) => Ok((&self.glyphs[i], &self.glyph_positions[i])),
            Err(i) => {
                self.layout_glyph(context, character, i)?;
                self.rasterize_glyph(context, i)?;

                Ok((&self.glyphs[i], &self.glyph_positions[i]))
            }
        }
    }

    fn layout_glyph(
        &mut self,
        context: &mut Context,
        character: char,
        index_to_place: usize,
    ) -> Result<(), Error> {
        const GLYPH_GAP: usize = 1;
        let glyph = {
            let mut glyph = context.get_font_glyph(self.font_id, character)?;
            if character == ' ' {
                glyph.width = glyph.advance_x;
            }
            glyph
        };
        let gapped_glyph_width = glyph.width as usize + GLYPH_GAP;
        for (row, free_block) in &mut self.rows_free_space.iter_mut().enumerate() {
            let free_after = free_block.width as isize - gapped_glyph_width as isize;

            if free_after < 0 {
                continue;
            }

            let position = GlyphPosition {
                x: free_block.position,
                y: self.font_size * row,
            };

            free_block.position += gapped_glyph_width;
            free_block.width = free_after as usize;

            self.glyphs.insert(index_to_place, glyph);
            self.glyph_positions.insert(index_to_place, position);
            self.characters.insert(index_to_place, character);

            return Ok(());
        }

        Err(Error::NoSpace)
    }

    fn rasterize_glyph(
        &mut self,
        context: &mut Context,
        glyph_index: usize,
    ) -> Result<(), Error> {
        let glyph = self.glyphs[glyph_index];
        let position = self.glyph_positions[glyph_index];
        let character = self.characters[glyph_index];

        let glyph_bytes = glyph.width as usize * glyph.height as usize;

        if self.rasterizer_buffer_capacity < glyph_bytes {
            unsafe {
                self.rasterizer_buffer = std::alloc::realloc(
                    self.rasterizer_buffer,
                    Layout::from_size_align_unchecked(self.rasterizer_buffer_capacity, 1),
                    glyph_bytes,
                )
            }
        }

        if character != ' ' {
            unsafe {
                context.rasterize_glyph_to_buffer(
                    self.font_id,
                    character,
                    self.rasterizer_buffer,
                )?;
            }
        } else {
            unsafe {
                self.rasterizer_buffer.write_bytes(0, glyph_bytes);
            }
        }

        for h in 0..glyph.height {
            let h = h as usize;
            let glyph_width = glyph.width as usize;

            let src_row_start = h * glyph_width;
            let dst_row_start = position.x + (position.y + h) * self.width;

            unsafe {
                let src = self.rasterizer_buffer.add(src_row_start);
                let dst = self.pixel_buffer.add(dst_row_start);
                dst.copy_from_nonoverlapping(src, glyph_width);
            }
        }

        self.texture_was_changed = true;

        Ok(())
    }
}

impl Drop for FontTexture {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.pixel_buffer,
                Layout::from_size_align_unchecked(self.bytes_len(), 1),
            );

            std::alloc::dealloc(
                self.rasterizer_buffer,
                Layout::from_size_align_unchecked(self.rasterizer_buffer_capacity, 1),
            );
        }
    }
}

impl From<context::Error> for Error {
    fn from(value: context::Error) -> Self {
        Error::Context(value)
    }
}
