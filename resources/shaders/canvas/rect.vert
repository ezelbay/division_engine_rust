#version 450 core

layout (location = 0) in vec2 inUV;
layout (location = 1) in vec4 inColor;

layout (location = 2) in mat4 transform;

layout (location = 0) out vec2 outUV;
layout (location = 1) out vec4 outColor;

layout (std140, binding = 1) uniform Uniforms {
    mat4 view;
};

void main() {
    outColor = inColor;
    outUV = inUV;

    gl_Position = view * transform * vec4(inUV, 0, 1);
}