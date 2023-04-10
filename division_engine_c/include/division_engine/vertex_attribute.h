#pragma once

#include <stdlib.h>
#include <stdbool.h>

#include "variable_type.h"

typedef struct {
    DivisionVariableType type;
    int32_t location;
} DivisionEngineVertexAttribute;

int32_t division_engine_attribute_get_location(const char* name, int32_t shader_program);