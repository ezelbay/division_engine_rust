#include <stdio.h>
#include <string.h>
#include "division_engine/renderer.h"
#include "division_engine/shader.h"
#include "division_engine/vertex_buffer.h"
#include "division_engine/render_pass.h"

void error_callback(int error_code, const char* message);
void update_callback(DivisionContext* ctx);

int main()
{
    DivisionSettings settings = {
        .error_callback = error_callback,
        .window_title = "New window",
        .window_width = 512,
        .window_height = 512
    };

    DivisionContext* context = NULL;
    division_engine_context_alloc(&settings, &context);

    int32_t shader_program = division_engine_shader_program_alloc();
    division_engine_shader_from_file_attach_to_program("test.vert", DIVISION_SHADER_VERTEX, shader_program);
    division_engine_shader_from_file_attach_to_program("test.frag", DIVISION_SHADER_FRAGMENT, shader_program);
    division_engine_shader_link_program(shader_program);

    int32_t posLocation = division_engine_shader_program_get_attribute_location("pos", shader_program);
    int32_t fColorLocation = division_engine_shader_program_get_attribute_location("fColor", shader_program);

    DivisionVertexAttributeSettings attr[2] = {
        {.type = DIVISION_FVEC3, .location = posLocation},
        {.type = DIVISION_FVEC4, .location = fColorLocation}
    };
    int32_t vertex_buffer = division_engine_vertex_buffer_alloc(context, attr, 2, 3);

    float positions[9] = {
        -1, -1, 0,
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
        context, vertex_buffer, objectIndex, posLocation, positions, 0, 3);
    division_engine_vertex_buffer_set_vertex_data_for_attribute(
        context, vertex_buffer, objectIndex, fColorLocation, colors, 0, 3);

    division_engine_render_pass_alloc(context, (DivisionRenderPass) {
        .vertex_buffer = vertex_buffer,
        .shader_program = shader_program,
        .topology = DIVISION_TOPOLOGY_TRIANGLES,
    });

    division_engine_renderer_run_loop(context, update_callback);
    division_engine_context_free(context);
}

void error_callback(int error_code, const char* message)
{
    fprintf(stderr, "Error code: %d, error message: %s", error_code, message);
}

void update_callback(DivisionContext* ctx)
{
}
