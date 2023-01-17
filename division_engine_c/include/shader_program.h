#pragma once

#include <stdint.h>

typedef enum {
    DivisionEngineShaderVertex = 0,
    DivisionEngineShaderFragment = 1
} DivisionEngineShaderType;

int32_t create_shader(const char* path, DivisionEngineShaderType type);