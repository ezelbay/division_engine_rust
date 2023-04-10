#pragma once

#include <stdbool.h>

#include "color.h"
#include "settings.h"
#include "state.h"
#include "vertex_attribute.h"

struct DivisionVertexBufferInternal_;

typedef struct {
    DivisionColor clear_color;
    void* window_data;
} DivisionRendererSystemContext;

typedef struct {
    struct DivisionVertexBufferInternal_* buffers;
    int32_t buffers_count;
} DivisionVertexBufferSystemContext;

typedef struct {
    struct DivisionRenderPass* render_passes;
    int32_t render_pass_count;
} DivisionRenderPassSystemContext;

typedef struct {
    DivisionEngineErrorFunc error_callback;
    DivisionRendererSystemContext renderer_context;
    DivisionVertexBufferSystemContext vertex_buffer_context;
    DivisionRenderPassSystemContext render_pass_context;
    DivisionEngineState state;

    void* user_data;
} DivisionContext;

bool division_engine_context_alloc(const DivisionEngineSettings* settings, DivisionContext** output_context);
void division_engine_context_free(DivisionContext* ctx);