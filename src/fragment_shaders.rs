use nalgebra_glm::{Vec2, Vec3, dot};
use crate::color::Color;
use crate::vertex::Vertex;

pub type FragmentShader = fn(&Vertex, &Vertex, &Vertex, Vec3, Vec3, Vec3, Vec2) -> Color;

// Utility functions for noise and patterns
fn hash(n: f32) -> f32 {
    let x = (n * 12.9898).sin() * 43758.5453;
    x - x.floor()
}

fn hash_vec3(p: Vec3) -> f32 {
    let n = p.x * 12.9898 + p.y * 78.233 + p.z * 45.164;
    hash(n)
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn noise(p: Vec3) -> f32 {
    let i = Vec3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let f = Vec3::new(p.x - i.x, p.y - i.y, p.z - i.z);
    
    // Smooth interpolation
    let u = Vec3::new(
        smoothstep(0.0, 1.0, f.x),
        smoothstep(0.0, 1.0, f.y),
        smoothstep(0.0, 1.0, f.z)
    );
    
    // Hash values at corners
    let a = hash_vec3(i);
    let b = hash_vec3(Vec3::new(i.x + 1.0, i.y, i.z));
    let c = hash_vec3(Vec3::new(i.x, i.y + 1.0, i.z));
    let d = hash_vec3(Vec3::new(i.x + 1.0, i.y + 1.0, i.z));
    let e = hash_vec3(Vec3::new(i.x, i.y, i.z + 1.0));
    let f_val = hash_vec3(Vec3::new(i.x + 1.0, i.y, i.z + 1.0));
    let g = hash_vec3(Vec3::new(i.x, i.y + 1.0, i.z + 1.0));
    let h = hash_vec3(Vec3::new(i.x + 1.0, i.y + 1.0, i.z + 1.0));
    
    // Trilinear interpolation
    let x1 = a + (b - a) * u.x;
    let x2 = c + (d - c) * u.x;
    let y1 = x1 + (x2 - x1) * u.y;
    
    let x3 = e + (f_val - e) * u.x;
    let x4 = g + (h - g) * u.x;
    let y2 = x3 + (x4 - x3) * u.y;
    
    y1 + (y2 - y1) * u.z
}

fn fbm(p: Vec3, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += amplitude * noise(Vec3::new(p.x * frequency, p.y * frequency, p.z * frequency));
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value
}

// Star/Sun Shader
pub fn star_shader(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, model_position: Vec3, _world_position: Vec3, normal: Vec3, _tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };

    let light_dir = Vec3::new(-0.2, 0.6, -1.0).normalize();
    let intensity = dot(&normal, &light_dir).max(0.0);

    // Base yellow-orange color
    let base_color = Vec3::new(1.0, 0.7, 0.3);

    // Add noise for surface variation using model space position
    let noise_value = fbm(model_position * 5.0, 3);
    let variation = 0.1 * noise_value;

    // Add bright center effect (towards local Y axis for visual interest)
    let center_dist = (model_position.x * model_position.x + model_position.y * model_position.y).sqrt();
    let center_glow = (1.0 - center_dist.min(1.0)).powf(2.0) * 0.3;

    // Add solar flare effect based on the eye-facing component
    let flare = dot(&normal, &light_dir).max(0.0).powf(3.0) * 0.3;

    let r = (base_color.x + variation + center_glow + flare).clamp(0.0, 1.0);
    let g = (base_color.y + variation * 0.5 + center_glow * 0.8 + flare * 0.9).clamp(0.0, 1.0);
    let b = (base_color.z + variation * 0.3 + center_glow * 0.5).clamp(0.0, 1.0);

    // Enhance the impression of volume using the normal's Z component (camera facing)
    let light_factor = intensity * 0.6 + 0.4;
    let final_color = Vec3::new(r * light_factor, g * light_factor, b * light_factor);

    Color::from_float(final_color.x, final_color.y, final_color.z)
}

// Rocky Planet Shader (Earth-like)
pub fn rocky_planet_shader(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, model_position: Vec3, world_position: Vec3, normal: Vec3, _tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let mut light_dir = Vec3::new(0.0, 0.0, 0.0) - world_position;
    let distance = light_dir.magnitude().max(1.0);
    light_dir = light_dir / distance;
    let diffuse = dot(&normal, &light_dir).max(0.0);
    let attenuation = (1200.0 / distance).clamp(0.35, 1.0);
    let light_factor = diffuse * 0.85 + 0.15;
    let shading = (light_factor * attenuation).clamp(0.1, 1.0);

    // Use spherical coordinates for consistent mapping in model space
    let radius = model_position.magnitude().max(1e-5);
    let lat = (model_position.y / radius).clamp(-1.0, 1.0).acos();

    // Layer 1: Ocean/Continents base
    let continent_noise = fbm(model_position * 2.0, 4);
    let is_land = continent_noise > 0.1;

    // Layer 2: Ocean depth variation
    let ocean_depth = if !is_land {
        fbm(model_position * 3.0, 3) * 0.3 + 0.7
    } else {
        0.0
    };

    // Layer 3: Land elevation
    let elevation = if is_land {
        fbm(model_position * 4.0, 3) * 0.5 + 0.5
    } else {
        0.0
    };

    // Layer 4: Climate zones (latitude-based)
    let climate = (lat / std::f32::consts::PI).abs();
    let is_polar = climate > 0.7;
    let is_tropical = climate < 0.3;

    // Calculate colors
    let base_color = if is_land {
        let base_green = Vec3::new(0.2, 0.6, 0.2);
        let brown = Vec3::new(0.4, 0.3, 0.2);
        let snow = Vec3::new(0.9, 0.9, 0.95);

        let land_color = if is_polar {
            Vec3::new(
                base_green.x * 0.3 + snow.x * 0.7,
                base_green.y * 0.3 + snow.y * 0.7,
                base_green.z * 0.3 + snow.z * 0.7
            )
        } else if is_tropical {
            Vec3::new(
                base_green.x * 0.8 + brown.x * 0.2,
                base_green.y * 0.8 + brown.y * 0.2,
                base_green.z * 0.8 + brown.z * 0.2
            )
        } else {
            let mix_factor = elevation * 0.5;
            Vec3::new(
                base_green.x * (1.0 - mix_factor) + brown.x * mix_factor,
                base_green.y * (1.0 - mix_factor) + brown.y * mix_factor,
                base_green.z * (1.0 - mix_factor) + brown.z * mix_factor
            )
        };

        land_color
    } else {
        let deep_blue = Vec3::new(0.0, 0.2, 0.5);
        let shallow_blue = Vec3::new(0.2, 0.4, 0.7);

        Vec3::new(
            deep_blue.x * ocean_depth + shallow_blue.x * (1.0 - ocean_depth),
            deep_blue.y * ocean_depth + shallow_blue.y * (1.0 - ocean_depth),
            deep_blue.z * ocean_depth + shallow_blue.z * (1.0 - ocean_depth)
        )
    };

    let final_color = Vec3::new(
        base_color.x * shading,
        base_color.y * shading,
        base_color.z * shading,
    );

    Color::from_float(final_color.x, final_color.y, final_color.z)
}

// Gas Giant Shader (Jupiter-like)
pub fn gas_giant_shader(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, model_position: Vec3, world_position: Vec3, normal: Vec3, _tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let mut light_dir = Vec3::new(0.0, 0.0, 0.0) - world_position;
    let distance = light_dir.magnitude().max(1.0);
    light_dir = light_dir / distance;
    let diffuse = dot(&normal, &light_dir).max(0.0);
    let attenuation = (1500.0 / distance).clamp(0.35, 1.0);
    let shading = (diffuse * 0.8 + 0.2) * attenuation;

    let radius = model_position.magnitude().max(1e-5);
    let lat = (model_position.y / radius).clamp(-1.0, 1.0);

    // Layer 1: Base band structure
    let band_freq = 8.0;
    let band = (lat * band_freq).sin() * 0.5 + 0.5;

    // Layer 2: Turbulence for swirls
    let turbulence = fbm(model_position * 3.0, 4);
    let swirl = (turbulence * 2.0 - 1.0) * 0.3;

    // Layer 3: Color variation within bands
    let color_variation = fbm(model_position * 5.0, 3) * 0.2;

    // Layer 4: Great Red Spot-like feature
    let spot_pos = Vec3::new(0.0, 0.3, 0.8);
    let mut normalized_pos = model_position;
    if normalized_pos.magnitude_squared() > 0.0 {
        normalized_pos = normalized_pos.normalize();
    }
    let spot_dist = (normalized_pos - spot_pos).magnitude();
    let spot = if spot_dist < 0.3 {
        (1.0 - spot_dist / 0.3).powf(2.0) * 0.4
    } else {
        0.0
    };

    // Jupiter-like colors: browns, oranges, whites
    let dark_band = Vec3::new(0.5, 0.3, 0.2);
    let light_band = Vec3::new(0.8, 0.7, 0.6);
    let red_spot = Vec3::new(0.8, 0.3, 0.2);

    // Mix bands
    let base_color = dark_band * (1.0 - band) + light_band * band;

    // Add swirl
    let swirled_color = base_color + Vec3::new(swirl, swirl * 0.5, -swirl * 0.3);

    // Add color variation
    let varied_color = swirled_color + Vec3::new(color_variation, color_variation * 0.5, -color_variation * 0.3);

    // Add red spot
    let final_base = varied_color * (1.0 - spot) + red_spot * spot;

    let final_color = Vec3::new(
        (final_base.x * shading).clamp(0.0, 1.0),
        (final_base.y * shading).clamp(0.0, 1.0),
        (final_base.z * shading).clamp(0.0, 1.0)
    );

    Color::from_float(final_color.x, final_color.y, final_color.z)
}

// Moon Shader (simple gray with craters)
pub fn moon_shader(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, model_position: Vec3, world_position: Vec3, normal: Vec3, _tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let mut light_dir = Vec3::new(0.0, 0.0, 0.0) - world_position;
    let distance = light_dir.magnitude().max(1.0);
    light_dir = light_dir / distance;
    let diffuse = dot(&normal, &light_dir).max(0.0);
    let attenuation = (1000.0 / distance).clamp(0.35, 1.0);
    let shading = (diffuse * 0.9 + 0.1) * attenuation;

    // Base gray color
    let base_gray = 0.5;

    // Add crater-like noise using model position
    let craters = fbm(model_position * 8.0, 4);
    let crater_depth = (craters - 0.5).abs() * 2.0;
    let crater = if crater_depth > 0.7 {
        crater_depth * 0.3
    } else {
        0.0
    };

    let gray = (base_gray - crater).clamp(0.2, 0.8);

    let final_gray = (gray * shading).clamp(0.1, 1.0);

    Color::from_float(final_gray, final_gray, final_gray)
}

// Ship Shader (metallic blue-gray)
pub fn ship_shader_metallic(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, _model_position: Vec3, world_position: Vec3, normal: Vec3, _tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let mut light_dir = Vec3::new(0.0, 0.0, 0.0) - world_position;
    let distance = light_dir.magnitude().max(1.0);
    light_dir = light_dir / distance;
    let diffuse = dot(&normal, &light_dir).max(0.0);
    let attenuation = (800.0 / distance).clamp(0.3, 1.0);
    let brightness = (diffuse * 0.8 + 0.2) * attenuation;

    // Metallic blue-gray color
    let base_color = Vec3::new(0.4, 0.5, 0.7);
    let highlight = Vec3::new(0.6, 0.7, 0.9);
    let color = base_color * (1.0 - diffuse * 0.3) + highlight * (diffuse * 0.3);

    Color::from_float(
        (color.x * brightness).clamp(0.0, 1.0),
        (color.y * brightness).clamp(0.0, 1.0),
        (color.z * brightness).clamp(0.0, 1.0)
    )
}

// Ring Shader (simple gradient)
pub fn ring_shader(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex, model_position: Vec3, world_position: Vec3, normal: Vec3, tex_coords: Vec2) -> Color {
    let normal = if normal.magnitude_squared() > 0.0 {
        normal.normalize()
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };

    let mut light_dir = Vec3::new(0.0, 0.0, 0.0) - world_position;
    let distance = light_dir.magnitude().max(1.0);
    light_dir = light_dir / distance;
    let diffuse = dot(&normal, &light_dir).max(0.0);
    let attenuation = (1400.0 / distance).clamp(0.35, 1.0);
    let light_factor = (diffuse * 0.6 + 0.4) * attenuation;

    // Use texture coordinates for radial gradient
    let radial = tex_coords.y; // 0.0 = inner, 1.0 = outer

    // Dusty brown-gray color
    let inner_color = Vec3::new(0.4, 0.35, 0.3);
    let outer_color = Vec3::new(0.5, 0.45, 0.4);

    let color = Vec3::new(
        inner_color.x * (1.0 - radial) + outer_color.x * radial,
        inner_color.y * (1.0 - radial) + outer_color.y * radial,
        inner_color.z * (1.0 - radial) + outer_color.z * radial
    );

    // Add some variation using model position for a subtle texture
    let variation = fbm(model_position * 10.0, 2) * 0.1;
    let final_color = Vec3::new(
        (color.x + variation).clamp(0.0, 1.0),
        (color.y + variation).clamp(0.0, 1.0),
        (color.z + variation).clamp(0.0, 1.0)
    );

    let ring_final = Vec3::new(final_color.x * light_factor, final_color.y * light_factor, final_color.z * light_factor);

    Color::from_float(
        ring_final.x.clamp(0.0, 1.0),
        ring_final.y.clamp(0.0, 1.0),
        ring_final.z.clamp(0.0, 1.0)
    )
}

