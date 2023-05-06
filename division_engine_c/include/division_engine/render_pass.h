#pragma once

#include <stdint.h>

#include "division_engine_c_export.h"
#include "context.h"

typedef struct DivisionRenderPass {
    int32_t* uniform_buffers;
    int32_t uniform_buffer_count;
    int32_t vertex_buffer;
    int32_t first_vertex;
    int32_t vertex_count;
    int32_t shader_program;
} DivisionRenderPass;

typedef struct DivisionRenderPassSystemContext {
    DivisionRenderPass* render_passes;
    int32_t render_pass_count;
} DivisionRenderPassSystemContext;

bool division_engine_internal_render_pass_context_alloc(
    DivisionContext* ctx, const DivisionSettings* settings);

void division_engine_internal_render_pass_context_free(DivisionContext* ctx);

#ifdef __cplusplus
extern "C" {
#endif

DIVISION_EXPORT int32_t division_engine_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass render_pass);

#ifdef __cplusplus
}
#endif