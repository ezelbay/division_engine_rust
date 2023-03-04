#pragma once

#include <stdbool.h>

#include "color.h"
#include "settings.h"
#include "state.h"

typedef struct {
    DivisionColor clear_color;
    void* window_data;
} DivisionRendererContext;

typedef struct {
    DivisionEngineErrorFunc error_callback;
    DivisionRendererContext renderer_context;
} DivisionContext;

bool division_engine_context_create(const DivisionEngineSettings* settings, DivisionContext** output_context);
void division_engine_context_destroy(DivisionContext* ctx);