use division_math::Vector2;
use std::path::Path;

use super::{Context, DivisionId, FontGlyph, TextureDescriptor, TextureFormat};

#[derive(Clone, Copy)]
pub struct GlyphLayout {
    pub x: usize,
    pub y: usize,
    pub u: f32,
    pub v: f32,
    pub glyph: FontGlyph,
}

pub struct FontTexture {
    glyph_layouts: Vec<GlyphLayout>,
    characters: Vec<char>,
    pixel_buffer: Vec<u8>,
    rasterizer_buffer: Vec<u8>,
    width: usize,
    height: usize,
    font_size: usize,
    font_id: DivisionId,
    texture_id: DivisionId,
}

impl FontTexture {
    pub const DEFAULT_WIDTH: usize = 1024;
    pub const DEFAULT_HEIGHT: usize = 512;

    pub fn new<T: Iterator<Item = char>>(
        context: &mut Context,
        font_path: &Path,
        font_size: usize,
        characters: T,
    ) -> Self {
        Self::new_with_resolution(
            context,
            font_path,
            font_size,
            characters,
            Self::DEFAULT_WIDTH,
            Self::DEFAULT_HEIGHT,
        )
    }

    pub fn new_with_resolution<T: Iterator<Item = char>>(
        context: &mut Context,
        font_path: &Path,
        font_size: usize,
        characters: T,
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

        let pixel_buffer = unsafe {
            let mut v = Vec::with_capacity(width * height);
            v.set_len(v.capacity());
            v
        };

        let mut font_texture = FontTexture {
            glyph_layouts: Vec::new(),
            characters: Vec::new(),
            rasterizer_buffer: Vec::new(),
            pixel_buffer,
            width,
            height,
            font_size,
            font_id,
            texture_id,
        };

        font_texture.calculate_glyph_metrics(context, characters);
        font_texture.rasterize_glyphs(context);

        context
            .set_texture_buffer_data(font_texture.texture_id, &font_texture.pixel_buffer);

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

    pub fn find_glyph_layout(&self, glyph_char: char) -> Option<&GlyphLayout> {
        match self.characters.binary_search(&glyph_char) {
            Ok(i) => Some(&self.glyph_layouts[i]),
            Err(_) => None,
        }
    }

    pub fn delete(&mut self, context: &mut Context) {
        context.delete_texture_buffer(self.texture_id);
        context.delete_font(self.font_id);
    }

    fn calculate_glyph_metrics<T: Iterator<Item = char>>(
        &mut self,
        context: &mut Context,
        characters: T,
    ) {
        let characters = characters.collect::<Vec<char>>();
        let mut glyph_layouts = Vec::with_capacity(characters.len());

        let mut curr_x: usize = 0;
        let mut curr_y: usize = 0;
        let mut max_glyph_bytes = 0;
        let mut max_glyph_per_row_height = 0;
        const GLYPH_GAP: usize = 1;

        for c in &characters {
            let glyph = context.get_font_glyph(self.font_id, *c).unwrap();

            if curr_x + glyph.width as usize > self.width {
                if curr_y + self.font_size as usize > self.height {
                    panic!("Texture bounds exceeded");
                }

                curr_x = 0;
                curr_y += self.font_size + GLYPH_GAP;
                max_glyph_per_row_height = 0;
            }

            let glyph_width = glyph.width as usize;
            let glyph_height = glyph.height as usize;
            let layout = GlyphLayout {
                x: curr_x as usize,
                y: curr_y,
                u: curr_x as f32 / self.width as f32,
                v: curr_y as f32 / self.height as f32,
                glyph,
            };
            glyph_layouts.push(layout);

            curr_x += glyph_width + GLYPH_GAP;
            max_glyph_bytes = max_glyph_bytes.max(glyph_width * glyph_height);
            max_glyph_per_row_height = max_glyph_per_row_height.max(glyph_height);
        }

        self.characters = characters;
        self.glyph_layouts = glyph_layouts;
        self.rasterizer_buffer.reserve(max_glyph_bytes);
        unsafe {
            self.rasterizer_buffer
                .set_len(self.rasterizer_buffer.capacity());
        }
    }

    fn rasterize_glyphs(&mut self, context: &mut Context) {
        for (layout, c) in self.glyph_layouts.iter().zip(self.characters.iter()) {
            unsafe {
                context
                    .rasterize_glyph_to_buffer(
                        self.font_id,
                        *c,
                        &mut self.rasterizer_buffer,
                    )
                    .unwrap();
            }

            for h in 0..layout.glyph.height {
                let h = h as usize;
                let glyph_width = layout.glyph.width as usize;

                let src_row_start = h * glyph_width;
                let src_row_end = src_row_start + glyph_width;
                let src = &self.rasterizer_buffer[src_row_start..src_row_end];

                let dst_row_start = layout.x + (layout.y + h) * self.width;
                let dst_row_end = dst_row_start + glyph_width;
                let dst = &mut self.pixel_buffer[dst_row_start..dst_row_end];

                dst.copy_from_slice(src);
            }
        }
    }
}
