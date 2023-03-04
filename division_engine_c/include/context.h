#pragma once

#include <stdbool.h>

#include "renderer.h"
#include "shader.h"
#include "settings.h"

typedef struct {
    DivisionEngineErrorFunc error_callback;
    DivisionRendererContext renderer_context;
} DivisionContext;

bool division_engine_context_create(const DivisionEngineSettings* settings, DivisionContext** output_context);
void division_engine_context_destroy(DivisionContext* context);