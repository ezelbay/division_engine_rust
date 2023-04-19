#pragma once

#include <stdint.h>

struct DivisionContext;

typedef void (*DivisionErrorFunc) (int, const char*);
typedef void (*DivisionLifecycleFunc)(struct DivisionContext* ctx);

typedef struct {
    uint32_t window_width;
    uint32_t window_height;
    const char* window_title;
    DivisionErrorFunc error_callback;
    DivisionLifecycleFunc init_callback;
    DivisionLifecycleFunc update_callback;
} DivisionSettings;