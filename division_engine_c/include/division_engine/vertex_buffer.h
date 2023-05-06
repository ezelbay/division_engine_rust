#pragma once

#include <stdlib.h>
#include <stdbool.h>

#include "context.h"
#include "shader.h"

#include <division_engine_c_export.h>

typedef enum {
    DIVISION_TOPOLOGY_TRIANGLES = 0,
    DIVISION_TOPOLOGY_POINTS = 1,
    DIVISION_TOPOLOGY_LINES = 2
} DivisionRenderTopology;

typedef struct DivisionVertexAttributeSettings {
    DivisionShaderVariableType type;
    int32_t location;
} DivisionVertexAttributeSettings;

typedef struct DivisionVertexAttribute {
    int32_t location;
    int32_t offset;
    int32_t base_size;
    int32_t component_count;
    DivisionShaderVariableType type;
} DivisionVertexAttribute;

typedef struct DivisionVertexBuffer {
    struct VertexAttributeInternalPlatform_* attributes_impl;
    DivisionVertexAttribute* attributes;
    int32_t attribute_count;
    int32_t vertex_count;
    size_t per_vertex_data_size;
    DivisionRenderTopology topology;
} DivisionVertexBuffer;

typedef struct DivisionVertexBufferObjects {
    int32_t* objects_start_vertex;
    int32_t* objects_vertex_count;
    int32_t objects_count;
} DivisionVertexBufferObjects;

typedef struct DivisionRenderPass {
    int32_t* uniform_buffers;
    int32_t uniform_buffer_count;
    int32_t vertex_buffer;
    int32_t shader_program;
} DivisionRenderPass;

typedef struct DivisionVertexBufferSystemContext {
    DivisionVertexBuffer* buffers;
    struct DivisionVertexBufferInternalPlatform_* buffers_impl;
    DivisionVertexBufferObjects* buffers_objects;
    DivisionRenderPass* render_passes;

    int32_t buffers_count;
    int32_t render_pass_count;
} DivisionVertexBufferSystemContext;


bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx, const DivisionSettings* settings);
void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx);

#ifdef __cplusplus
extern "C" {
#endif


DIVISION_EXPORT int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx,
    DivisionVertexAttributeSettings* attrs,
    int32_t attr_count,
    int32_t vertex_count,
    DivisionRenderTopology render_topology);

DIVISION_EXPORT void division_engine_vertex_buffer_set_vertex_data_for_attribute(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t object_index,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
);

DIVISION_EXPORT void* division_engine_vertex_buffer_borrow_data_pointer(DivisionContext* ctx, int32_t vertex_buffer);
DIVISION_EXPORT void division_engine_vertex_buffer_return_data_pointer(
    DivisionContext* ctx, int32_t vertex_buffer, void* data_pointer);

DIVISION_EXPORT int32_t division_engine_vertex_buffer_render_pass_alloc(
    DivisionContext* ctx, DivisionRenderPass render_pass);

#ifdef __cplusplus
}
#endif