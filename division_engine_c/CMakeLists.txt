CMAKE_MINIMUM_REQUIRED(VERSION 3.22.2)
project(division_engine_c)
set(CMAKE_C_STANDARD 11)

include(FetchContent)

FetchContent_Declare(
        glfw
        GIT_REPOSITORY https://github.com/glfw/glfw
        GIT_TAG 3.3.8
)
FetchContent_MakeAvailable(glfw)

set(SOURCES
        lib/glad/include/glad/gl.h
        src/context.c
        src/renderer.c
        src/render_pass.c
        src/shader.c
        src/vertex_attribute.c
        src/vertex_buffer.c
)
set(FILES_TO_COPY ${GLFW_BINARY_DIR}/src/libglfw3.a)

add_library(division_engine_c STATIC ${SOURCES})

target_include_directories(division_engine_c PUBLIC
        ${CMAKE_SOURCE_DIR}/include ${CMAKE_SOURCE_DIR}/lib/glad/include
        )
target_link_libraries(division_engine_c PUBLIC glfw ${GLFW_STATIC_LIBRARIES})

add_executable(division_main_test tests/main.c)
target_link_libraries(division_main_test PUBLIC division_engine_c)
target_include_directories(division_main_test PUBLIC include)


set(TEST_FILES_TO_COPY ${CMAKE_SOURCE_DIR}/tests/shaders/test.frag ${CMAKE_SOURCE_DIR}/tests/shaders/test.vert)
foreach(TEST_COPY_FILE ${TEST_FILES_TO_COPY})
    configure_file(${TEST_COPY_FILE} ${PROJECT_BINARY_DIR} COPYONLY)
endforeach()


foreach (COPY_FILE ${FILES_TO_COPY})
    add_custom_command(TARGET division_engine_c POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy ${COPY_FILE} ${PROJECT_BINARY_DIR}
            )
endforeach ()
