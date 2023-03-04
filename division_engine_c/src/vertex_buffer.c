#include "division_engine/vertex_buffer.h"

#include <stdio.h>
#include <stdlib.h>
#include <glad/gl.h>
#include <stdlib.h>
#include "memory.h"

#include "division_engine/context.h"
#include "division_engine/vertex_attribute.h"
#include "division_engine/list_utility.h"

typedef struct {
    DivisionEngineVertexAttribute attr;
    GLenum gl_type;
    int32_t stride;
    int32_t size_of_components;
} VertexAttributeInternal_;

typedef struct DivisionVertexBufferInternal_ {
    VertexAttributeInternal_* attributes;
    int32_t attributes_count;
    GLuint gl_buffer;
} DivisionVertexBufferInternal_;

static void division_attribute_get_traits(
    DivisionAttributeType attributeType, GLenum* gl_type, int32_t* base_size, int32_t* number_of_components);

bool division_engine_internal_vertex_buffer_create_context(DivisionContext* ctx)
{
    ctx->vertex_buffer_context = (DivisionVertexBufferSystemContext) {
        .buffers = NULL,
        .buffers_count = 0
    };

    return true;
}

void division_engine_internal_vertex_buffer_destroy_context(DivisionContext* ctx)
{
    DivisionVertexBufferSystemContext vertex_buffer_ctx = ctx->vertex_buffer_context;
    for (int i = 0; i < vertex_buffer_ctx.buffers_count; i++)
    {
        free(vertex_buffer_ctx.buffers[i].attributes);
    }

    free(vertex_buffer_ctx.buffers);
}

int32_t division_engine_vertex_buffer_create(
    DivisionContext* ctx, DivisionEngineVertexAttribute* attrs, int32_t attr_count, int32_t reserved_vertices_count)
{
    size_t vertex_data_size = 0;
    int overall_sizes[attr_count];

    GLuint gl_handle;
    glGenBuffers(1, &gl_handle);
    glBindBuffer(GL_ARRAY_BUFFER, gl_handle);

    DivisionVertexBufferSystemContext* vertex_ctx = &ctx->vertex_buffer_context;
    DivisionVertexBufferInternal_ vertex_buffer;
    vertex_buffer.attributes = malloc(sizeof(VertexAttributeInternal_) * attr_count);
    vertex_buffer.gl_buffer = gl_handle;
    vertex_buffer.attributes_count = attr_count;

    for (int i = 0; i < attr_count; i++)
    {
        DivisionEngineVertexAttribute at = attrs[i];
        GLenum gl_type;
        int base_size, number_of_components, overall_size, stride;

        division_attribute_get_traits(at.type, &gl_type, &base_size, &number_of_components);
        overall_size = base_size * number_of_components;

        if (i == 0)
        {
            stride = 0;
        }
        else
        {
            stride = overall_sizes[i - 1];
        }

        glVertexAttribPointer(at.index, number_of_components, gl_type, GL_FALSE, stride, NULL);

        overall_sizes[i] = overall_size;
        vertex_data_size += (size_t) overall_size;
        vertex_buffer.attributes[i] = (VertexAttributeInternal_) {
            .attr = at,
            .gl_type = gl_type,
            .stride = stride,
            .size_of_components = number_of_components
        };
    }

    int32_t buffers_count = vertex_ctx->buffers_count;
    vertex_ctx->buffers = realloc(vertex_ctx->buffers,
                                  (buffers_count + 1) * sizeof(DivisionVertexBufferInternal_));
    vertex_ctx->buffers[buffers_count] = vertex_buffer;
    vertex_ctx->buffers_count++;

    glBufferData(GL_ARRAY_BUFFER, (GLsizei) (vertex_data_size * reserved_vertices_count), NULL, GL_STATIC_DRAW);

    return vertex_ctx->buffers_count - 1;
}

// TODO: move to the render pass
void division_engine_vertex_buffer_draw_triangles(long buffer_id, size_t first_index, size_t count)
{
//    VertexBufferInternal_ buffer_data = gl_handlers_.items[buffer_id];
//
//    glBindVertexArray(buffer_data.vao_handler);
//    glBindBuffer(GL_ARRAY_BUFFER, buffer_data.vbo_handler);
    glDrawArrays(GL_TRIANGLES, (GLint) first_index, (GLsizei) count);
}

void division_attribute_get_traits(
    DivisionAttributeType attributeType, GLenum* gl_type, int32_t* base_size, int32_t* number_of_components)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT:
        {
            *gl_type = GL_FLOAT;
            *base_size = 4;
            *number_of_components = 1;
            break;
        }
        case DIVISION_DOUBLE:
        {
            *gl_type = GL_DOUBLE;
            *base_size = 8;
            *number_of_components = 1;
            break;
        }
        case DIVISION_INTEGER:
        {
            *gl_type = GL_INT;
            *base_size = 4;
            *number_of_components = 1;
            break;
        }
        case DIVISION_FVEC2:
        {
            *gl_type = GL_FLOAT;
            *base_size = 4;
            *number_of_components = 2;
            break;
        }
        case DIVISION_FVEC3:
        {
            *gl_type = GL_FLOAT;
            *base_size = 4;
            *number_of_components = 3;
            break;
        }
        case DIVISION_FVEC4:
        {
            *gl_type = GL_FLOAT;
            *base_size = 4;
            *number_of_components = 4;
            break;
        }
        case DIVISION_FMAT4X4:
        {
            *gl_type = GL_FLOAT;
            *base_size = 4;
            *number_of_components = 16;
            break;
        }
    };
}
