#define GLFW_INCLUDE_NONE
#define GLAD_GL_IMPLEMENTATION
#include <time.h>
#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "window.h"

bool division_engine_create_window(
    const DivisionEngineSettings* settings,
    DivisionEngineHandler* output_handler
) {
    if (!glfwInit()) {
        settings->error_callback(0, "Failed to init GLFW");
        return false;
    }

    glfwSetErrorCallback(settings->error_callback);

    GLFWwindow* window = glfwCreateWindow(
        (int) settings->window_width,
        (int) settings->window_height,
        settings->window_title, NULL, NULL
    );
    if (!window) {
        return false;
    }
    glfwMakeContextCurrent(window);

    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0) {
        settings->error_callback(0, "Failed to load GLAD");
        return false;
    }

    output_handler->_internal_data = window;
    return true;
}

void division_engine_run_event_loop(DivisionEngineHandler handler, DivisionEngineUpdateFunc update_callback) {
    GLFWwindow* window = (GLFWwindow*) handler._internal_data;
    DivisionEngineState state;
    double last_frame_time, current_time;

    last_frame_time = glfwGetTime();
    while(!glfwWindowShouldClose(window)) {
        glClear(GL_COLOR_BUFFER_BIT);
        glClearColor(0,1,0,1);

        current_time = glfwGetTime();
        state.delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        update_callback(state);

        glfwPollEvents();
        glfwSwapBuffers(window);
    }
}

void division_engine_destroy_window(DivisionEngineHandler handler) {
    glfwDestroyWindow((GLFWwindow*) handler._internal_data);
    glfwTerminate();
}