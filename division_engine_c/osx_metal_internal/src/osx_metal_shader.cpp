#include "division_engine/platform_internal/platfrom_shader.h"

#include <MetalKit/MetalKit.hpp>
#include <NSUtils.h>
#include "division_engine/renderer.h"
#include "division_engine/shader.h"
#include "osx_window_context.h"
#include "osx_shader_context.h"

bool division_engine_internal_platform_shader_system_context_alloc(
    DivisionContext* ctx, const DivisionSettings* settings)
{
    auto* shader_context = static_cast<DivisionShaderSystemContext*>(malloc(sizeof(DivisionShaderSystemContext)));
    shader_context->shader_programs = nullptr;
    shader_context->shader_program_count = 0;

    ctx->shader_context = shader_context;

    return true;
}

int32_t division_engine_internal_platform_shader_program_create(
    DivisionContext* ctx, const DivisionShaderSettings* settings, int32_t source_count)
{
    auto* window_ctx = static_cast<DivisionOSXWindowContext*>(ctx->renderer_context->window_data);
    auto* shader_ctx = ctx->shader_context;
    auto pipeline_state_count = shader_ctx->shader_program_count;

    shader_ctx->shader_programs = static_cast<DivisionMetalShaderProgram*>(realloc(
        shader_ctx->shader_programs,
        sizeof(DivisionMetalShaderProgram) * (pipeline_state_count + 1)
    ));

    window_ctx->app_delegate->viewDelegate->createShaderProgram(
        settings, source_count, &shader_ctx->shader_programs[pipeline_state_count]);

    return static_cast<int32_t>(pipeline_state_count);
}

void division_engine_internal_platform_shader_system_context_free(DivisionContext* ctx)
{
    DivisionShaderSystemContext* shader_context = ctx->shader_context;
    auto* window_context = static_cast<DivisionOSXWindowContext*>(ctx->renderer_context->window_data);
    DivisionOSXViewDelegate* view_delegate = window_context->app_delegate->viewDelegate;

    for (int32_t i; i < shader_context->shader_program_count; i++)
    {
        view_delegate->deleteShaderProgram(&shader_context->shader_programs[i]);
    }
    free(shader_context->shader_programs);
    free(shader_context);
}

int32_t division_engine_internal_platform_shader_program_alloc(DivisionContext* ctx)
{
    return 0;
}

void division_engine_internal_platform_shader_program_free(DivisionContext* ctx, int32_t program_id)
{
    // TODO: Not implemented
}

bool division_engine_internal_platform_shader_from_file_attach_to_program(
    DivisionContext* ctx, const char* path, DivisionShaderType type, int32_t program_id)
{
    return false;
}

bool division_engine_internal_platform_shader_from_source_attach_to_program(
    DivisionContext* ctx, const char* source, size_t source_size, DivisionShaderType type, int32_t program_id)
{
    return false;
}

bool division_engine_internal_platform_shader_link_program(DivisionContext* ctx, int32_t program_id)
{
    return false;
}

int32_t division_engine_internal_platform_shader_program_get_attribute_location(
    DivisionContext* ctx, const char* name, int32_t program_id)
{
    DivisionMetalShaderProgram* shader = &ctx->shader_context->shader_programs[program_id];
    for (int i = 0; i < shader->attribute_count; ++i)
    {
        DivisionMetalAttribute attribute = shader->attributes[i];
        if (strcmp(attribute.name, name) == 0) return static_cast<int32_t>(attribute.index);
    }
    
    return -1;
}

int32_t division_engine_internal_platform_shader_program_get_uniform_location(
    DivisionContext* ctx, const char* name, int32_t program_id)
{
    DivisionMetalShaderProgram* shader = &ctx->shader_context->shader_programs[program_id];
    NS::String* ns_name = NSUtils::createUtf8String(name);

    auto* constant = shader->vertex_function->functionConstantsDictionary()->object<MTL::FunctionConstant>(ns_name);
    if (constant == nullptr)
    {
        constant = shader->fragment_function->functionConstantsDictionary()->object<MTL::FunctionConstant>(ns_name);
    }

    return constant != nullptr ? static_cast<int32_t>(constant->index()) : -1;
}

void division_engine_internal_platform_shader_program_get_uniform_float(
    DivisionContext* ctx, int32_t program_id, int32_t location, float* output_value)
{
    DivisionMetalShaderProgram* shader = &ctx->shader_context->shader_programs[program_id];
}

void division_engine_internal_platform_shader_program_get_uniform_vec2(
    DivisionContext* ctx, int32_t program_id, int32_t location, float* output_values)
{

}

void division_engine_internal_platform_shader_program_get_uniform_vec3(
    DivisionContext* ctx, int32_t program_id, int32_t location, float* output_values)
{

}

void division_engine_internal_platform_shader_program_get_uniform_vec4(
    DivisionContext* ctx, int32_t program_id, int32_t location, float* output_values)
{
}

void division_engine_internal_platform_shader_program_set_uniform_float(
    DivisionContext* ctx, int32_t program_id, int32_t location, float value)
{

}

void division_engine_internal_platform_shader_program_set_uniform_vec2(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float* values)
{

}

void division_engine_internal_platform_shader_program_set_uniform_vec3(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float* values)
{

}

void division_engine_internal_platform_shader_program_set_uniform_vec4(
    DivisionContext* ctx, int32_t program_id, int32_t location, const float* values)
{

}
