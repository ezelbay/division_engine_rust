#version 450 core

layout (location = 0) in vec2 UV;
layout (location = 1) in vec4 Color;

layout (location = 0) out vec4 ResultColor;

layout (binding = 0) uniform sampler2D _tex;

void main() {
    vec4 texColor = texture(_tex, UV);

    ResultColor = texColor * Color;
}