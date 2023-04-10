#include <stdio.h>
#include "division_engine/renderer.h"
#include "division_engine/render_pass.h"
#include "division_engine/vertex_buffer.h"

#include "glad/gl.h"
#include "GLFW/glfw3.h"

bool division_engine_internal_renderer_context_alloc(
    DivisionContext* renderer_context,
    const DivisionEngineSettings* settings
)
{
    glfwSetErrorCallback(settings->error_callback);

    if (!glfwInit())
    {
        settings->error_callback(0, "Failed to init GLFW");
        return false;
    }

    GLFWwindow* window = glfwCreateWindow(
        (int) settings->window_width,
        (int) settings->window_height,
        settings->window_title, NULL, NULL
    );

    if (!window)
    {
        settings->error_callback(0, "Can't create a new GLFW window");
        return false;
    }

    glfwMakeContextCurrent(window);

    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0)
    {
        settings->error_callback(0, "Failed to load GLAD");
        return -1;
    }

    renderer_context->renderer_context = (DivisionRendererSystemContext) {
        .window_data = window,
        .clear_color = {0, 0, 0, 1}
    };

    return true;
}

void division_engine_renderer_run_loop(
    DivisionContext* ctx, DivisionEngineUpdateFunc update_callback)
{
    DivisionRendererSystemContext* renderer_context = &ctx->renderer_context;
    GLFWwindow* window = (GLFWwindow*) renderer_context->window_data;
    double last_frame_time, current_time, delta_time;

    last_frame_time = glfwGetTime();
    while (!glfwWindowShouldClose(window))
    {
        current_time = glfwGetTime();
        delta_time = current_time - last_frame_time;

        if (delta_time >= 1 / 60.f)
        {
            glClearBufferfv(GL_COLOR, 0, (const GLfloat*) &renderer_context->clear_color);

            delta_time = current_time - last_frame_time;
            last_frame_time = current_time;

            ctx->state.delta_time = delta_time;
            update_callback(ctx);
            division_engine_internal_vertex_buffer_draw(ctx);
            glfwSwapBuffers(window);
        }

        glfwPollEvents();
    }
}

void division_engine_internal_renderer_context_free(DivisionContext* ctx)
{
    glfwDestroyWindow((GLFWwindow*) ctx->renderer_context.window_data);
    glfwTerminate();

    ctx->renderer_context.window_data = NULL;
}