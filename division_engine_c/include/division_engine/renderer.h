#pragma once

#include <stdint.h>
#include <stdbool.h>
#include "settings.h"

#include "context.h"
#include "state.h"
#include "color.h"

typedef void(*DivisionEngineUpdateFunc)(DivisionContext* ctx);

void division_engine_renderer_run_loop(DivisionContext* ctx, DivisionEngineUpdateFunc update_callback);

bool division_engine_internal_renderer_context_alloc(
    DivisionContext* renderer_context, const DivisionEngineSettings* settings);

void division_engine_internal_renderer_context_free(DivisionContext* ctx);