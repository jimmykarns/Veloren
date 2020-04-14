#version 450 core

#include <globals.glsl>
#include <srgb.glsl>

layout(location=0) in vec3 v_pos;
layout(location=1) in vec3 v_norm;
layout(location=2) in vec3 v_col;
layout(location=3) in vec4 inst_mat0;
layout(location=4) in vec4 inst_mat1;
layout(location=5) in vec4 inst_mat2;
layout(location=6) in vec4 inst_mat3;
layout(location=7) in vec3 inst_col;
layout(location=8) in float inst_wind_sway;

layout(location=0) out vec3 f_pos;
layout(location=1) flat out vec3 f_norm;
layout(location=2) out vec3 f_col;
layout(location=3) out float f_light;

const float SCALE = 1.0 / 11.0;

void main() {
	mat4 inst_mat;
	inst_mat[0] = inst_mat0;
	inst_mat[1] = inst_mat1;
	inst_mat[2] = inst_mat2;
	inst_mat[3] = inst_mat3;

	vec3 sprite_pos = (inst_mat * vec4(0, 0, 0, 1)).xyz;

	f_pos = (inst_mat * vec4(v_pos * SCALE, 1)).xyz;

	// Wind waving
	f_pos += inst_wind_sway * vec3(
		sin(tick.x * 1.5 + f_pos.y * 0.1) * sin(tick.x * 0.35),
		sin(tick.x * 1.5 + f_pos.x * 0.1) * sin(tick.x * 0.25),
		0.0
	) * pow(abs(v_pos.z) * SCALE, 1.3) * 0.2;

	f_norm = (inst_mat * vec4(v_norm, 0)).xyz;

	f_col = srgb_to_linear(v_col) * srgb_to_linear(inst_col);

	// Select glowing
	if (select_pos.w > 0 && select_pos.xyz == floor(sprite_pos)) {
		f_col *= 4.0;
	}

	f_light = 1.0;

	gl_Position =
		all_mat *
		vec4(f_pos, 1);
	gl_Position.z = -1000.0 / (gl_Position.z + 10000.0);
}
