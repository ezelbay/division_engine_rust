#version 450 core

layout (location = 0) in vec4 Color;
layout (location = 1) in vec2 TexelCoord;
layout (location = 2) in vec2 UV;

layout (location = 0) out vec4 FragColor;

layout (binding = 0) uniform sampler2D Tex;

void main() {
    ivec2 iTexCoord = ivec2(TexelCoord);

    float col = 0;
    for (int i = -1; i <= 1; i++)
    {
        for (int j = -1; j <= 1; j++)
        {
            col += texelFetch(Tex, ivec2(iTexCoord.x + i, iTexCoord.y + j), 0).r;
        }
    }
    col /= 9.0f;

    FragColor = col * Color;
}