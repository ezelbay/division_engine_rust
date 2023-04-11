#include "division_engine/vertex_buffer.h"

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "glad/gl.h"

typedef struct VertexAttributeInternalImpl_ {
    GLenum gl_type;
} VertexAttributeInternalImpl_;

typedef struct DivisionVertexBufferInternalImpl_ {
    GLuint gl_buffer;
    GLenum gl_topology;
} DivisionVertexBufferInternalImpl_;

typedef struct AttrTraits_ {
    GLenum gl_type;
    int32_t base_size;
    int32_t component_count;
} AttrTraits_;

static inline AttrTraits_ division_attribute_get_traits(DivisionShaderVariableType attributeType);
static inline GLenum topology_to_gl_type(DivisionRenderTopology t);

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx)
{
    ctx->vertex_buffer_context = malloc(sizeof(DivisionVertexBufferSystemContext));
    *ctx->vertex_buffer_context = (DivisionVertexBufferSystemContext) {
        .buffers = NULL,
        .buffers_impl = NULL,
        .buffers_objects = NULL,
        .buffers_count = 0,
        .render_passes = NULL,
        .render_pass_count = 0
    };

    return true;
}

void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vertex_buffer_ctx = ctx->vertex_buffer_context;
    for (int i = 0; i < vertex_buffer_ctx->buffers_count; i++)
    {
        DivisionVertexBuffer* buffer = &vertex_buffer_ctx->buffers[i];
        DivisionVertexBufferObjects* buffer_objects = &vertex_buffer_ctx->buffers_objects[i];

        free(buffer->attributes);
        free(buffer->attributes_impl);
        free(buffer_objects->objects_start_vertex);
        free(buffer_objects->objects_vertex_count);
    }

    free(vertex_buffer_ctx->buffers);
    free(vertex_buffer_ctx->buffers_impl);
    free(vertex_buffer_ctx->render_passes);
    free(vertex_buffer_ctx);
}

int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx,
    DivisionVertexAttributeSettings* attrs,
    int32_t attr_count,
    int32_t vertex_count,
    DivisionRenderTopology render_topology
)
{
    int32_t per_vertex_data_size = 0;

    GLuint gl_buffer;
    glGenBuffers(1, &gl_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, gl_buffer);

    DivisionVertexBufferSystemContext* vertex_ctx = ctx->vertex_buffer_context;

    DivisionVertexBuffer vertex_buffer = {
        .attributes = malloc(sizeof(DivisionVertexAttribute) * attr_count),
        .attributes_impl = malloc(sizeof(VertexAttributeInternalImpl_) * attr_count),
        .attribute_count = attr_count,
    };

    DivisionVertexBufferObjects vertex_buffer_objects = {
        .objects_start_vertex = malloc(sizeof(int32_t)),
        .objects_vertex_count = malloc(sizeof(int32_t)),
        .objects_count = 1
    };
    vertex_buffer_objects.objects_start_vertex[0] = 0;
    vertex_buffer_objects.objects_vertex_count[0] = vertex_count;

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionVertexAttributeSettings at = attrs[i];
        AttrTraits_ attr_traits = division_attribute_get_traits(at.type);

        int32_t attr_size = attr_traits.base_size * attr_traits.component_count;
        int32_t offset = per_vertex_data_size;
        per_vertex_data_size += attr_size;

        vertex_buffer.attributes_impl[i] = (VertexAttributeInternalImpl_) {
            .gl_type = attr_traits.gl_type
        };
        vertex_buffer.attributes[i] = (DivisionVertexAttribute) {
            .location = at.location,
            .offset = offset,
            .base_size = attr_traits.base_size,
            .component_count = attr_traits.component_count,
        };
    }

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionVertexAttribute* at = &vertex_buffer.attributes[i];
        VertexAttributeInternalImpl_* at_impl = &vertex_buffer.attributes_impl[i];

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wint-to-pointer-cast"
        void* offset = (void*) at->offset;
#pragma clang diagnostic pop

        glVertexAttribPointer(
            at->location, at->component_count, at_impl->gl_type, GL_FALSE, per_vertex_data_size, offset);
        glEnableVertexAttribArray(at->location);
    }

    vertex_buffer.per_vertex_data_size = per_vertex_data_size;
    vertex_buffer.vertex_count = vertex_count;

    int32_t buffers_count = vertex_ctx->buffers_count;
    int32_t new_buffers_count = buffers_count + 1;
    vertex_ctx->buffers = realloc(vertex_ctx->buffers, new_buffers_count * sizeof(DivisionVertexBuffer));
    vertex_ctx->buffers_impl = realloc(
        vertex_ctx->buffers_impl, new_buffers_count * sizeof(DivisionVertexBufferInternalImpl_));
    vertex_ctx->buffers_objects = realloc(
        vertex_ctx->buffers_objects, new_buffers_count * sizeof(DivisionVertexBufferObjects));

    vertex_ctx->buffers[buffers_count] = vertex_buffer;
    vertex_ctx->buffers_impl[buffers_count] = (DivisionVertexBufferInternalImpl_) {
        .gl_buffer = gl_buffer,
        .gl_topology = topology_to_gl_type(render_topology)
    };
    vertex_ctx->buffers_objects[buffers_count] = vertex_buffer_objects;

    vertex_ctx->buffers_count++;

    glBufferData(GL_ARRAY_BUFFER, (GLsizei) (per_vertex_data_size * vertex_count), NULL, GL_STATIC_DRAW);

    return vertex_ctx->buffers_count - 1;
}

void division_engine_vertex_buffer_set_vertex_data_for_attribute(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t object_index,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
)
{
    DivisionVertexBufferSystemContext* vertex_buffer_context = ctx->vertex_buffer_context;
    DivisionVertexBuffer vb = vertex_buffer_context->buffers[vertex_buffer];
    DivisionVertexBufferObjects vb_objs = vertex_buffer_context->buffers_objects[vertex_buffer];
    GLuint gl_buffer = vertex_buffer_context->buffers_impl[vertex_buffer].gl_buffer;
    size_t obj_vertex_count = vb_objs.objects_vertex_count[object_index];

    if (vertex_count > obj_vertex_count)
    {
        vertex_count = obj_vertex_count;
        fprintf(stderr, "Vertex count overflow. Available: %zu, but set: %zu", obj_vertex_count, vertex_count);
    }

    glBindBuffer(GL_ARRAY_BUFFER, gl_buffer);
    void* buffer_data_ptr = glMapBuffer(GL_ARRAY_BUFFER, GL_WRITE_ONLY);

    size_t obj_first_index = vb_objs.objects_start_vertex[object_index] + first_vertex_index;
    DivisionVertexAttribute attr = vb.attributes[attribute_index];
    int32_t attr_size = attr.base_size * attr.component_count;

    for (size_t i = 0; i < vertex_count; i++)
    {
        size_t src_vi = first_vertex_index + i;
        size_t obj_vi = obj_first_index + i;

        const void* src_data_ptr = data_ptr + attr_size * src_vi;
        void* dst_data_ptr = buffer_data_ptr + vb.per_vertex_data_size * obj_vi + attr.offset;
        memcpy(dst_data_ptr, src_data_ptr, attr_size);
    }

    glUnmapBuffer(GL_ARRAY_BUFFER);
}

AttrTraits_ division_attribute_get_traits(DivisionShaderVariableType attributeType)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT:
            return (AttrTraits_) {GL_FLOAT, 4, 1};
        case DIVISION_DOUBLE:
            return (AttrTraits_) {GL_DOUBLE, 8, 1};
        case DIVISION_INTEGER:
            return (AttrTraits_) {GL_INT, 4, 1};
        case DIVISION_FVEC2:
            return (AttrTraits_) {GL_FLOAT, 4, 2};
        case DIVISION_FVEC3:
            return (AttrTraits_) {GL_FLOAT, 4, 3};
        case DIVISION_FVEC4:
            return (AttrTraits_) {GL_FLOAT, 4, 4};
        case DIVISION_FMAT4X4:
            return (AttrTraits_) {GL_FLOAT, 4, 16};
        default:
        {
            fprintf(stderr, "Unknown attribute type");
        }
    }
}

int32_t division_engine_vertex_buffer_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass render_pass)
{
    DivisionVertexBufferSystemContext* pass_ctx = ctx->vertex_buffer_context;

    int32_t render_pass_count = pass_ctx->render_pass_count;
    int32_t new_render_pass_count = render_pass_count + 1;
    pass_ctx->render_passes = realloc(pass_ctx->render_passes, sizeof(DivisionRenderPass) * new_render_pass_count);
    pass_ctx->render_passes[render_pass_count] = render_pass;
    pass_ctx->render_pass_count++;

    return pass_ctx->render_pass_count - 1;
}

void division_engine_internal_vertex_buffer_draw(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vert_buff_ctx = ctx->vertex_buffer_context;
    DivisionVertexBufferObjects* buffers_objects = vert_buff_ctx->buffers_objects;
    DivisionVertexBufferInternalImpl_* buffers_impl = vert_buff_ctx->buffers_impl;
    int32_t pass_count = vert_buff_ctx->render_pass_count;

    for (int32_t i = 0; i < pass_count; i++)
    {
        DivisionRenderPass pass = vert_buff_ctx->render_passes[i];
        DivisionVertexBufferObjects vb_objs = buffers_objects[pass.vertex_buffer];
        GLuint gl_buffer = buffers_impl[i].gl_buffer;
        GLenum gl_draw_type = buffers_impl[i].gl_topology;

        glBindBuffer(GL_ARRAY_BUFFER, gl_buffer);
        glUseProgram(pass.shader_program);
        glMultiDrawArrays(
            gl_draw_type, vb_objs.objects_start_vertex, vb_objs.objects_vertex_count, vb_objs.objects_count);
    }
}

GLenum topology_to_gl_type(DivisionRenderTopology t)
{
    switch (t)
    {
        case DIVISION_TOPOLOGY_TRIANGLES:
            return GL_TRIANGLES;
        case DIVISION_TOPOLOGY_LINES:
            return GL_LINES;
        case DIVISION_TOPOLOGY_POINTS:
            return GL_POINTS;
        default:
        {
            fprintf(stderr, "Unknown type of topology");
            exit(EXIT_FAILURE);
        }
    }
}
