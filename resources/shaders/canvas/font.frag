#version 450 core

layout (location = 0) in vec4 Color;
layout (location = 1) in vec2 TexelCoord;
layout (location = 2) in centroid vec2 UV;

layout (location = 0) out vec4 FragColor;

layout (binding = 0) uniform sampler2D Tex;

void main() {
    ivec2 iTexCoord = ivec2(TexelCoord);
    float col = texelFetch(Tex, ivec2(iTexCoord.x, iTexCoord.y), 0).r;

    FragColor = col * Color;
}