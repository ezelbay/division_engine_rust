use std::path::Path;

use super::{
    Context, DivisionId, FontGlyph, TextureChannelSwizzleVariant, TextureChannelsSwizzle,
    TextureDescriptor, TextureFormat, TextureMinMagFilter,
};

#[derive(Clone, Copy)]
pub struct GlyphLayout {
    x: usize,
    y: usize,
    u: f32,
    v: f32,
    glyph: FontGlyph,
}

pub struct FontTexture {
    glyph_layouts: Vec<GlyphLayout>,
    characters: Vec<char>,
    width: usize,
    height: usize,
    texture_id: DivisionId,
}

struct GlyphMetrics {
    glyph_layouts: Vec<GlyphLayout>,
    characters: Vec<char>,
    max_glyph_bytes: usize,
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
            .create_texture_buffer_from_data(
                &TextureDescriptor::new(width, height, TextureFormat::R8Uint)
                    .with_channels_swizzle(TextureChannelsSwizzle::all(
                        TextureChannelSwizzleVariant::Red,
                    ))
                    .with_min_mag_filter(
                        TextureMinMagFilter::Linear,
                        TextureMinMagFilter::Linear,
                    ),
                &tex_data,
            )
            .unwrap();

        FontTexture {
            texture_id,
            glyph_layouts: metrics.glyph_layouts,
            characters: metrics.characters,
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
    let characters = characters.collect::<Vec<char>>();
    let mut glyph_layouts = Vec::with_capacity(characters.len());

    let mut curr_x: usize = 0;
    let mut curr_y: usize = 0;
    let mut max_glyph_bytes = 0;
    let mut max_glyph_per_row_height = 0;

    for c in &characters {
        let glyph = context.get_font_glyph(font, *c).unwrap();

        if curr_x + glyph.width as usize > width {
            if curr_y + glyph.height as usize > height {
                panic!("Texture bounds exceeded");
            }

            curr_x = 0;
            curr_y += max_glyph_per_row_height;
            max_glyph_per_row_height = 0;
        }

        let width = glyph.width as usize;
        let height = glyph.height as usize;
        let layout = GlyphLayout {
            x: curr_x,
            y: curr_y,
            u: curr_x as f32 / width as f32,
            v: curr_y as f32 / height as f32,
            glyph,
        };
        glyph_layouts.push(layout);

        curr_x += width;
        max_glyph_bytes = max_glyph_bytes.max(width * height);
        max_glyph_per_row_height = max_glyph_per_row_height.max(height);
    }

    GlyphMetrics {
        max_glyph_bytes,
        glyph_layouts,
        characters,
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

    for (layout, c) in metrics.glyph_layouts.iter().zip(metrics.characters.iter()) {
        unsafe {
            context
                .rasterize_glyph_to_buffer(font, *c, &mut glyph_buff)
                .unwrap();
        }

        for h in 0..layout.glyph.height {
            let h = h as usize;
            let glyph_width = layout.glyph.width as usize;

            let src_row_start = h * glyph_width;
            let src_row_end = src_row_start + glyph_width;
            let src = &glyph_buff[src_row_start..src_row_end];

            let dst_row_start = layout.x + (layout.y + h) * tex_width;
            let dst_row_end = dst_row_start + glyph_width;
            let dst = &mut tex_data[dst_row_start..dst_row_end];

            dst.copy_from_slice(src);
        }
    }
}
