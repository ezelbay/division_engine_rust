#pragma once

#include <stdint.h>

typedef enum {
    DivisionEngineShaderVertex = 0,
    DivisionEngineShaderFragment = 1
} DivisionEngineShaderType;

int32_t division_engine_shader_create(const char* path, DivisionEngineShaderType type);