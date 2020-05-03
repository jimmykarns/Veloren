layout (set = 1, binding = 0) uniform texture2D t_src_color;
layout (set = 1, binding = 1) uniform sampler s_src_color;

vec4 aa_apply(vec2 fragCoord, vec2 resolution) {
	return texture(sampler2D(t_src_color,s_src_color), fragCoord / resolution);
}