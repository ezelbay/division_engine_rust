use std::path::Path;

use super::{
    Context, DivisionId, FontGlyph, TextureChannelSwizzleVariant, TextureChannelsSwizzle,
    TextureFormat,
};

pub struct GlyphLayout {
    glyph: FontGlyph,
    x: usize,
    y: usize,
    character: char,
}

pub struct FontTexture {
    glyph_layouts: Vec<GlyphLayout>,
    width: usize,
    height: usize,
    texture_id: DivisionId,
}

struct GlyphMetrics {
    max_glyph_bytes: usize,
    glyph_layouts: Vec<GlyphLayout>,
}

impl FontTexture {
    pub const DEFAULT_WIDTH: usize = 1024;
    pub const DEFAULT_HEIGHT: usize = 512;

    pub fn new<T: Iterator<Item = char>>(
        context: &mut Context,
        font_path: &Path,
        font_height: usize,
        characters: T,
    ) -> Self {
        Self::new_with_resolution(
            context,
            font_path,
            font_height,
            characters,
            Self::DEFAULT_WIDTH,
            Self::DEFAULT_HEIGHT,
        )
    }

    pub fn new_with_resolution<T: Iterator<Item = char>>(
        context: &mut Context,
        font_path: &Path,
        font_height: usize,
        characters: T,
        width: usize,
        height: usize,
    ) -> Self {
        let mut tex_data = Vec::with_capacity(width * height);
        unsafe { tex_data.set_len(tex_data.capacity()) };

        let font = context.create_font(&font_path, font_height as u32).unwrap();
        let metrics = get_glyph_metrics(context, font, characters, width, height);
        rasterize_glyphs(context, font, &metrics, &mut tex_data, width);
        context.delete_font(font);

        let texture_id = context
            .create_texture_buffer_from_data_with_channels_swizzle(
                width as u32,
                height as u32,
                TextureFormat::R8Uint,
                Some(TextureChannelsSwizzle::all(
                    TextureChannelSwizzleVariant::Red,
                )),
                &tex_data,
            )
            .unwrap();

        FontTexture {
            texture_id,
            glyph_layouts: metrics.glyph_layouts,
            width,
            height,
        }
    }

    pub fn texture_id(&self) -> DivisionId {
        self.texture_id
    }
}

fn get_glyph_metrics<T: Iterator<Item = char>>(
    context: &mut Context,
    font: DivisionId,
    characters: T,
    width: usize,
    height: usize,
) -> GlyphMetrics {
    let mut glyph_layouts = Vec::with_capacity(50);

    let mut curr_x: usize = 0;
    let mut curr_y: usize = 0;
    let mut max_glyph_bytes = 0;
    let mut max_glyph_per_row_height = 0;

    for c in characters {
        let glyph = context.get_font_glyph(font, c).unwrap();

        if curr_x + glyph.width as usize > width {
            if curr_y + glyph.height as usize > height {
                panic!("Texture bounds exceeded");
            }

            curr_x = 0;
            curr_y += max_glyph_per_row_height;
            max_glyph_per_row_height = 0;
        }

        glyph_layouts.push(GlyphLayout {
            x: curr_x,
            y: curr_y,
            glyph,
            character: c,
        });

        curr_x += glyph.width as usize;
        max_glyph_bytes = max_glyph_bytes.max((glyph.width * glyph.height) as usize);
        max_glyph_per_row_height = max_glyph_per_row_height.max(glyph.height as usize);
    }

    GlyphMetrics {
        max_glyph_bytes,
        glyph_layouts,
    }
}

fn rasterize_glyphs(
    context: &mut Context,
    font: DivisionId,
    metrics: &GlyphMetrics,
    tex_data: &mut [u8],
    tex_width: usize,
) {
    let mut glyph_buff = Vec::with_capacity(metrics.max_glyph_bytes);
    unsafe {
        glyph_buff.set_len(metrics.max_glyph_bytes);
    }

    for glyph_layout in &metrics.glyph_layouts {
        let glyph = glyph_layout.glyph;
        unsafe {
            context
                .rasterize_glyph_to_buffer(font, glyph_layout.glyph, &mut glyph_buff)
                .unwrap();
        }

        for h in 0..glyph.height {
            let h = h as usize;
            let src_row_start = h * glyph.width as usize;
            let src_row_end = src_row_start + glyph.width as usize;
            let src = &glyph_buff[src_row_start..src_row_end];

            let dst_row_start = glyph_layout.x + (glyph_layout.y + h) * tex_width;
            let dst_row_end = dst_row_start + glyph.width as usize;
            let dst = &mut tex_data[dst_row_start..dst_row_end];

            dst.copy_from_slice(src);
        }
    }
}
