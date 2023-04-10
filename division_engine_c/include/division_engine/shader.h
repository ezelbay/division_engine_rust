#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

typedef enum {
    DIVISION_SHADER_VERTEX = 0,
    DIVISION_SHADER_FRAGMENT = 1
} DivisionEngineShaderType;

int32_t division_engine_shader_program_alloc();
bool division_engine_shader_from_file_attach_to_program(
    const char* path, DivisionEngineShaderType type, int32_t program_id);
bool division_engine_shader_from_source_attach_to_program(
    const char* source, size_t source_size, DivisionEngineShaderType type, int32_t program_id);

bool division_engine_shader_link_program(int32_t program_id);
void division_engine_shader_program_free(int32_t program_id);