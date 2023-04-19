#pragma once

#include "division_engine/context.h"
#include "division_engine/settings.h"

bool division_engine_internal_platform_renderer_alloc(DivisionContext* ctx, const DivisionSettings* settings);
void division_engine_internal_platform_renderer_free(DivisionContext* ctx);

void division_engine_internal_platform_renderer_run_loop(DivisionContext* ctx, const DivisionSettings* settings);
