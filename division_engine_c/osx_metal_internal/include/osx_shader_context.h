#pragma once

#include <MetalKit/MetalKit.hpp>
#include <unordered_map>
#include <string>

typedef struct DivisionMetalAttribute {
    char* name;
    uint32_t index;
} DivisionMetalAttribute;

typedef struct DivisionMetalShaderProgram {
    MTL::RenderPipelineState* pipeline_state;
    MTL::Function* vertex_function;
    MTL::Function* fragment_function;
    DivisionMetalAttribute* attributes;
    int32_t attribute_count;
} DivisionMetalShaderProgram;

typedef struct DivisionShaderSystemContext {
    DivisionMetalShaderProgram* shader_programs;
    size_t shader_program_count;
} DivisionShaderSystemContext ;
