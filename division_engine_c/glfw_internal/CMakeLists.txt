include(FetchContent)

FetchContent_Declare(
        glfw
        GIT_REPOSITORY https://github.com/glfw/glfw
        GIT_TAG 3.3.8
)
FetchContent_MakeAvailable(glfw)


set(GLFW_SOURCES
    lib/glad/include/glad/gl.h
    src/glfw_renderer.c
    src/glfw_vertex_buffer.c
    src/glfw_shader.c
)

add_library(glfw_internal STATIC ${GLFW_SOURCES})

target_include_directories(glfw_internal PRIVATE ${CMAKE_SOURCE_DIR}/include lib/glad/include)
target_link_libraries(glfw_internal PRIVATE glfw ${GLFW_STATIC_LIBRARIES})
set(FILES_TO_COPY ${GLFW_BINARY_DIR}/src/libglfw3.a)