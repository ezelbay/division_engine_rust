use std::path::Path;

use division_engine_rust_macro::location;
use division_math::{Vector2, Vector4};

use crate::core::{
    AlphaBlend, AlphaBlendOperation, Context, DivisionId, IdWithBinding,
    RenderPassDescriptor, RenderPassInstance, RenderTopology, ShaderVariableType,
    VertexAttributeDescriptor, VertexBufferSize, VertexData,
};

use super::{
    renderable_text::RenderableText,
    renderer::{RenderQueue, Renderer}, font_texture::FontTexture,
};

pub struct TextRenderer {
    font_texture: FontTexture,
    screen_size_uniform: IdWithBinding,
    textures_heap: Vec<IdWithBinding>,
    vertex_buffer_id: u32,
    render_pass_desc_id: u32,
    instance_count: u32,
    instance_capacity: u32,
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

const TEXTURE_SHADER_LOCATION: u32 = 0;
const SCREEN_SIZE_UNIFORM_LOCATION: u32 = 1;

const VERTEX_PER_RECT: u32 = 4;
const INDEX_PER_RECT: u32 = 6;
const DEFAULT_RECT_CAPACITY: u32 = 1024;

const RASTERIZED_FONT_SIZE: usize = 64;

impl TextRenderer {
    pub fn new(
        context: &mut Context,
        screen_size_uniform_id: DivisionId,
        font_path: &Path,
    ) -> Self {
        Self::with_capacity(
            context,
            screen_size_uniform_id,
            font_path,
            DEFAULT_RECT_CAPACITY,
        )
    }

    pub fn with_capacity(
        context: &mut Context,
        screen_size_uniform_id: DivisionId,
        font_path: &Path,
        characters_capacity: u32,
    ) -> TextRenderer {
        let font_texture =
            FontTexture::new(context, font_path, RASTERIZED_FONT_SIZE).unwrap();
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
                VertexBufferSize {
                    vertex_count: VERTEX_PER_RECT,
                    index_count: INDEX_PER_RECT,
                    instance_count: characters_capacity,
                },
                RenderTopology::Triangles,
            )
            .unwrap();

        fill_vertex_data(context, vertex_buffer_id);

        let render_pass_desc_id = context
            .create_render_pass_descriptor(
                &RenderPassDescriptor::with_shader_and_vertex_buffer(
                    shader_id,
                    vertex_buffer_id,
                )
                .alpha_blending(
                    AlphaBlend::SrcAlpha,
                    AlphaBlend::OneMinusSrcAlpha,
                    AlphaBlendOperation::Add,
                ),
            )
            .unwrap();

        TextRenderer {
            font_texture,
            vertex_buffer_id,
            screen_size_uniform: IdWithBinding::new(
                screen_size_uniform_id,
                SCREEN_SIZE_UNIFORM_LOCATION,
            ),
            textures_heap: Vec::new(),
            instance_count: 0,
            render_pass_desc_id,
            instance_capacity: characters_capacity,
        }
    }

    fn add_text_to_pass(
        &mut self,
        context: &mut Context,
        render_pass_instance: &mut RenderPassInstance,
        renderable: &RenderableText,
    ) {
        let font_atlas_size = self.font_texture.size();
        let base_instance = render_pass_instance.instance_count as usize;
        let font_scale = renderable.font_size / RASTERIZED_FONT_SIZE as f32;

        let char_count = {
            let mut char_count = 0;
            for ch in renderable.text.chars() {
                self.font_texture.cache_character(context, ch).unwrap();
                char_count += 1;
            }
            char_count
        };

        if self.instance_count + char_count > self.instance_capacity {
            context.vertex_buffer_resize(
                self.vertex_buffer_id,
                VertexBufferSize {
                    vertex_count: VERTEX_PER_RECT,
                    index_count: INDEX_PER_RECT,
                    instance_count: std::cmp::max(1, self.instance_capacity * 2) as u32,
                },
            );
        }

        let data =
            context.vertex_buffer_data::<TextVertex, TextInstance>(self.vertex_buffer_id);

        let mut pen_pos = renderable.position;

        for (i, ch) in renderable.text.chars().enumerate() {
            let (glyph, pos) = self.font_texture.find_glyph_layout(ch).unwrap();
            let scaled_advance = glyph.advance_x as f32 * font_scale;

            if glyph.width > 0 {
                let scaled_width = glyph.width as f32 * font_scale;
                let scaled_height = glyph.height as f32 * font_scale;
                let offset = Vector2::new(
                    glyph.left as f32 * font_scale,
                    (glyph.top as f32 - glyph.height as f32) * font_scale,
                );

                data.per_instance_data[base_instance + i] = TextInstance {
                    texel_coord: Vector2::new(pos.x as f32, pos.y as f32),
                    size: Vector2::new(scaled_width, scaled_height),
                    position: pen_pos + offset,
                    color: *renderable.color,
                    glyph_in_tex_size: Vector2::new(
                        glyph.width as f32,
                        glyph.height as f32,
                    ),
                    tex_size: font_atlas_size,
                };
            }

            pen_pos.x += scaled_advance as f32;

            render_pass_instance.instance_count += 1;
            self.instance_count += 1;
        }
    }

    pub fn cleanup(&mut self, context: &mut Context) {
        context.delete_render_pass_descriptor(self.render_pass_desc_id);

        self.font_texture.delete(context);
        context.delete_vertex_buffer(self.vertex_buffer_id);
    }
}

impl<'a> Renderer for TextRenderer {
    type RenderableData = RenderableText;

    fn before_render_frame(&mut self, _: &mut Context) {
        self.instance_count = 0;
        self.textures_heap.clear();
    }

    fn enqueue_render_passes(
        &mut self,
        context: &mut Context,
        data: &[Self::RenderableData],
        render_queue: &mut RenderQueue,
    ) {
        if data.len() == 0 {
            return;
        }

        let mut render_pass = RenderPassInstance::new(self.render_pass_desc_id)
            .vertices(VERTEX_PER_RECT, INDEX_PER_RECT)
            .enable_instancing();

        unsafe {
            self.textures_heap.push(IdWithBinding::new(
                self.font_texture.texture_id(),
                TEXTURE_SHADER_LOCATION,
            ));

            render_pass.set_uniform_vertex_buffer_from_ref(&self.screen_size_uniform);
            render_pass.set_uniform_fragment_texture_from_ref(
                &self.textures_heap.last().unwrap_unchecked(),
            );
        }

        for renderable in data {
            self.add_text_to_pass(context, &mut render_pass, renderable)
        }

        render_queue.enqueue_render_pass(render_pass);

        self.font_texture.upload_texture(context);
    }

    fn after_render_frame(&mut self, _: &mut Context) {}
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
