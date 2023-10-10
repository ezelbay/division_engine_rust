use division_math::Vector2;
use std::path::Path;

use super::{Context, DivisionId, FontGlyph, TextureDescriptor, TextureFormat};

#[derive(Debug)]
pub enum Error {
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
    pixel_buffer: Vec<u8>,
    rasterizer_buffer: Vec<u8>,
    rows_free_space: Vec<FreeBlock>,
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

    pub fn new(context: &mut Context, font_path: &Path, font_size: usize) -> Self {
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
    ) -> Self {
        let font_id = context.create_font(&font_path, font_size as u32).unwrap();

        let texture_id = context
            .create_texture_buffer(&TextureDescriptor::new(
                width,
                height,
                TextureFormat::R8Uint,
            ))
            .unwrap();

        let mut pixel_buffer = Vec::with_capacity(width * height);
        unsafe {
            pixel_buffer.set_len(pixel_buffer.capacity());
        }

        let row_count = height / font_size;
        let mut rows_free_space = Vec::with_capacity(row_count);
        rows_free_space.resize(row_count, FreeBlock { position: 0, width });

        let font_texture = FontTexture {
            glyphs: Vec::new(),
            characters: Vec::new(),
            rasterizer_buffer: Vec::new(),
            glyph_positions: Vec::new(),
            rows_free_space,
            pixel_buffer,
            width,
            height,
            font_size,
            font_id,
            texture_id,
            texture_was_changed: false,
        };

        font_texture
    }

    #[inline]
    pub fn texture_id(&self) -> DivisionId {
        self.texture_id
    }

    #[inline]
    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width as f32, self.height as f32)
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

        context.set_texture_buffer_data(self.texture_id, &self.pixel_buffer);
        self.texture_was_changed = false;
    }

    pub fn cache_character(
        &mut self,
        context: &mut Context,
        character: char,
    ) -> Result<(), Error> {
        match self.characters.binary_search(&character) {
            Ok(_) => Ok(()),
            Err(i) => {
                let (glyph, position) = self.layout_glyph(context, character, i)?;
                self.rasterize_glyph(context, character, glyph, position);

                Ok(())
            }
        }
    }

    fn layout_glyph(
        &mut self,
        context: &mut Context,
        character: char,
        index_to_place: usize,
    ) -> Result<(FontGlyph, GlyphPosition), Error> {
        const GLYPH_GAP: usize = 1;
        let glyph = context.get_font_glyph(self.font_id, character).unwrap();
        let glyph_width = glyph.width as usize;

        for (row, free_block) in &mut self.rows_free_space.iter_mut().enumerate() {
            let free_after =
                free_block.width as isize - (glyph_width + GLYPH_GAP) as isize;

            if free_after < 0 {
                continue;
            }

            let position = GlyphPosition {
                x: free_block.position,
                y: self.font_size * row,
            };

            free_block.position += glyph_width + GLYPH_GAP;
            free_block.width = free_after as usize;

            self.glyphs.insert(index_to_place, glyph);
            self.glyph_positions.insert(index_to_place, position);
            self.characters.insert(index_to_place, character);

            return Ok((glyph, position));
        }

        return Err(Error::NoSpace);
    }

    fn rasterize_glyph(
        &mut self,
        context: &mut Context,
        character: char,
        glyph: FontGlyph,
        position: GlyphPosition,
    ) {
        let glyph_bytes = glyph.width as usize * glyph.height as usize;

        self.rasterizer_buffer.reserve(glyph_bytes);
        unsafe { self.rasterizer_buffer.set_len(glyph_bytes) }

        unsafe {
            context
                .rasterize_glyph_to_buffer(
                    self.font_id,
                    character,
                    &mut self.rasterizer_buffer,
                )
                .unwrap();
        }

        for h in 0..glyph.height {
            let h = h as usize;
            let glyph_width = glyph.width as usize;

            let src_row_start = h * glyph_width;
            let src_row_end = src_row_start + glyph_width;
            let src = &self.rasterizer_buffer[src_row_start..src_row_end];

            let dst_row_start = position.x + (position.y + h) * self.width;
            let dst_row_end = dst_row_start + glyph_width;
            let dst = &mut self.pixel_buffer[dst_row_start..dst_row_end];

            dst.copy_from_slice(src);
        }

        self.texture_was_changed = true;
    }
}
