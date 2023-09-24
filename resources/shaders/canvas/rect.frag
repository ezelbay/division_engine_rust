#version 450 core

layout (location = 0) in vec4 Color;
layout (location = 1) in vec4 TRBRTLBL_BorderRadius;
layout (location = 2) in vec2 UV;
layout (location = 3) in vec2 Position;
layout (location = 4) in vec2 Size;

layout (location = 0) out vec4 ResultColor;

layout (binding = 0) uniform sampler2D Tex;

layout(origin_upper_left) in vec4 gl_FragCoord;

layout (std140, binding = 1) uniform Uniforms {
    vec2 screenSize;
};

float select(bool selector, float a, float b) {
    return float(selector) * a + float(!selector) * b;
}

vec2 select(bool selector, vec2 a, vec2 b) {
    return float(selector) * a + float(!selector) * b;
}

float sdRoundedBox(in vec2 p, in vec2 b, in vec4 r)
{
    r.xy = select(p.x > 0.0, r.xy, r.zw);
    r.x  = select(p.y > 0.0, r.x, r.y);
    
    vec2 q = abs(p)-b+r.x;
    return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
}

vec2 fragCoordBottomLeft() {
    return vec2(gl_FragCoord.x, screenSize.y - gl_FragCoord.y);
}

void main() {
    vec4 texColor = texture(Tex, UV);
    vec2 extents = Size * 0.5;

    ResultColor = texColor * Color;
    ResultColor.a = -sdRoundedBox(
        fragCoordBottomLeft() - Position - extents,
        extents, 
        TRBRTLBL_BorderRadius
    );
}