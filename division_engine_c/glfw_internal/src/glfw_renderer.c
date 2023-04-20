#define GLFW_INCLUDE_NONE
#define GLAD_GL_IMPLEMENTATION
#include "glad/gl.h"
#include "GLFW/glfw3.h"

#include "division_engine/renderer.h"
#include "division_engine/vertex_buffer.h"

#include "division_engine/platform_internal/platform_renderer.h"
#include "division_engine/platform_internal/platform_vertex_buffer.h"

bool division_engine_internal_platform_renderer_alloc(DivisionContext* ctx, const DivisionSettings* settings)
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
        return false;
    }

    ctx->renderer_context->window_data = window;

    settings->init_callback(ctx);

    return true;
}

void division_engine_internal_platform_renderer_run_loop(DivisionContext* ctx, const DivisionSettings* settings)
{
    settings->init_callback(ctx);

    DivisionRendererSystemContext* renderer_context = ctx->renderer_context;
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
            settings->update_callback(ctx);
            division_engine_internal_platform_vertex_buffer_draw(ctx);
            glfwSwapBuffers(window);
        }

        glfwPollEvents();
    }
}

void division_engine_internal_platform_renderer_free(DivisionContext* ctx)
{
    glfwDestroyWindow((GLFWwindow*) ctx->renderer_context->window_data);
    glfwTerminate();
}