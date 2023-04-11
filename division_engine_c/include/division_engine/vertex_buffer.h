#pragma once

#include <stdlib.h>
#include <stdbool.h>

#include "context.h"
#include "shader.h"

struct VertexAttributeInternalImpl_;
struct DivisionVertexBufferInternalImpl_;

typedef struct DivisionVertexAttributeSettings {
    DivisionShaderVariableType type;
    int32_t location;
} DivisionVertexAttributeSettings;

typedef struct DivisionVertexAttribute {
    int32_t location;
    int32_t offset;
    int32_t base_size;
    int32_t component_count;
} DivisionVertexAttribute;

typedef struct DivisionVertexBuffer {
    struct VertexAttributeInternalImpl_* attributes_impl;
    DivisionVertexAttribute* attributes;
    int32_t attribute_count;

    int32_t* objects_start_vertex;
    int32_t* objects_vertex_count;
    int32_t objects_count;
    int32_t vertex_count;

    size_t per_vertex_data_size;
} DivisionVertexBuffer;

typedef struct DivisionVertexBufferSystemContext {
    struct DivisionVertexBuffer* buffers;
    struct DivisionVertexBufferInternalImpl_* buffers_impl;
    int32_t buffers_count;
} DivisionVertexBufferSystemContext;

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx);
void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx);

int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx, DivisionVertexAttributeSettings* attrs, int32_t attr_count, int32_t vertex_count);

void division_engine_vertex_buffer_set_vertex_data_for_attribute(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t object_index,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
);

void division_engine_internal_vertex_buffer_draw(DivisionContext* ctx);