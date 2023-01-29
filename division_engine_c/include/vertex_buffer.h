#pragma once

#include <stdlib.h>
#include <stdbool.h>

typedef enum {
    DIVISION_FLOAT,
    DIVISION_DOUBLE,
    DIVISION_INTEGER,
} DivisionAttributeType;

typedef struct {
    DivisionAttributeType attribute_type;
    int index;
    int offset;
    int stride;
    int size_of_components;
    bool normalized;
} DivisionEngineVertexAttribute;

long division_engine_vertex_buffer_create(size_t size);
void division_engine_vertex_buffer_define_attribute(long buffer_id, DivisionEngineVertexAttribute attribute);

void* division_engine_vertex_buffer_access_ptr_begin(long buffer_id);
void division_engine_vertex_buffer_access_ptr_end(long buffer_id);

void division_engine_vertex_buffer_draw_triangles(long buffer_id, size_t first_index, size_t count);