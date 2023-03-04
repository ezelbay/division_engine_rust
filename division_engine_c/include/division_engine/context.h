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
    DivisionEngineErrorFunc error_callback;
    DivisionRendererSystemContext renderer_context;
    DivisionVertexBufferSystemContext vertex_buffer_context;
} DivisionContext;

bool division_engine_context_create(const DivisionEngineSettings* settings, DivisionContext** output_context);
void division_engine_context_destroy(DivisionContext* ctx);