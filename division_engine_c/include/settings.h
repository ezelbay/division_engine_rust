#pragma once

#include <stdint.h>

typedef void (*DivisionEngineErrorFunc) (int, const char*);

typedef struct {
    uint32_t window_width;
    uint32_t window_height;
    const char* window_title;
    DivisionEngineErrorFunc error_callback;
} DivisionEngineSettings;