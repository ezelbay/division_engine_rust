#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <settings.h>
#include <state.h>

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);

typedef struct {
    void* window_data;
} DivisionRendererContext;

void division_engine_renderer_run_loop(DivisionRendererContext* renderer_context, DivisionEngineUpdateFunc update_callback);
void division_engine_renderer_destroy(DivisionRendererContext* renderer_context);

bool division_engine_internal_renderer_create(
    DivisionRendererContext* renderer_context, const DivisionEngineSettings* settings);