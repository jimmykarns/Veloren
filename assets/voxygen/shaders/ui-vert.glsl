#version 450 core

#include <globals.glsl>

layout(location=0) in vec2 v_pos;
layout(location=1) in vec2 v_uv;
layout(location=2) in vec4 v_color;
layout(location=3) in vec2 v_center;
layout(location=4) in uint v_mode;

layout (set = 1, binding = 0,std140)
uniform u_locals {
	vec4 w_pos;
};

layout(location=0) out vec2 f_uv;
layout(location=1) out vec4 f_color;
layout(location=2) flat out uint f_mode;

void main() {
    f_color = v_color;

    if (w_pos.w == 1.0) {
        f_uv = v_uv;
        // Fixed scale In-game element      
        vec4 projected_pos = proj_mat * view_mat * vec4(w_pos.xyz, 1.0); 
        gl_Position = vec4(projected_pos.xy / projected_pos.w + v_pos, 0.0, 1.0); 
    } else if (v_mode == uint(3)) {
        // HACK: North facing source rectangle.
        vec2 look_at_dir = normalize(vec2(-view_mat[0][2], -view_mat[1][2]));
        mat2 look_at = mat2(look_at_dir.y, look_at_dir.x, -look_at_dir.x, look_at_dir.y);
        f_uv = v_center + look_at * (v_uv - v_center);
        gl_Position = vec4(v_pos, 0.0, 1.0);
    } else if (v_mode == uint(5)) {
        // HACK: North facing target rectangle.
        f_uv = v_uv;
        float aspect_ratio = screen_res.x / screen_res.y;
        vec2 look_at_dir = normalize(vec2(-view_mat[0][2], -view_mat[1][2]));
        mat2 look_at = mat2(look_at_dir.y, -look_at_dir.x, look_at_dir.x, look_at_dir.y);
        vec2 v_len = v_pos - v_center;
        vec2 v_proj = look_at * vec2(v_len.x, v_len.y / aspect_ratio);
        gl_Position = vec4(v_center + vec2(v_proj.x, v_proj.y * aspect_ratio), 0.0, 1.0);
    } else {
        // Interface element
        f_uv = v_uv;
        gl_Position = vec4(v_pos, 0.0, 1.0);
    }
    f_mode = v_mode;
}
