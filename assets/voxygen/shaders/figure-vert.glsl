#version 450 core

#include <globals.glsl>

layout(location=0) in vec3 v_pos;
layout(location=1) in vec3 v_norm;
layout(location=2) in vec3 v_col;
layout(location=3) in uint v_bone_idx;

layout (set = 3, binding = 0,std140)
uniform u_locals {
	mat4 model_mat;
	vec4 model_col;
};

struct BoneData {
	mat4 bone_mat;
};

layout (set = 4, binding = 0,std140)
uniform u_bones {
	BoneData bones[16];
};

layout(location=0) out vec3 f_pos;
layout(location=1) out vec3 f_col;
layout(location=2) flat out vec3 f_norm;

void main() {
	// Pre-calculate bone matrix
	mat4 combined_mat = model_mat * bones[v_bone_idx].bone_mat;

	f_pos = (
		combined_mat *
		vec4(v_pos, 1)).xyz;

	f_col = v_col;

	// Calculate normal here rather than for each pixel in the fragment shader
	f_norm = normalize((
		combined_mat *
		vec4(v_norm, 0.0)
	).xyz);

	gl_Position = all_mat * vec4(f_pos, 1);
	gl_Position.z = -1000.0 / (gl_Position.z + 10000.0);
}
