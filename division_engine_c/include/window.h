#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <division_engine_settings.h>
#include <division_engine_state.h>

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);

typedef struct {
    void* _internal_data;
} DivisionEngineHandler;

bool division_engine_create_window(const DivisionEngineSettings* settings, DivisionEngineHandler* output_handler);
void division_engine_run_event_loop(DivisionEngineHandler handler, DivisionEngineUpdateFunc event_update_callback);
void division_engine_destroy_window(DivisionEngineHandler handler);