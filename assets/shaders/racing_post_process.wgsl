// Racing Game Post-Processing Effects
// Includes: Chromatic Aberration, Vignette, Speed Lines, Color Grading

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RacingPostProcessSettings {
    speed_intensity: f32,
    chromatic_aberration: f32,
    vignette_strength: f32,
    speed_lines: f32,
    color_saturation: f32,
    contrast: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec2<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: RacingPostProcessSettings;

// Utility functions
fn luma(color: vec3<f32>) -> f32 {
    return dot(color, vec3<f32>(0.299, 0.587, 0.114));
}

fn contrast_adjust(color: vec3<f32>, contrast: f32) -> vec3<f32> {
    return (color - 0.5) * contrast + 0.5;
}

fn saturation_adjust(color: vec3<f32>, saturation: f32) -> vec3<f32> {
    let gray = vec3<f32>(luma(color));
    return mix(gray, color, saturation);
}

// Chromatic aberration effect
fn chromatic_aberration(uv: vec2<f32>, strength: f32) -> vec3<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let offset = (uv - center) * strength;
    
    let r = textureSample(screen_texture, texture_sampler, uv + offset).r;
    let g = textureSample(screen_texture, texture_sampler, uv).g;
    let b = textureSample(screen_texture, texture_sampler, uv - offset).b;
    
    return vec3<f32>(r, g, b);
}

// Vignette effect
fn vignette(uv: vec2<f32>, strength: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);
    let vignette = smoothstep(0.8, 0.2, dist * strength);
    return vignette;
}

// Speed lines / radial blur effect
fn speed_lines(uv: vec2<f32>, strength: f32) -> vec3<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let direction = normalize(uv - center);
    let dist = distance(uv, center);
    
    // Only apply speed lines to edges, not center
    let edge_factor = smoothstep(0.1, 0.6, dist);
    let blur_amount = strength * edge_factor;
    
    var color = vec3<f32>(0.0);
    let samples = 8;
    
    for (var i = 0; i < samples; i++) {
        let offset = direction * blur_amount * (f32(i) / f32(samples - 1) - 0.5);
        color += textureSample(screen_texture, texture_sampler, uv + offset).rgb;
    }
    
    return color / f32(samples);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Base color with chromatic aberration
    var color: vec3<f32>;
    if (settings.chromatic_aberration > 0.0) {
        color = chromatic_aberration(uv, settings.chromatic_aberration);
    } else {
        color = textureSample(screen_texture, texture_sampler, uv).rgb;
    }
    
    // Apply speed lines effect
    if (settings.speed_lines > 0.0) {
        let speed_color = speed_lines(uv, settings.speed_lines * 0.02);
        color = mix(color, speed_color, settings.speed_lines * 0.5);
    }
    
    // Color grading
    color = saturation_adjust(color, settings.color_saturation);
    color = contrast_adjust(color, settings.contrast);
    
    // Vignette effect
    let vignette_factor = vignette(uv, settings.vignette_strength);
    color *= vignette_factor;
    
    // Slight speed-based color tint (warmer colors at high speed)
    let speed_tint = vec3<f32>(1.0 + settings.speed_intensity * 0.1, 1.0, 1.0 - settings.speed_intensity * 0.05);
    color *= speed_tint;
    
    return vec4<f32>(color, 1.0);
} 