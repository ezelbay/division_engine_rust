#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <settings.h>

#include "context.h"
#include "state.h"
#include "color.h"

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);

void division_engine_renderer_run_loop(DivisionContext* ctx, DivisionEngineUpdateFunc update_callback);
void division_engine_renderer_destroy(DivisionContext* ctx);

bool division_engine_internal_renderer_create(
    DivisionContext* renderer_context, const DivisionEngineSettings* settings);