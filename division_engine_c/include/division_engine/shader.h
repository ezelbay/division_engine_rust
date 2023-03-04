#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    DivisionEngineShaderVertex = 0,
    DivisionEngineShaderFragment = 1
} DivisionEngineShaderType;

int32_t division_engine_shader_create_program();
bool division_engine_shader_attach_to_program(const char* path, DivisionEngineShaderType type, int32_t program_id);
bool division_engine_shader_link_program(int32_t program_id);
void division_engine_shader_use_program(int32_t program_id);
void division_engine_shader_destroy_program(int32_t program_id);