#include <division_engine/vertex_attribute.h>

#include <glad/gl.h>

int32_t division_engine_attribute_get_location(const char* name, int32_t shader_program)
{
    return glGetAttribLocation(shader_program, name);
}
