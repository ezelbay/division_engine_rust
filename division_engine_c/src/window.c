#define GLFW_INCLUDE_NONE
#include <glad/gl.h>
#include <GLFW/glfw3.h>
#include <stdio.h>

#include "window.h"

bool division_engine_init(int32_t width, int32_t height, const char* title) {
    if (!glfwInit()) {
        printf("Failed to init GLFW");
        return false;
    }

    GLFWwindow* window = glfwCreateWindow(width, height, title, NULL, NULL);
    if (!window) {
        const char* str;
        glfwGetError(&str);
        printf("%s", str);
        return false;
    }
    glfwMakeContextCurrent(window);

    int version = gladLoadGL(glfwGetProcAddress);
    if (version == 0) {
        printf("Failed to initialize opengl context");
        return false;
    }

    while(!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }

    glfwTerminate();
    return true;
}