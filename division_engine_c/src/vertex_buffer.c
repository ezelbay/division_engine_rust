#include <vertex_buffer.h>

#include <stdio.h>
#include <stdlib.h>
#include <glad/gl.h>
#include <list_utility.h>

typedef struct {
    GLuint vao_handler;
    GLuint vbo_handler;
} VertexBufferInternal_;

DIVISION_LIST_DEFINE(VertexBufferInternal_);

static List_VertexBufferInternal_ gl_handlers_;

long division_engine_vertex_buffer_create(size_t size) {
    VertexBufferInternal_ vert_internal;

    glCreateVertexArrays(1, &vert_internal.vao_handler);
    glBindVertexArray(vert_internal.vao_handler);

    glGenBuffers(1, &vert_internal.vbo_handler);
    glBindBuffer(GL_ARRAY_BUFFER, vert_internal.vbo_handler);
    glBufferData(GL_ARRAY_BUFFER, (GLsizeiptr) size, NULL, GL_STATIC_DRAW);

    gl_handlers_ = DIVISION_LIST_CREATE(VertexBufferInternal_, 10);
    DIVISION_LIST_APPEND(gl_handlers_, vert_internal);

    return (long) gl_handlers_.length - 1;
}

void* division_engine_vertex_buffer_access_ptr_begin(long buffer_id) {
    return glMapNamedBuffer(gl_handlers_.items[buffer_id].vbo_handler, GL_READ_WRITE);
}

void division_engine_vertex_buffer_access_ptr_end(long buffer_id) {
    glUnmapNamedBuffer(gl_handlers_.items[buffer_id].vbo_handler);
}

static GLenum division_attribute_type_to_gl_type(DivisionAttributeType attributeType) {
    switch (attributeType) {
        case DIVISION_FLOAT: return GL_FLOAT;
        case DIVISION_DOUBLE: return GL_DOUBLE;
        case DIVISION_INTEGER: return GL_INT;
        default: {
            fprintf(stderr, "Unknown type of attribute: %d", attributeType);
            return -1;
        }
    };
}

void division_engine_vertex_buffer_define_attribute(long buffer_id, DivisionEngineVertexAttribute attribute) {
    glBindBuffer(GL_ARRAY_BUFFER, gl_handlers_.items[buffer_id].vbo_handler);
    glVertexAttribPointer(
        attribute.index,
        attribute.size_of_components,
        attribute.attribute_type,
        attribute.normalized,
        attribute.stride,
        &attribute.offset
    );
    glEnableVertexAttribArray(attribute.index);
}

void division_engine_vertex_buffer_draw_triangles(long buffer_id, size_t first_index, size_t count) {
    VertexBufferInternal_ buffer_data = gl_handlers_.items[buffer_id];

    glBindVertexArray(buffer_data.vao_handler);
    glBindBuffer(GL_ARRAY_BUFFER, buffer_data.vbo_handler);
    glDrawArrays(GL_TRIANGLES, (GLint) first_index, (GLsizei) count);
}
