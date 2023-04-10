#include "division_engine/vertex_buffer.h"

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "glad/gl.h"

#include "division_engine/vertex_attribute.h"
#include "division_engine/render_pass.h"

typedef struct {
    DivisionEngineVertexAttribute attr;
    GLenum gl_type;
    int32_t offset;
    int32_t base_size;
    int32_t component_count;
} VertexAttributeInternal_;

typedef struct DivisionVertexBufferInternal_ {
    VertexAttributeInternal_* attributes;
    size_t attribute_count;
    size_t per_vertex_data_size;
    size_t vertex_count;
    GLuint gl_buffer;
} DivisionVertexBufferInternal_;

typedef struct DivisionAttributeTraitsInternal_ {
    GLenum gl_type;
    int32_t base_size;
    int32_t component_count;
} DivisionAttributeTraitsInternal_;

static bool division_attribute_get_traits(
    DivisionAttributeType attributeType, DivisionAttributeTraitsInternal_* output_trait);

static inline GLenum topology_to_gl_type(DivisionRenderTopology t);

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx)
{
    ctx->vertex_buffer_context = (DivisionVertexBufferSystemContext) {
        .buffers = NULL,
        .buffers_count = 0
    };

    return true;
}

void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext* vertex_buffer_ctx = &ctx->vertex_buffer_context;
    for (int i = 0; i < vertex_buffer_ctx->buffers_count; i++)
    {
        free(vertex_buffer_ctx->buffers[i].attributes);
    }

    free(vertex_buffer_ctx->buffers);
    vertex_buffer_ctx->buffers = NULL;
}

int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx, DivisionEngineVertexAttribute* attrs, int32_t attr_count, int32_t reserved_vertex_count)
{
    int32_t per_vertex_data_size = 0;
    int32_t prev_attr_size = 0;

    GLuint gl_handle;
    glGenBuffers(1, &gl_handle);
    glBindBuffer(GL_ARRAY_BUFFER, gl_handle);

    DivisionVertexBufferSystemContext* vertex_ctx = &ctx->vertex_buffer_context;
    DivisionVertexBufferInternal_ vertex_buffer = {
        .attributes = malloc(sizeof(VertexAttributeInternal_) * attr_count),
        .gl_buffer = gl_handle,
        .attribute_count = attr_count,
    };

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionEngineVertexAttribute at = attrs[i];

        DivisionAttributeTraitsInternal_ attr_traits;
        if (!division_attribute_get_traits(at.type, &attr_traits))
        {
            free(vertex_buffer.attributes);
            return -1;
        };

        int32_t attr_size = attr_traits.base_size * attr_traits.component_count;
        int32_t offset = per_vertex_data_size;
        per_vertex_data_size += attr_size;

        vertex_buffer.attributes[i] = (VertexAttributeInternal_) {
            .attr = at,
            .gl_type = attr_traits.gl_type,
            .offset = offset,
            .base_size = attr_traits.base_size,
            .component_count = attr_traits.component_count,
        };
    }

    for (int32_t i = 0; i < attr_count; i++)
    {
        VertexAttributeInternal_* at = &vertex_buffer.attributes[i];

        glVertexAttribPointer(
            at->attr.location, at->component_count, at->gl_type, GL_FALSE, per_vertex_data_size, (void*) at->offset);
        glEnableVertexAttribArray(at->attr.location);
    }

    vertex_buffer.per_vertex_data_size = per_vertex_data_size;
    vertex_buffer.vertex_count = reserved_vertex_count;

    int32_t buffers_count = vertex_ctx->buffers_count;
    vertex_ctx->buffers = realloc(vertex_ctx->buffers, (buffers_count + 1) * sizeof(DivisionVertexBufferInternal_));
    vertex_ctx->buffers[buffers_count] = vertex_buffer;
    vertex_ctx->buffers_count++;

    glBufferData(GL_ARRAY_BUFFER, (GLsizei) (per_vertex_data_size * reserved_vertex_count), NULL, GL_STATIC_DRAW);

    return vertex_ctx->buffers_count - 1;
}
void division_engine_vertex_buffer_set_vertex_data_for_attribute(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
)
{
    DivisionVertexBufferInternal_ vb = ctx->vertex_buffer_context.buffers[vertex_buffer];
    glBindBuffer(GL_ARRAY_BUFFER, vb.gl_buffer);
    void* buffer_data_ptr = glMapBuffer(GL_ARRAY_BUFFER, GL_WRITE_ONLY);

    size_t end_vertex = first_vertex_index + vertex_count - 1;
    VertexAttributeInternal_ attr = vb.attributes[attribute_index];
    int32_t attr_size = attr.base_size * attr.component_count;

    for (size_t vi = first_vertex_index; vi <= end_vertex; vi++)
    {
        const void* src_data_ptr = data_ptr + attr_size * vi;
        void* dst_data_ptr = buffer_data_ptr + vb.per_vertex_data_size * vi + attr.offset;
        memcpy(dst_data_ptr, src_data_ptr, attr_size);
    }

    glUnmapBuffer(GL_ARRAY_BUFFER);
}

bool division_attribute_get_traits(DivisionAttributeType attributeType, DivisionAttributeTraitsInternal_* output_trait)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 1};
            return true;
        case DIVISION_DOUBLE:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_DOUBLE, 8, 1};
            return true;
        case DIVISION_INTEGER:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_INT, 4, 1};
            return true;
        case DIVISION_FVEC2:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 2};
            return true;
        case DIVISION_FVEC3:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 3};
            return true;
        case DIVISION_FVEC4:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 4};
            return true;
        case DIVISION_FMAT4X4:
            *output_trait = (DivisionAttributeTraitsInternal_) {GL_FLOAT, 4, 16};
            return true;
        default:
        {
            fprintf(stderr, "Unknown attribute type");
            return false;
        }
    };
}

void division_engine_internal_vertex_buffer_draw(DivisionContext* ctx)
{
    DivisionRenderPassSystemContext* pass_ctx = &ctx->render_pass_context;

    for (int32_t i = 0; i < pass_ctx->render_pass_count; i++) {
        DivisionRenderPass pass = pass_ctx->render_passes[i];
        GLenum gl_draw_type = topology_to_gl_type(pass.topology);
        GLuint gl_buffer = ctx->vertex_buffer_context.buffers[pass.vertex_buffer].gl_buffer;

        glBindBuffer(GL_ARRAY_BUFFER, gl_buffer);
        glUseProgram(pass.shader_program);
        glDrawArrays(gl_draw_type, (int) pass.first_vertex, (int) pass.vertex_count);
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
