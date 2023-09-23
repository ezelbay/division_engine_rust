#version 450 core

layout (location = 0) in vec2 inUV;

layout (location = 1) in float inBorderRadiusUV;
layout (location = 2) in vec2 inSize;
layout (location = 3) in vec2 inPosition;
layout (location = 4) in vec4 inColor;

layout (location = 0) out vec4 outColor;
layout (location = 1) out vec2 outUV;
layout (location = 2) out vec2 outWorldPosition;
layout (location = 3) out vec2 outSize;
layout (location = 4) out float outBorderRadius;

layout (std140, binding = 1) uniform Uniforms {
    vec2 screenSize;
};

void main() {
    vec2 worldPosition = inUV * inSize + inPosition; 
    vec2 normPos = worldPosition / screenSize;

    outColor = inColor;
    outUV = inUV;
    outWorldPosition = inPosition;
    outSize = inSize;
    outBorderRadius = inBorderRadiusUV;
    gl_Position = vec4(normPos * 2 - vec2(1), 0, 1);
}