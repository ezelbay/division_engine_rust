#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <settings.h>
#include <state.h>

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);
typedef void (*DivisionEngineErrorFunc) (int, const char*);

bool division_engine_renderer_create(const DivisionEngineSettings* settings, DivisionEngineErrorFunc error_callback);
void division_engine_renderer_run_loop(DivisionEngineUpdateFunc update_callback);
void division_engine_renderer_destroy();