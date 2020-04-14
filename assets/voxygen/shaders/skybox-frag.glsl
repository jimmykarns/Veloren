#version 450 core

#include <globals.glsl>
#include <sky.glsl>

layout(location=0) in vec3 f_pos;

layout (set = 1, binding = 0,std140)
uniform u_locals {
	vec4 nul;
};

layout(location=0) out vec4 tgt_color;

void main() {
	vec4 _clouds;
	tgt_color = vec4(get_sky_color(normalize(f_pos), time_of_day.x, cam_pos.xyz, vec3(-100000), 1.0, true, _clouds), 1.0);
}
