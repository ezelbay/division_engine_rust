#version 450 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec4 fColor;

uniform vec3 TestVec;

out vec4 VertexColor;

void main() {
    VertexColor = fColor;
    gl_Position = vec4(pos, 1);
}