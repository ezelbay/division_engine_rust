#define GLFW_INCLUDE_NONE
#define GLAD_GL_IMPLEMENTATION
#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "renderer.h"

bool division_engine_internal_renderer_create(
    DivisionRendererContext *renderer_context,
    const DivisionEngineSettings *settings
)
{
    glfwSetErrorCallback(settings->error_callback);

    if (!glfwInit())
    {
        settings->error_callback(0, "Failed to init GLFW");
        return false;
    }

    GLFWwindow *window = glfwCreateWindow(
        (int) settings->window_width,
        (int) settings->window_height,
        settings->window_title, NULL, NULL
    );
    renderer_context->window_data = window;

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

    return true;
}

void division_engine_renderer_run_loop(
    DivisionRendererContext *renderer_context, DivisionEngineUpdateFunc update_callback)
{
    GLFWwindow *window = (GLFWwindow *) renderer_context->window_data;
    DivisionEngineState state;
    double last_frame_time, current_time;

    last_frame_time = glfwGetTime();
    while (!glfwWindowShouldClose(window))
    {
        glClearColor(0, 1, 0, 1);
        glClear(GL_COLOR_BUFFER_BIT);

        current_time = glfwGetTime();
        state.delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        update_callback(state);

        glfwPollEvents();
        glfwSwapBuffers(window);
    }
}

void division_engine_renderer_destroy(DivisionRendererContext *renderer_context)
{
    glfwDestroyWindow((GLFWwindow *) renderer_context->window_data);
    glfwTerminate();

    renderer_context->window_data = NULL;
}