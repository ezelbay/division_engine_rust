#include <stdio.h>
#include "division_engine/color.h"
#include "division_engine/renderer.h"
#include "division_engine/shader.h"
#include "division_engine/vertex_buffer.h"

void error_callback(int error_code, const char* message);
void init_callback(DivisionContext* ctx);
void update_callback(DivisionContext* ctx);

typedef struct UserData {
    int32_t shader_id;
} UserData;

int main()
{
    DivisionSettings settings = {
        .window_title = "New window",
        .window_width = 512,
        .window_height = 512,
        .init_callback = init_callback,
        .update_callback = update_callback,
        .error_callback = error_callback,
    };

    DivisionContext* ctx = NULL;
    division_engine_context_alloc(&settings, &ctx);

    division_engine_renderer_run_loop(ctx, &settings);
    division_engine_context_free(ctx);
}

void init_callback(DivisionContext* ctx)
{
#ifdef __APPLE__
    DivisionShaderSettings settings[] = {
        (DivisionShaderSettings) {
            .type = DIVISION_SHADER_VERTEX,
            .entry_point_name = "vertexMain",
            .file_path = "test.metal"
        },
        (DivisionShaderSettings) {
            .type = DIVISION_SHADER_FRAGMENT,
            .entry_point_name = "fragmentMain",
            .file_path = "test.metal"
        }
    };

    int32_t shader_program = division_engine_shader_program_create(ctx, settings, 2);
#else
    // TODO: Rework for GLFW as for OSX
    int32_t shader_program = division_engine_shader_program_alloc(ctx);
    division_engine_shader_from_file_attach_to_program(ctx, "test.vert", DIVISION_SHADER_VERTEX, shader_program);
    division_engine_shader_from_file_attach_to_program(ctx, "test.frag", DIVISION_SHADER_FRAGMENT, shader_program);
#endif

    division_engine_shader_link_program(ctx, shader_program);

    int32_t posLocation = division_engine_shader_program_get_attribute_location(ctx, "pos", shader_program);
    int32_t fColorLocation = division_engine_shader_program_get_attribute_location(ctx, "fColor", shader_program);

    DivisionVertexAttributeSettings attr[2] = {
        {.type = DIVISION_FVEC3, .location = 0},
        {.type = DIVISION_FVEC4, .location = 0}
    };
    int32_t vertex_buffer = division_engine_vertex_buffer_alloc(ctx, attr, 2, 3, DIVISION_TOPOLOGY_TRIANGLES);

    float positions[9] = {
        -0.5f, -0.5f, 0,
        -1, 0, 0,
        1, 1, 0
    };

    float colors[12] = {
        1, 1, 1, 1,
        1, 1, 1, 1,
        1, 1, 1, 1
    };

    int32_t objectIndex = 0;
    division_engine_vertex_buffer_set_vertex_data_for_attribute(
        ctx, vertex_buffer, objectIndex, 0, positions, 0, 3);
    division_engine_vertex_buffer_set_vertex_data_for_attribute(
        ctx, vertex_buffer, objectIndex, 1, colors, 0, 3);

    division_engine_vertex_buffer_render_pass_alloc(ctx, (DivisionRenderPass) {
        .vertex_buffer = vertex_buffer,
        .shader_program = shader_program,
    });

    int32_t uniform_id = division_engine_shader_program_get_uniform_location(ctx, "TestColor", shader_program);
    float testVec[] = { 1, 0, 1, 1 };
    division_engine_shader_program_set_uniform_vec4(ctx, shader_program, uniform_id, testVec);

    float outputTestVec[4];
    division_engine_shader_program_get_uniform_vec4(ctx, shader_program, uniform_id, outputTestVec);
    printf("TestVec location: %d, Output values are: { %f, %f, %f, %f }",
        uniform_id, outputTestVec[0], outputTestVec[1], outputTestVec[2], outputTestVec[3]
    );
}

void update_callback(DivisionContext* ctx)
{
}

void error_callback(int error_code, const char* message)
{
    fprintf(stderr, "Error code: %d, error message: %s\n", error_code, message);
}