#include "division_engine/platform_internal/platform_vertex_buffer.h"
#include "division_engine/vertex_buffer.h"

#include <stdlib.h>
#include "glad/gl.h"
#include <stdio.h>
#include <string.h>

typedef struct VertexAttributeInternalPlatform_ {
    GLenum gl_type;
} VertexAttributeInternalPlatform_;

typedef struct DivisionVertexBufferInternalPlatform_ {
    GLuint gl_buffer;
    GLenum gl_topology;
} DivisionVertexBufferInternalPlatform_;

static inline GLenum division_attribute_to_gl_type(DivisionShaderVariableType attributeType);
static inline GLenum topology_to_gl_type(DivisionRenderTopology t);

bool division_engine_internal_platform_vertex_buffer_context_alloc(
    DivisionContext* ctx, const DivisionSettings* settings)
{
    return true;
}

void division_engine_internal_platform_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vertex_buffer_ctx = ctx->vertex_buffer_context;

    for (int i = 0; i < vertex_buffer_ctx->buffers_count; i++)
    {
        DivisionVertexBuffer* buffer = &vertex_buffer_ctx->buffers[i];

        free(buffer->attributes_impl);
    }

    free(vertex_buffer_ctx->buffers_impl);
}

void division_engine_internal_platform_vertex_buffer_alloc(DivisionContext* ctx)
{
    GLuint gl_buffer;
    glGenBuffers(1, &gl_buffer);
    glBindBuffer(GL_ARRAY_BUFFER, gl_buffer);

    struct DivisionVertexBufferSystemContext* vertex_ctx = ctx->vertex_buffer_context;
    struct DivisionVertexBuffer* vertex_buffer = &vertex_ctx->buffers[vertex_ctx->buffers_count - 1];
    int attr_count = vertex_buffer->attribute_count;

    vertex_buffer->attributes_impl = malloc(sizeof(VertexAttributeInternalPlatform_) * attr_count);

    vertex_ctx->buffers_impl = realloc(
        vertex_ctx->buffers_impl, vertex_ctx->buffers_count * sizeof(DivisionVertexBufferInternalPlatform_));
    vertex_ctx->buffers_impl[vertex_ctx->buffers_count - 1] = (DivisionVertexBufferInternalPlatform_) {
        .gl_buffer = gl_buffer,
        .gl_topology = topology_to_gl_type(vertex_buffer->topology)
    };

    int per_vertex_data_size = (int) vertex_buffer->per_vertex_data_size;

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionVertexAttribute* at = &vertex_buffer->attributes[i];
        VertexAttributeInternalPlatform_ at_impl = {
            .gl_type = division_attribute_to_gl_type(vertex_buffer->attributes[i].type)
        };
        vertex_buffer->attributes_impl[i] = at_impl;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wint-to-pointer-cast"
        void* offset = (void*) at->offset;
#pragma clang diagnostic pop

        glVertexAttribPointer(
            at->location,
            at->component_count,
            at_impl.gl_type,
            GL_FALSE,
            (int) vertex_buffer->per_vertex_data_size,
            offset
        );
        glEnableVertexAttribArray(at->location);
    }

    glBufferData(GL_ARRAY_BUFFER, (GLsizei) (per_vertex_data_size * vertex_buffer->vertex_count), NULL, GL_STATIC_DRAW);
}

GLenum division_attribute_to_gl_type(DivisionShaderVariableType attributeType)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT:
            return GL_FLOAT;
        case DIVISION_DOUBLE:
            return GL_DOUBLE;
        case DIVISION_INTEGER:
            return GL_INT;
        case DIVISION_FVEC2:
            return GL_FLOAT;
        case DIVISION_FVEC3:
            return GL_FLOAT;
        case DIVISION_FVEC4:
            return GL_FLOAT;
        case DIVISION_FMAT4X4:
            return GL_FLOAT;
        default:
        {
            fprintf(stderr, "Unknown attribute type");
        }
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

void division_engine_internal_platform_vertex_buffer_set_vertex_data(
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
    GLuint gl_buffer = vertex_buffer_context->buffers_impl[vertex_buffer].gl_buffer;
    DivisionVertexBufferObjects vb_objs = vertex_buffer_context->buffers_objects[vertex_buffer];

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

void division_engine_internal_platform_vertex_buffer_draw(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vert_buff_ctx = ctx->vertex_buffer_context;
    DivisionVertexBufferObjects* buffers_objects = vert_buff_ctx->buffers_objects;
    DivisionVertexBufferInternalPlatform_ * buffers_impl = vert_buff_ctx->buffers_impl;
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
