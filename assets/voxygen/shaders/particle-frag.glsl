#version 330 core

#include <constants.glsl>

#define LIGHTING_TYPE LIGHTING_TYPE_REFLECTION

#define LIGHTING_REFLECTION_KIND LIGHTING_REFLECTION_KIND_GLOSSY

#define LIGHTING_TRANSPORT_MODE LIGHTING_TRANSPORT_MODE_IMPORTANCE

#define LIGHTING_DISTRIBUTION_SCHEME LIGHTING_DISTRIBUTION_SCHEME_MICROFACET

#define LIGHTING_DISTRIBUTION LIGHTING_DISTRIBUTION_BECKMANN

#define HAS_SHADOW_MAPS

#include <globals.glsl>

in vec3 f_pos;
flat in vec3 f_norm;
in vec3 f_col;

out vec4 tgt_color;

#include <sky.glsl>
#include <light.glsl>
#include <lod.glsl>

const float FADE_DIST = 32.0;

void main() {
    vec3 cam_to_frag = normalize(f_pos - cam_pos.xyz);
    vec3 view_dir = -cam_to_frag;

#if (SHADOW_MODE == SHADOW_MODE_CHEAP || SHADOW_MODE == SHADOW_MODE_MAP || FLUID_MODE == FLUID_MODE_SHINY)
    float f_alt = alt_at(f_pos.xy);
#elif (SHADOW_MODE == SHADOW_MODE_NONE || FLUID_MODE == FLUID_MODE_CHEAP)
    float f_alt = f_pos.z;
#endif

#if (SHADOW_MODE == SHADOW_MODE_CHEAP || SHADOW_MODE == SHADOW_MODE_MAP)
    vec4 f_shadow = textureBicubic(t_horizon, pos_to_tex(f_pos.xy));
    float sun_shade_frac = horizon_at2(f_shadow, f_alt, f_pos, sun_dir);
#elif (SHADOW_MODE == SHADOW_MODE_NONE)
    float sun_shade_frac = 1.0;
#endif
    float moon_shade_frac = 1.0;

    float point_shadow = shadow_at(f_pos, f_norm);
    DirectionalLight sun_info = get_sun_info(sun_dir, point_shadow * sun_shade_frac, f_pos);
    DirectionalLight moon_info = get_moon_info(moon_dir, point_shadow * moon_shade_frac);

	vec3 surf_color = f_col;
    float alpha = 1.0;
    const float n2 = 1.5;
    const float R_s2s0 = pow((1.0 - n2) / (1.0 + n2), 2);
    const float R_s1s0 = pow((1.3325 - n2) / (1.3325 + n2), 2);
    const float R_s2s1 = pow((1.0 - 1.3325) / (1.0 + 1.3325), 2);
    const float R_s1s2 = pow((1.3325 - 1.0) / (1.3325 + 1.0), 2);
    float R_s = (f_pos.z < f_alt) ? mix(R_s2s1 * R_s1s0, R_s1s0, medium.x) : mix(R_s2s0, R_s1s2 * R_s2s0, medium.x);

    vec3 k_a = vec3(1.0);
    vec3 k_d = vec3(1.0);
    vec3 k_s = vec3(R_s);

    vec3 emitted_light, reflected_light;

    // To account for prior saturation.
    float max_light = 0.0;
    max_light += get_sun_diffuse2(sun_info, moon_info, f_norm, view_dir, k_a, k_d, k_s, alpha, emitted_light, reflected_light);

    max_light += lights_at(f_pos, f_norm, view_dir, k_a, k_d, k_s, alpha, emitted_light, reflected_light);

	surf_color = illuminate(max_light, view_dir, surf_color * emitted_light, surf_color * reflected_light);

#if (CLOUD_MODE == CLOUD_MODE_REGULAR)
	float fog_level = fog(f_pos.xyz, focus_pos.xyz, medium.x);
	vec4 clouds;
	vec3 fog_color = get_sky_color(cam_to_frag, time_of_day.x, cam_pos.xyz, f_pos, 0.5, false, clouds);
	vec3 color = mix(mix(surf_color, fog_color, fog_level), clouds.rgb, clouds.a);
#elif (CLOUD_MODE == CLOUD_MODE_NONE)
    vec3 color = surf_color;
#endif

	tgt_color = vec4(color, 0.3);
}
