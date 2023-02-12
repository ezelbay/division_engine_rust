#define GLFW_INCLUDE_NONE
#define GLAD_GL_IMPLEMENTATION
#include <time.h>
#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "renderer.h"

static GLFWwindow* window_;
static DivisionEngineErrorFunc error_callback_;

bool division_engine_renderer_create(
    const DivisionEngineSettings* settings,
    DivisionEngineErrorFunc error_callback
) {
    error_callback_ = error_callback;
    glfwSetErrorCallback(error_callback_);

    if (window_) {
        error_callback_(0, "There is can be only single instance of the division renderer");
        return false;
    }

    if (!glfwInit()) {
        error_callback_(0, "Failed to init GLFW");
        return false;
    }

    window_ = glfwCreateWindow(
            (int) settings->window_width,
            (int) settings->window_height,
            settings->window_title, NULL, NULL
    );
    if (!window_) {
        if (error_callback_) {
            error_callback_(0, "Can't create a new GLFW window");
        }
        return false;
    }

    glfwMakeContextCurrent(window_);

    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0) {
        if (error_callback_) {
            error_callback_(0, "Failed to load GLAD");
        }
        return -1;
    }

    return true;
}

void division_engine_renderer_run_loop(DivisionEngineUpdateFunc update_callback) {
    DivisionEngineState state;
    double last_frame_time, current_time;

    last_frame_time = glfwGetTime();
    while(!glfwWindowShouldClose(window_)) {
        glClearColor(0,1,0,1);
        glClear(GL_COLOR_BUFFER_BIT);

        current_time = glfwGetTime();
        state.delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        update_callback(state);

        glfwPollEvents();
        glfwSwapBuffers(window_);
    }
}

void division_engine_renderer_destroy() {
    glfwDestroyWindow(window_);
    glfwTerminate();

    window_ = NULL;
    error_callback_ = NULL;
}