#pragma once

#include <stdlib.h>
#include <stdbool.h>

#include "context.h"
#include "vertex_attribute.h"

bool division_engine_internal_vertex_buffer_context_alloc(DivisionContext* ctx);
void division_engine_internal_vertex_buffer_context_free(DivisionContext* ctx);

int32_t division_engine_vertex_buffer_alloc(
    DivisionContext* ctx, DivisionEngineVertexAttribute* attrs, int32_t attr_count, int32_t vertex_count);

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