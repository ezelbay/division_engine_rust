#version 450 core

in vec4 VertexColor;
out vec4 FragColor;

uniform vec4 TestColor;

void main() {
    FragColor = VertexColor * TestColor;
}