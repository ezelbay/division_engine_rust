#version 450 core

layout (location = 0) in vec2 vertPos;
layout (location = 1) in vec2 inUV;

layout (location = 2) in vec2 inSize;
layout (location = 3) in vec2 inPosition;
layout (location = 4) in vec4 inColor;
layout (location = 5) in vec4 in_TRBRTLBL_BorderRadius;

layout (location = 0) out vec4 outColor;
layout (location = 1) out vec4 out_TRBRTLBL_BorderRadius;
layout (location = 2) out vec2 outUV;
layout (location = 3) out vec2 outPosition;
layout (location = 4) out vec2 outSize;
layout (location = 5) out vec2 outVertPos;

layout (std140, binding = 1) uniform Uniforms {
    vec2 screenSize;
};

void main() {
    vec2 vertWorldPos = vertPos * inSize + inPosition; 
    vec2 normPos = vertWorldPos / screenSize;

    outColor = inColor;
    out_TRBRTLBL_BorderRadius = in_TRBRTLBL_BorderRadius;
    outUV = inUV;
    outPosition = inPosition;
    outSize = inSize;
    outVertPos = vertWorldPos;

    gl_Position = vec4(mix(vec2(-1,-1), vec2(1,1), normPos), 0, 1);
}