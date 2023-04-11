#pragma once

#include <stdint.h>
#include <stddef.h>

#include "division_engine/context.h"

typedef enum {
    DIVISION_TOPOLOGY_TRIANGLES = 0,
    DIVISION_TOPOLOGY_POINTS = 1,
    DIVISION_TOPOLOGY_LINES = 2
} DivisionRenderTopology;

typedef struct DivisionRenderPass {
    int32_t vertex_buffer;
    int32_t shader_program;
    DivisionRenderTopology topology;
} DivisionRenderPass;

typedef struct DivisionRenderPassSystemContext {
    DivisionRenderPass* render_passes;
    int32_t render_pass_count;
} DivisionRenderPassSystemContext;

int32_t division_engine_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass pass);

bool division_engine_internal_render_pass_context_alloc(DivisionContext* ctx);
void division_engine_internal_render_pass_context_free(DivisionContext* ctx);
