#pragma once

#include <stdbool.h>

#include "color.h"
#include "settings.h"
#include "state.h"

struct DivisionVertexBufferInternal_;
struct DivisionRendererSystemContext;
struct DivisionVertexBufferSystemContext;
struct DivisionRenderPassSystemContext;

typedef struct {
    DivisionEngineState state;

    DivisionEngineErrorFunc error_callback;
    struct DivisionRendererSystemContext* renderer_context;
    struct DivisionVertexBufferSystemContext* vertex_buffer_context;
    struct DivisionRenderPassSystemContext* render_pass_context;

    void* user_data;
} DivisionContext;

bool division_engine_context_alloc(const DivisionEngineSettings* settings, DivisionContext** output_context);
void division_engine_context_free(DivisionContext* ctx);