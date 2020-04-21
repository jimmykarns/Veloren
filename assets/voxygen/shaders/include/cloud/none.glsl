layout(set = 0, binding = 1)
uniform texture2D t_noise;
layout(set = 0, binding = 2)
uniform sampler s_noise;

vec4 get_cloud_color(vec3 dir, vec3 origin, float time_of_day, float max_dist, float quality) {
    return vec4(0.0);
}
