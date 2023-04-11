#pragma once

#include <stdint.h>

typedef void (*DivisionErrorFunc) (int, const char*);

typedef struct {
    uint32_t window_width;
    uint32_t window_height;
    const char* window_title;
    DivisionErrorFunc error_callback;
} DivisionSettings;