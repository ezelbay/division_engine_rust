#version 450 core

layout (location = 0) in vec2 vertPos;
layout (location = 1) in vec2 inUV;

layout (location = 2) in vec4 inColor;
layout (location = 3) in vec2 inTexelCoord;
layout (location = 4) in vec2 inSize;
layout (location = 5) in vec2 inPosition;
layout (location = 6) in vec2 glyphInTexSize;

layout (location = 0) out vec4 outColor;
layout (location = 1) out vec2 outTexelCoord;
layout (location = 2) out centroid vec2 outUV;

layout (std140, binding = 1) uniform Uniforms {
    vec2 screenSize;
};

void main() {
    vec2 vertWorldPos = vertPos * inSize + inPosition; 
    vec2 normPos = vertWorldPos / screenSize;
    
    outColor = inColor;
    outTexelCoord = inTexelCoord + glyphInTexSize * inUV;
    outUV = inUV;

    gl_Position = vec4(mix(vec2(-1,-1), vec2(1,1), normPos), 0, 1);
}