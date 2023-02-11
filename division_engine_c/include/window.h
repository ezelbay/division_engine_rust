#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <settings.h>
#include <state.h>

typedef void(*DivisionEngineUpdateFunc)(DivisionEngineState);
typedef void (*DivisionEngineErrorFunc) (int, const char*);

bool division_engine_start_session(DivisionEngineErrorFunc error_callback);
int32_t division_engine_window_create(const DivisionEngineSettings* settings);
void division_engine_window_run_event_loop(int32_t window_id, DivisionEngineUpdateFunc event_update_callback);
void division_engine_window_destroy(int32_t window_id);
void division_engine_finish_session();