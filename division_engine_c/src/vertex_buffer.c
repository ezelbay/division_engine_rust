#include "division_engine/vertex_buffer.h"
#include "division_engine/platform_internal/platform_vertex_buffer.h"

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

typedef struct AttrTraits_
{
    int32_t base_size;
    int32_t component_count;
} AttrTraits_;

static inline AttrTraits_ division_attribute_get_traits(DivisionShaderVariableType attributeType);

static inline void gather_attributes_info(
    DivisionVertexAttributeSettings* attrs,
    int32_t attr_count,
    DivisionVertexAttribute* attributes,
    int32_t* per_vertex_data_size
                                         );

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx, const DivisionSettings* settings)
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

    return division_engine_internal_platform_vertex_buffer_context_alloc(ctx, settings);
}

void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx)
{
    division_engine_internal_platform_vertex_buffer_context_free(ctx);

    DivisionVertexBufferSystemContext* vertex_buffer_ctx = ctx->vertex_buffer_context;

    for (int i = 0; i < vertex_buffer_ctx->buffers_count; i++)
    {
        DivisionVertexBuffer* buffer = &vertex_buffer_ctx->buffers[i];
        DivisionVertexBufferObjects* buffer_objects = &vertex_buffer_ctx->buffers_objects[i];

        free(buffer->attributes);
        free(buffer_objects->objects_start_vertex);
        free(buffer_objects->objects_vertex_count);
    }

    for (int i = 0; i < vertex_buffer_ctx->render_pass_count; i++)
    {
        free(vertex_buffer_ctx->render_passes[i].uniform_buffers);
    }

    free(vertex_buffer_ctx->buffers);
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
    DivisionVertexBufferSystemContext* vertex_ctx = ctx->vertex_buffer_context;

    DivisionVertexBuffer vertex_buffer = {
        .attributes = malloc(sizeof(DivisionVertexAttribute) * attr_count),
        .attribute_count = attr_count,
    };

    DivisionVertexBufferObjects vertex_buffer_objects = {
        .objects_start_vertex = malloc(sizeof(int32_t)),
        .objects_vertex_count = malloc(sizeof(int32_t)),
        .objects_count = 1
    };
    vertex_buffer_objects.objects_start_vertex[0] = 0;
    vertex_buffer_objects.objects_vertex_count[0] = vertex_count;

    int32_t per_vertex_data_size;
    gather_attributes_info(
        attrs, attr_count, vertex_buffer.attributes, &per_vertex_data_size);

    vertex_buffer.vertex_count = vertex_count;
    vertex_buffer.per_vertex_data_size = per_vertex_data_size;
    vertex_buffer.topology = render_topology;

    int32_t buffers_count = vertex_ctx->buffers_count;
    int32_t new_buffers_count = buffers_count + 1;
    vertex_ctx->buffers = realloc(vertex_ctx->buffers, new_buffers_count * sizeof(DivisionVertexBuffer));
    vertex_ctx->buffers_objects = realloc(
        vertex_ctx->buffers_objects, new_buffers_count * sizeof(DivisionVertexBufferObjects));

    vertex_ctx->buffers[buffers_count] = vertex_buffer;
    vertex_ctx->buffers_objects[buffers_count] = vertex_buffer_objects;
    vertex_ctx->buffers_count++;

    division_engine_internal_platform_vertex_buffer_alloc(ctx);

    return vertex_ctx->buffers_count - 1;
}

void gather_attributes_info(
    DivisionVertexAttributeSettings* attrs,
    int32_t attr_count,
    DivisionVertexAttribute* attributes,
    int32_t* per_vertex_data_size
                           )
{
    *per_vertex_data_size = 0;

    for (int32_t i = 0; i < attr_count; i++)
    {
        DivisionVertexAttributeSettings at = attrs[i];
        AttrTraits_ attr_traits = division_attribute_get_traits(at.type);

        int32_t attr_size = attr_traits.base_size * attr_traits.component_count;
        int32_t offset = *per_vertex_data_size;
        *per_vertex_data_size += attr_size;

        attributes[i] = (DivisionVertexAttribute) {
            .location = at.location,
            .offset = offset,
            .base_size = attr_traits.base_size,
            .component_count = attr_traits.component_count,
            .type = at.type
        };
    }
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
    DivisionVertexBufferObjects vb_objs = vertex_buffer_context->buffers_objects[vertex_buffer];
    size_t obj_vertex_count = vb_objs.objects_vertex_count[object_index];

    if (vertex_count > obj_vertex_count)
    {
        vertex_count = obj_vertex_count;
        fprintf(stderr, "Vertex count overflow. Available: %zu, but set: %zu", obj_vertex_count, vertex_count);
    }

    division_engine_internal_platform_vertex_buffer_set_vertex_data(
        ctx, vertex_buffer, object_index, attribute_index, data_ptr, first_vertex_index, vertex_count);
}

AttrTraits_ division_attribute_get_traits(DivisionShaderVariableType attributeType)
{
    switch (attributeType)
    {
        case DIVISION_FLOAT:
            return (AttrTraits_) {4, 1};
        case DIVISION_DOUBLE:
            return (AttrTraits_) {8, 1};
        case DIVISION_INTEGER:
            return (AttrTraits_) {4, 1};
        case DIVISION_FVEC2:
            return (AttrTraits_) {4, 2};
        case DIVISION_FVEC3:
            return (AttrTraits_) {4, 3};
        case DIVISION_FVEC4:
            return (AttrTraits_) {4, 4};
        case DIVISION_FMAT4X4:
            return (AttrTraits_) {4, 16};
        default:
        {
            fprintf(stderr, "Unknown attribute type");
        }
    }
}

int32_t division_engine_vertex_buffer_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass render_pass)
{
    DivisionVertexBufferSystemContext* pass_ctx = ctx->vertex_buffer_context;

    DivisionRenderPass render_pass_copy = render_pass;
    size_t uniform_buffers_size = sizeof(int32_t) * render_pass.uniform_buffer_count;
    render_pass_copy.uniform_buffers = malloc(uniform_buffers_size);
    render_pass_copy.uniform_buffer_count = render_pass.uniform_buffer_count;
    memcpy(render_pass_copy.uniform_buffers, render_pass.uniform_buffers, uniform_buffers_size);

    int32_t render_pass_count = pass_ctx->render_pass_count;
    int32_t new_render_pass_count = render_pass_count + 1;
    pass_ctx->render_passes = realloc(pass_ctx->render_passes, sizeof(DivisionRenderPass) * new_render_pass_count);
    pass_ctx->render_passes[render_pass_count] = render_pass_copy;
    pass_ctx->render_pass_count++;

    return pass_ctx->render_pass_count - 1;
}