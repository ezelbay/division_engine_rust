#pragma once

#include <stddef.h>

#include "division_engine/context.h"
#include "division_engine/settings.h"

#include <division_engine_c_export.h>

#ifdef __cplusplus
extern "C" {
#endif

DIVISION_EXPORT bool division_engine_internal_platform_vertex_buffer_context_alloc(
    DivisionContext* ctx, const DivisionSettings* settings);

DIVISION_EXPORT void division_engine_internal_platform_vertex_buffer_alloc(DivisionContext* ctx);
DIVISION_EXPORT void division_engine_internal_platform_vertex_buffer_context_free(DivisionContext* ctx);

DIVISION_EXPORT void division_engine_internal_platform_vertex_buffer_set_vertex_data(
    DivisionContext* ctx,
    int32_t vertex_buffer,
    int32_t object_index,
    int32_t attribute_index,
    const void* data_ptr,
    size_t first_vertex_index,
    size_t vertex_count
);

DIVISION_EXPORT void division_engine_internal_platform_vertex_buffer_draw(DivisionContext* ctx);

#ifdef __cplusplus
}
#endif