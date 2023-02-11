#define GLFW_INCLUDE_NONE
#define GLAD_GL_IMPLEMENTATION
#include <time.h>
#include <glad/gl.h>
#include <GLFW/glfw3.h>
#include <list_utility.h>

#include "window.h"

typedef struct {
    GLFWwindow* window_ptr;
} DivisionWindow;

DIVISION_LIST_DEFINE(DivisionWindow);

static List_DivisionWindow windows_;
static DivisionEngineErrorFunc error_callback_;

inline static bool check_window_id(int32_t window_id);

bool division_engine_start_session(DivisionEngineErrorFunc error_callback) {
    error_callback_ = error_callback;
    glfwSetErrorCallback(error_callback_);

    if (windows_.items != NULL) {
        error_callback_(0, "Previous division engine session was not finished");
        return false;
    }

    if (!glfwInit()) {
        error_callback_(0, "Failed to init GLFW");
        return false;
    }

    windows_ = DIVISION_LIST_CREATE(DivisionWindow, 1);
    return true;
}

int32_t division_engine_window_create(const DivisionEngineSettings* settings) {
    if (windows_.items == NULL) {
        return -1;
    }

    GLFWwindow* window = glfwCreateWindow(
        (int) settings->window_width,
        (int) settings->window_height,
        settings->window_title, NULL, NULL
    );
    if (!window) {
        if (error_callback_ != NULL) {
            error_callback_(0, "Can't create a new GLFW window");
        }
        return -1;
    }
    glfwMakeContextCurrent(window);

    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0) {
        if (error_callback_ != NULL) {
            error_callback_(0, "Failed to load GLAD");
        }
        return -1;
    }

    DivisionWindow instance = { .window_ptr = window };
    DIVISION_LIST_APPEND(windows_, instance);

    return (int32_t) windows_.length - 1;
}

void division_engine_window_run_event_loop(int32_t window_id, DivisionEngineUpdateFunc event_update_callback) {
    check_window_id(window_id);

    GLFWwindow* window = windows_.items[window_id].window_ptr;
    DivisionEngineState state;
    double last_frame_time, current_time;

    last_frame_time = glfwGetTime();
    while(!glfwWindowShouldClose(window)) {
        glClearColor(0,1,0,1);
        glClear(GL_COLOR_BUFFER_BIT);

        current_time = glfwGetTime();
        state.delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        event_update_callback(state);

        glfwPollEvents();
        glfwSwapBuffers(window);
    }
}

void division_engine_window_destroy(int32_t window_id) {
    check_window_id(window_id);
    glfwDestroyWindow(windows_.items[window_id].window_ptr);
    DIVISION_LIST_REMOVE_AT(windows_, window_id);
}

void division_engine_finish_session() {
    error_callback_ = NULL;
    DIVISION_LIST_DESTROY(windows_);
    glfwTerminate();
}

bool check_window_id(int32_t window_id) {
    if (window_id < 0 || window_id >= windows_.length) {
        if (error_callback_ != NULL) {
            error_callback_(0, "Incorrect instance id");
        }

        return false;
    }

    return true;
}