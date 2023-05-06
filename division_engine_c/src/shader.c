#include "division_engine/shader.h"
#include "division_engine/platform_internal/platfrom_shader.h"

#include <stdbool.h>

bool division_engine_shader_system_context_alloc(DivisionContext* ctx, const DivisionSettings* settings)
{
    return division_engine_internal_platform_shader_system_context_alloc(ctx, settings);
}

void division_engine_shader_system_context_free(DivisionContext* ctx)
{
    division_engine_internal_platform_shader_system_context_free(ctx);
}

int32_t division_engine_shader_program_alloc(DivisionContext* ctx)
{
    return division_engine_internal_platform_shader_program_alloc(ctx);
}


int32_t division_engine_shader_program_create(
    DivisionContext* ctx, const DivisionShaderSettings* settings, int32_t source_count)
{
    return division_engine_internal_platform_shader_program_create(ctx, settings, source_count);
}

// TODO: For the Metal API there is a need to set an entry point for the shader function.
// May be it's better to pass the shader func name as an additional argument?
bool division_engine_shader_from_file_attach_to_program(
    DivisionContext* ctx, const char* path, DivisionShaderType type, int32_t program_id)
{
    return division_engine_internal_platform_shader_from_file_attach_to_program(ctx, path, type, program_id);
}

bool division_engine_shader_from_source_attach_to_program(
    DivisionContext* ctx, const char* source, size_t source_size, DivisionShaderType type, int32_t program_id)
{
    return division_engine_internal_platform_shader_from_source_attach_to_program(
        ctx, source, source_size, type, program_id);
}

bool division_engine_shader_link_program(DivisionContext* ctx, int32_t program_id)
{
    return division_engine_internal_platform_shader_link_program(ctx, program_id);
}

void division_engine_shader_program_free(DivisionContext* ctx, int32_t program_id)
{
    division_engine_internal_platform_shader_program_free(ctx, program_id);
}

int32_t division_engine_shader_program_get_attribute_location(
    DivisionContext* ctx, const char* name, int32_t program_id)
{
    return division_engine_internal_platform_shader_program_get_attribute_location(ctx, name, program_id);
}

int32_t division_engine_shader_program_get_uniform_location(
    DivisionContext* ctx, const char* name, int32_t program_id)
{
    return division_engine_internal_platform_shader_program_get_uniform_location(ctx, name, program_id);
}

void division_engine_shader_program_get_uniform_float(
    DivisionContext* ctx, int32_t program_id, int32_t location, float* output_value)
{
    division_engine_internal_platform_shader_program_get_uniform_float(ctx, program_id, location, output_value);
}

void division_engine_shader_program_get_uniform_vec2(
    DivisionContext* ctx, int32_t program_id, int32_t location, float output_values[2])
{
    division_engine_internal_platform_shader_program_get_uniform_vec2(ctx, program_id, location, output_values);
}

void division_engine_shader_program_get_uniform_vec3(
    DivisionContext* ctx, int32_t program_id, int32_t location, float output_values[3])
{
    division_engine_internal_platform_shader_program_get_uniform_vec3(ctx, program_id, location, output_values);
}

void division_engine_shader_program_get_uniform_vec4(
    DivisionContext* ctx, int32_t program_id, int32_t location, float output_values[4])
{
    division_engine_internal_platform_shader_program_get_uniform_vec4(ctx, program_id, location, output_values);
}

void division_engine_shader_program_set_uniform_float(
    DivisionContext* ctx, int32_t program_id, int32_t location, float value)
{
    division_engine_internal_platform_shader_program_set_uniform_float(ctx, program_id, location, value);
}

void division_engine_shader_program_set_uniform_vec2(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float values[2])
{
    division_engine_internal_platform_shader_program_set_uniform_vec2(ctx, program_id, location, values);
}

void division_engine_shader_program_set_uniform_vec3(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float values[3])
{
    division_engine_internal_platform_shader_program_set_uniform_vec3(ctx, program_id, location, values);
}

void division_engine_shader_program_set_uniform_vec4(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float values[4])
{
    division_engine_internal_platform_shader_program_set_uniform_vec4(ctx, program_id, location, values);
}
