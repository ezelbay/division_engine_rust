#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <settings.h>
#include <state.h>

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);

typedef struct {
    void* _internal_data;
} DivisionEngineHandler;

bool division_engine_window_create(const DivisionEngineSettings* settings, DivisionEngineHandler* output_handler);
void division_engine_window_run_event_loop(DivisionEngineHandler handler, DivisionEngineUpdateFunc event_update_callback);
void division_engine_window_destroy(DivisionEngineHandler handler);