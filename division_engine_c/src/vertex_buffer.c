#include "division_engine/vertex_buffer.h"

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "glad/gl.h"

#include "division_engine/vertex_attribute.h"
#include "division_engine/render_pass.h"

typedef struct {
    int32_t location;
    GLenum gl_type;
    int32_t offset;
    int32_t base_size;
    int32_t component_count;
} VertexAttributeInternal_;

typedef struct DivisionVertexBufferInternal_ {
    VertexAttributeInternal_* attributes;
    int32_t attribute_count;

    int32_t* objects_start_vertex;
    int32_t* objects_vertex_count;
    int32_t objects_count;
    int32_t vertex_count;

    GLuint gl_buffer;

    size_t per_vertex_data_size;
} DivisionVertexBufferInternal_;

typedef struct DivisionAttributeTraitsInternal_ {
    GLenum gl_type;
    int32_t base_size;
    int32_t component_count;
} DivisionAttributeTraitsInternal_;

static inline DivisionAttributeTraitsInternal_ division_attribute_get_traits(DivisionVariableType attributeType);
static inline GLenum topology_to_gl_type(DivisionRenderTopology t);

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx)
{
    ctx->vertex_buffer_context = malloc(sizeof(DivisionVertexBufferSystemContext));
    *ctx->vertex_buffer_context = (DivisionVertexBufferSystemContext) {
        .buffers = NULL,
        .buffers_count = 0
    };

    return true;
}

void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vertex_buffer_ctx = ctx->vertex_buffer_context;
    for (int i = 0; i < vertex_buffer_ctx->buffers_count; i++)
    {
        free(vertex_buffer_ctx->buffers[i].attributes);
        free(vertex_buffer_ctx->buffers[i].objects_start_vertex);
        free(vertex_buffer_ctx->buffers[i].objects_vertex_count);
    }

    free(vertex_buffer_ctx->buffers);
    free(vertex_buffer_ctx);
}

int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx, DivisionEngineVertexAttribute* attrs, int32_t attr_count, int32_t vertex_count)
{
    int32_t per_vertex_data_size = 0;

    GLuint gl_handle;
    glGenBuffers(1, &gl_handle);
    glBindBuffer(GL_ARRAY_BUFFER, gl_handle);

    DivisionVertexBufferSystemContext* vertex_ctx = ctx->vertex_buffer_context;
    DivisionVertexBufferInternal_ vertex_buffer = {
        .attributes = malloc(sizeof(VertexAttributeInternal_) * attr_count),
        .gl_buffer = gl_handle,
        .attribute_count = attr_count,
        .objects_start_vertex = malloc(sizeof(int32_t)),
        .objects_vertex_count = malloc(sizeof(int32_t)),
        .objects_count = 1
    };

    vertex_buffer.objects_start_vertex[0] = 0;
    vertex_buffer.objects_vertex_count[0] = vertex_count;

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionEngineVertexAttribute at = attrs[i];
        DivisionAttributeTraitsInternal_ attr_traits = division_attribute_get_traits(at.type);

        int32_t attr_size = attr_traits.base_size * attr_traits.component_count;
        int32_t offset = per_vertex_data_size;
        per_vertex_data_size += attr_size;

        vertex_buffer.attributes[i] = (VertexAttributeInternal_) {
            .location = at.location,
            .gl_type = attr_traits.gl_type,
            .offset = offset,
            .base_size = attr_traits.base_size,
            .component_count = attr_traits.component_count,
        };
    }

    for (int32_t i = 0; i < attr_count; i++)
    {
        VertexAttributeInternal_* at = &vertex_buffer.attributes[i];

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wint-to-pointer-cast"
        void* offset = (void*) at->offset;
#pragma clang diagnostic pop

        glVertexAttribPointer(
            at->location, at->component_count, at->gl_type, GL_FALSE, per_vertex_data_size, offset);
        glEnableVertexAttribArray(at->location);
    }

    vertex_buffer.per_vertex_data_size = per_vertex_data_size;
    vertex_buffer.vertex_count = vertex_count;

    int32_t buffers_count = vertex_ctx->buffers_count;
    vertex_ctx->buffers = realloc(vertex_ctx->buffers, (buffers_count + 1) * sizeof(DivisionVertexBufferInternal_));
    vertex_ctx->buffers[buffers_count] = vertex_buffer;
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
    DivisionVertexBufferInternal_ vb = ctx->vertex_buffer_context->buffers[vertex_buffer];
    size_t obj_vertex_count = vb.objects_vertex_count[object_index];

    if (vertex_count > obj_vertex_count)
    {
        vertex_count = obj_vertex_count;
        fprintf(stderr, "Vertex count overflow. Available: %zu, but set: %zu", obj_vertex_count, vertex_count);
    }

    glBindBuffer(GL_ARRAY_BUFFER, vb.gl_buffer);
    void* buffer_data_ptr = glMapBuffer(GL_ARRAY_BUFFER, GL_WRITE_ONLY);

    size_t obj_first_index = vb.objects_start_vertex[object_index] + first_vertex_index;
    VertexAttributeInternal_ attr = vb.attributes[attribute_index];
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

DivisionAttributeTraitsInternal_ division_attribute_get_traits(DivisionVariableType attributeType)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT: return (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 1};
        case DIVISION_DOUBLE: return (DivisionAttributeTraitsInternal_) {GL_DOUBLE, 8, 1};
        case DIVISION_INTEGER: return (DivisionAttributeTraitsInternal_) {GL_INT, 4, 1};
        case DIVISION_FVEC2: return (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 2};
        case DIVISION_FVEC3: return (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 3};
        case DIVISION_FVEC4: return (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 4};
        case DIVISION_FMAT4X4: return (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 16};
        default:
        {
            fprintf(stderr, "Unknown attribute type");
        }
    }
}

void division_engine_internal_vertex_buffer_draw(DivisionContext* ctx)
{
    DivisionRenderPassSystemContext* pass_ctx = ctx->render_pass_context;
    DivisionVertexBufferSystemContext* vert_buff_ctx = ctx->vertex_buffer_context;

    for (int32_t i = 0; i < pass_ctx->render_pass_count; i++) {
        DivisionRenderPass pass = pass_ctx->render_passes[i];
        GLenum gl_draw_type = topology_to_gl_type(pass.topology);
        DivisionVertexBufferInternal_ vb = vert_buff_ctx->buffers[pass.vertex_buffer];

        glBindBuffer(GL_ARRAY_BUFFER, vb.gl_buffer);
        glUseProgram(pass.shader_program);
        glMultiDrawArrays(gl_draw_type, vb.objects_start_vertex, vb.objects_vertex_count, vb.objects_count);
    }
}

GLenum topology_to_gl_type(DivisionRenderTopology t)
{
    switch (t)
    {
        case DIVISION_TOPOLOGY_TRIANGLES: return GL_TRIANGLES;
        case DIVISION_TOPOLOGY_LINES: return GL_LINES;
        case DIVISION_TOPOLOGY_POINTS: return GL_POINTS;
        default:
        {
            fprintf(stderr, "Unknown type of topology");
            exit(EXIT_FAILURE);
        }
    }
}
