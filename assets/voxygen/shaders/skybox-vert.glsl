#version 450 core

#include <globals.glsl>

layout(location=0) in vec3 v_pos;

layout(location=0) out vec3 f_pos;

void main() {
	f_pos = v_pos;

	// TODO: Make this position-independent to avoid rounding error jittering
	gl_Position =
		proj_mat *
		view_mat *
		vec4(v_pos * 100000.0 + cam_pos.xyz, 1);
	gl_Position.z = 0.0;
}
