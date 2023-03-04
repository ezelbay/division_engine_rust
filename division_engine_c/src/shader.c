#include "shader.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <glad/gl.h>

#include <list_utility.h>

static int create_shader_from_source(const char* path, GLuint gl_shader_type);
static bool check_program_status(GLuint programHandle);
static void get_program_info_log(GLuint program_handle, char** error_ptr);
static int shader_type_to_gl_type(DivisionEngineShaderType shaderType);

int32_t division_engine_shader_create_program() {
    return glCreateProgram();
}

bool division_engine_shader_attach_to_program(const char* path, DivisionEngineShaderType type, int32_t program_id) {
    int gl_shader_type = shader_type_to_gl_type(type);
    if (gl_shader_type < 0) {
        return false;
    }

    int shader_handle = create_shader_from_source(path, gl_shader_type);
    if (shader_handle < 0) {
        return false;
    }

    glAttachShader((GLuint) program_id, shader_handle);
    glDeleteShader(shader_handle);
    return true;
}

int create_shader_from_source(const char* path, GLuint gl_shader_type) {
    GLuint shader_handle = glCreateShader(gl_shader_type);
    if (!shader_handle) {
        fprintf(stderr, "Failed to create a shader");
        return -1;
    }

    FILE* shader_file_ptr = fopen(path, "rt");
    if (shader_file_ptr == NULL){
        fprintf(stderr, "Failed to open file by path %s", path);
        return -1;
    }

    fseek(shader_file_ptr, 0, SEEK_END);
    int shader_file_size = (int) ftell(shader_file_ptr);
    rewind(shader_file_ptr);

    char* shader_data = (char*) malloc(shader_file_size);
    fread(shader_data, 1, shader_file_size, shader_file_ptr);
    if (ferror(shader_file_ptr)) {
        fprintf(stderr, "Failed to read file by path: %s", path);
        fclose(shader_file_ptr);
        free(shader_data);
        return -1;
    }

    fclose(shader_file_ptr);
    glShaderSource(shader_handle, 1, &shader_data, &shader_file_size);
    glCompileShader(shader_handle);
    free(shader_data);

    GLint compile_result = 0;
    glGetShaderiv(shader_handle, GL_COMPILE_STATUS, &compile_result);
    if (compile_result == GL_TRUE) {
        return (int) shader_handle;
    }

    GLint error_length = 0;
    glGetShaderiv(shader_handle, GL_INFO_LOG_LENGTH, &error_length);
    char* error_log_data = malloc(error_length);
    glGetShaderInfoLog(shader_handle, error_length, &error_length, error_log_data);

    fprintf(stderr, "Failed to compile shader source. Info log: \n%s\n", error_log_data);
    free(error_log_data);
    return -1;
}

bool division_engine_shader_link_program(int32_t program_id) {
    GLuint gl_program = (GLuint) program_id;
    glLinkProgram(gl_program);
    return check_program_status(gl_program);
}

bool check_program_status(GLuint programHandle) {
    GLint linkStatus;
    glGetProgramiv(programHandle, GL_LINK_STATUS, &linkStatus);
    if (linkStatus == GL_FALSE)
    {
        char* error;
        get_program_info_log(programHandle, &error);
        fprintf(stderr, "Failed to link a shader program. Info log: \n%s", error);
        free(error);
        return false;
    }

    glValidateProgram(programHandle);
    GLint validateStatus;
    glGetProgramiv(programHandle, GL_VALIDATE_STATUS, &validateStatus);
    if (validateStatus == GL_FALSE)
    {
        char* error;
        get_program_info_log(programHandle, &error);
        fprintf(stderr, "Failed to validate a shader program. Info log: \n%s", error);
        free(error);
        return false;
    }

    return true;
}

void get_program_info_log(GLuint program_handle, char** error_ptr) {
    GLint error_length;
    glGetProgramiv(program_handle, GL_INFO_LOG_LENGTH, &error_length);
    char* error = malloc(error_length);
    glGetProgramInfoLog(program_handle, error_length, &error_length, error);

    *error_ptr = error;
}

int shader_type_to_gl_type(DivisionEngineShaderType shaderType) {
    switch (shaderType) {
        case DivisionEngineShaderVertex:
            return GL_VERTEX_SHADER;
        case DivisionEngineShaderFragment:
            return GL_FRAGMENT_SHADER;
        default:
            fprintf(stderr, "Unknown type of shader");
            return -1;
    }
}

void division_engine_shader_use_program(int32_t program_id) {
    glUseProgram((GLuint) program_id);
}

void division_engine_shader_destroy_program(int32_t program_id) {
    glDeleteProgram((GLuint) program_id);
}
