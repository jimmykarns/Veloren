#version 450 core

#include <globals.glsl>

layout(location=0) in vec2 v_pos;

layout (set = 1, binding = 0,std140)
uniform u_locals {
	vec4 nul;
};

layout(location=0) out vec2 f_pos;

void main() {
	f_pos = v_pos;

	gl_Position = vec4(v_pos, 0.0, 1.0);
}
