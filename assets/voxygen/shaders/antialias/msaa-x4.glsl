layout (set = 1, binding = 0) uniform texture2D t_src_color;
layout (set = 1, binding = 1) uniform sampler s_src_color;

sampler2DMS get_sampler() {
	return sampler2DMS(t_src_color,s_src_color);
}

vec4 aa_apply(sampler2DMS tex, vec2 fragCoord, vec2 resolution) {
	ivec2 texel_coord = ivec2(fragCoord.x, fragCoord.y);

        vec4 sample1 = texelFetch(tex, texel_coord, 0);
        vec4 sample2 = texelFetch(tex, texel_coord, 1);
        vec4 sample3 = texelFetch(tex, texel_coord, 2);
        vec4 sample4 = texelFetch(tex, texel_coord, 3);

	// Average Samples
	vec4 msaa_color = (sample1 + sample2 + sample3 + sample4) / 4.0;

	return msaa_color;
}