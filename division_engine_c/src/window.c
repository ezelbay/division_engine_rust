#define GLFW_INCLUDE_NONE
#include <glad/gl.h>
#include <GLFW/glfw3.h>
#include <division_engine_state.h>
#include <time.h>

#include "window.h"

bool division_engine_init(const DivisionEngineSettings* settings) {
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

    DivisionEngineState state;
    time_t last_frame_time, current_time;
    time(&last_frame_time);

    while(!glfwWindowShouldClose(window)) {
        glClear(GL_COLOR_BUFFER_BIT);
        glClearColor(0,1,0,1);

        time(&current_time);
        state.deltaTimeSec = (float) (current_time - last_frame_time);
        time(&last_frame_time);

        glfwPollEvents();
        glfwSwapBuffers(window);
    }

    return true;
}