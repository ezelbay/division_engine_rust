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
int32_t division_engine_shader_program_get_uniform_location(const char* name, int32_t program_id);

void division_engine_shader_program_get_uniform_float(int32_t program_id, int32_t location, float* output_value);
void division_engine_shader_program_get_uniform_vec2(int32_t program_id, int32_t location, float output_values[2]);
void division_engine_shader_program_get_uniform_vec3(int32_t program_id, int32_t location, float output_values[3]);
void division_engine_shader_program_get_uniform_vec4(int32_t program_id, int32_t location, float output_values[4]);

void division_engine_shader_program_set_uniform_float(int32_t program_id, int32_t location, float value);
void division_engine_shader_program_set_uniform_vec2(int32_t program_id, int32_t location, const float values[2]);
void division_engine_shader_program_set_uniform_vec3(int32_t program_id, int32_t location, const float values[3]);
void division_engine_shader_program_set_uniform_vec4(int32_t program_id, int32_t location, const float values[4]);
