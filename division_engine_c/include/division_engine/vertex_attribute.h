#pragma once
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
    DIVISION_FLOAT,
    DIVISION_DOUBLE,
    DIVISION_INTEGER,
    DIVISION_FVEC2,
    DIVISION_FVEC3,
    DIVISION_FVEC4,
    DIVISION_FMAT4X4
} DivisionAttributeType;

typedef struct {
    DivisionAttributeType type;
    int32_t index;
} DivisionEngineVertexAttribute;
