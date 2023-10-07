#version 450 core

layout (location = 0) in vec4 Color;
layout (location = 1) in vec2 TexelCoord;

layout (location = 0) out vec4 FragColor;

layout (binding = 0) uniform sampler2D Tex;

void main() {
    ivec2 iTexCoord = ivec2(TexelCoord);
    float col = 0;

    int texelAdds[] = { -1 , 1, 0 };
    for (int i = 0; i < 3; i++)
    {
        int add = texelAdds[i];
        col += texelFetch(Tex, ivec2(iTexCoord.x + add, iTexCoord.y + add), 0).r;
    }

    col /= 3.0f;

    FragColor = col * Color;
}