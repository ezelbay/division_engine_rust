#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

typedef enum {
    DIVISION_SHADER_VERTEX = 0,
    DIVISION_SHADER_FRAGMENT = 1
} DivisionShaderType;

typedef enum {
    DIVISION_FLOAT = 0,
    DIVISION_DOUBLE = 1,
    DIVISION_INTEGER = 2,
    DIVISION_FVEC2 = 3,
    DIVISION_FVEC3 = 4,
    DIVISION_FVEC4 = 5,
    DIVISION_FMAT4X4 = 6
} DivisionShaderVariableType;

int32_t division_engine_shader_program_alloc();
void division_engine_shader_program_free(int32_t program_id);

bool division_engine_shader_from_file_attach_to_program(
    const char* path, DivisionShaderType type, int32_t program_id);
bool division_engine_shader_from_source_attach_to_program(
    const char* source, size_t source_size, DivisionShaderType type, int32_t program_id);

bool division_engine_shader_link_program(int32_t program_id);

int32_t division_engine_shader_program_get_attribute_location(const char* name, int32_t program_id);