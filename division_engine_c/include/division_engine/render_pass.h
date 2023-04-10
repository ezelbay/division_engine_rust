#pragma once

#include <stdint.h>
#include <stddef.h>

#include "division_engine/context.h"

typedef enum {
    DIVISION_TOPOLOGY_TRIANGLES,
    DIVISION_TOPOLOGY_POINTS,
    DIVISION_TOPOLOGY_LINES
} DivisionRenderTopology;

typedef struct DivisionRenderPass {
    size_t first_vertex;
    size_t vertex_count;
    int32_t vertex_buffer;
    int32_t shader_program;
    DivisionRenderTopology topology;
} DivisionRenderPass;

int32_t division_engine_render_pass_alloc(DivisionContext* ctx, DivisionRenderPass pass);

bool division_engine_internal_render_pass_context_alloc(DivisionContext* ctx);
void division_engine_internal_render_pass_context_free(DivisionContext* ctx);
