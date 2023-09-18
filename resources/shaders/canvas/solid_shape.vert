#version 450 core

layout (location = 1) in vec2 pos;
layout (location = 2) in mat4 localToWorld;

void main() {
    gl_Position = localToWorld * vec4(pos.xy, 0, 1);
}
