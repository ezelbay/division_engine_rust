#version 450 core

layout (location = 0) in vec4 Color;
layout (location = 1) in vec2 UV;
layout (location = 2) in vec2 Position;
layout (location = 3) in vec2 Size;
layout (location = 4) in float BorderRadius;

layout (location = 0) out vec4 ResultColor;

layout (binding = 0) uniform sampler2D Tex;

float sdRoundedBox( in vec2 p, in vec2 b, in vec4 r )
{
    r.xy = (p.x>0.0)?r.xy : r.zw;
    r.x  = (p.y>0.0)?r.x  : r.y;
    vec2 q = abs(p)-b+r.x;
    return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
}

float circle(vec2 position, float radius) {
    return length(position) - radius;
}

float rectange(vec2 position, vec2 halfSize) {
    return (abs(position) - halfSize).x;
}

float testSDF(vec2 position) {
    return rectange(position, vec2(50.0, 50.0));
}

void main() {
    vec4 texColor = texture(Tex, UV);
    vec2 extents = Size * 0.5;

    ResultColor = texColor * Color;
    ResultColor.a = -sdRoundedBox(
        gl_FragCoord.xy - Position - extents, 
        extents, 
        vec4(BorderRadius)
    );
}