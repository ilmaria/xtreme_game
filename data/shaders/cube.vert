#version 410
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 color;

layout (location = 0) out vec4 frag_color;

void main() {
    frag_color = color;
    gl_Position = pos;
}