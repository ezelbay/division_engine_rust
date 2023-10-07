use std::path::Path;

use division_engine_rust_macro::location;
use division_math::{Vector2, Vector4};

use crate::core::{
    AlphaBlend, AlphaBlendOperation, Context, DivisionId, FontTexture, IdWithBinding,
    RenderTopology, ShaderVariableType, VertexAttributeDescriptor, VertexData,
};

use super::color::Color32;

pub struct TextDrawSystem {
    font_texture: FontTexture,
    vertex_buffer_id: u32,
    uniform_buffer_id: u32,
    render_pass_id: u32,
    instance_count: usize,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct UniformData {
    pub size: Vector2,
}

#[repr(C, packed)]
#[derive(VertexData, Clone, Copy)]
struct TextVertex {
    #[location(0)]
    pub vert_pos: Vector2,
    #[location(1)]
    pub uv: Vector2,
}

#[repr(C, packed)]
#[derive(VertexData, Clone, Copy)]
struct TextInstance {
    #[location(2)]
    pub color: Vector4,
    #[location(3)]
    pub texel_coord: Vector2,
    #[location(4)]
    pub size: Vector2,
    #[location(5)]
    pub position: Vector2,
    #[location(6)]
    pub glyph_in_tex_size: Vector2,
    #[location(7)]
    pub tex_size: Vector2,
}

const VERTEX_PER_RECT: usize = 4;
const INDEX_PER_RECT: usize = 6;
const RECT_CAPACITY: usize = 64;
const RASTERIZED_FONT_SIZE: usize = 64;

impl TextDrawSystem {
    pub fn new(context: &mut Context, font_path: &Path) -> Self {
        let char_set = [' '..='~'].into_iter().flatten();
        let font_texture =
            FontTexture::new(context, font_path, RASTERIZED_FONT_SIZE, char_set);
        let shader_id = context
            .create_bundled_shader_program(
                &Path::new("resources")
                    .join("shaders")
                    .join("canvas")
                    .join("font"),
            )
            .unwrap();

        let vertex_buffer_id = context
            .create_vertex_buffer::<TextVertex, TextInstance>(
                VERTEX_PER_RECT,
                INDEX_PER_RECT,
                RECT_CAPACITY,
                RenderTopology::Triangles,
            )
            .unwrap();

        fill_vertex_data(context, vertex_buffer_id);

        let uniform_buffer_id = context
            .create_uniform_buffer_with_size_of::<UniformData>()
            .unwrap();

        let render_pass_id = context
            .render_pass_builder()
            .shader(shader_id)
            .vertex_buffer(vertex_buffer_id, VERTEX_PER_RECT, INDEX_PER_RECT)
            .vertex_uniform_buffers(&[IdWithBinding::new(uniform_buffer_id, 1)])
            .fragment_textures(&[IdWithBinding::new(font_texture.texture_id(), 0)])
            .enable_instancing()
            .alpha_blending(
                AlphaBlend::SrcAlpha,
                AlphaBlend::OneMinusSrcAlpha,
                AlphaBlendOperation::Add,
            )
            .build()
            .unwrap();

        TextDrawSystem {
            font_texture,
            vertex_buffer_id,
            uniform_buffer_id,
            render_pass_id,
            instance_count: 0,
        }
    }

    pub fn draw_text(
        &mut self,
        context: &mut Context,
        text: &str,
        font_size: f32,
        position: Vector2,
        color: Color32,
    ) {
        let font_scale = font_size / RASTERIZED_FONT_SIZE as f32;
        let char_count =
            self.write_text_instance_data(context, text, font_scale, position, color);

        let borrowed_render_pass =
            context.borrow_render_pass_mut_ptr(self.render_pass_id);
        borrowed_render_pass.render_pass.instance_count += char_count as u64;

        self.instance_count += char_count;
    }

    fn write_text_instance_data(
        &mut self,
        context: &mut Context,
        text: &str,
        font_scale: f32,
        position: Vector2,
        color: Color32,
    ) -> usize {
        let data =
            context.vertex_buffer_data::<TextVertex, TextInstance>(self.vertex_buffer_id);

        let font_atlas_size = self.font_texture.size();

        let mut char_count = 0;
        let mut x = position.x;

        for (i, ch) in text.chars().enumerate() {
            let glyph_layout = self.font_texture.find_glyph_layout(ch).unwrap();
            let glyph = glyph_layout.glyph;
            let scaled_advance = glyph.hor_advance as f32 * font_scale;

            if glyph_layout.glyph.width > 0 {
                let scaled_width = glyph.width as f32 * font_scale;
                let scaled_height = glyph.height as f32 * font_scale;
                let scaled_bearing_x = glyph.hor_bearing_x as f32 * font_scale;
                let scaled_bearing_y = glyph.hor_bearing_y as f32 * font_scale;
                let y_offset = scaled_height - scaled_bearing_y;

                data.per_instance_data[self.instance_count + i] = TextInstance {
                    texel_coord: Vector2::new(
                        glyph_layout.x as f32,
                        glyph_layout.y as f32,
                    ),
                    size: Vector2::new(scaled_width, scaled_height),
                    position: Vector2::new(x + scaled_bearing_x, position.y - y_offset),
                    color: color.into(),
                    glyph_in_tex_size: Vector2::new(
                        glyph.width as f32,
                        glyph.height as f32,
                    ),
                    tex_size: font_atlas_size,
                };
            }

            x += scaled_advance as f32;
            char_count += 1;
            debug_assert!(self.instance_count + char_count < RECT_CAPACITY);
        }

        char_count
    }

    pub fn set_canvas_size(&mut self, context: &mut Context, size: Vector2) {
        let data = context.uniform_buffer_data::<UniformData>(self.uniform_buffer_id);
        *(data.data) = UniformData { size };
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_render_pass(self.render_pass_id);

        self.font_texture.delete(context);
        context.delete_vertex_buffer(self.vertex_buffer_id);
        context.delete_uniform_buffer(self.uniform_buffer_id);
    }
}

fn fill_vertex_data(context: &mut Context, vertex_buffer_id: DivisionId) {
    let vb_data =
        context.vertex_buffer_data::<TextVertex, TextInstance>(vertex_buffer_id);
    vb_data.per_vertex_data.copy_from_slice(&[
        TextVertex {
            vert_pos: Vector2::new(0., 1.),
            uv: Vector2::new(0., 0.),
        },
        TextVertex {
            vert_pos: Vector2::new(0., 0.),
            uv: Vector2::new(0., 1.),
        },
        TextVertex {
            vert_pos: Vector2::new(1., 0.),
            uv: Vector2::new(1., 1.),
        },
        TextVertex {
            vert_pos: Vector2::new(1., 1.),
            uv: Vector2::new(1., 0.),
        },
    ]);
    vb_data.vertex_indices.copy_from_slice(&[0, 1, 2, 2, 3, 0]);
}
