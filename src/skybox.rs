use crate::color::Color;
use crate::texture::Texture;
use crate::vertex::Vertex;
use nalgebra_glm::Vec3;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    static SKYBOX_TEXTURE: RefCell<Option<Arc<Texture>>> = RefCell::new(None);
}

pub struct Skybox {
    pub texture: Arc<Texture>,
}

impl Skybox {
    pub fn load(filename: &str) -> Result<Self, String> {
        let texture = Texture::load(filename)?;
        Ok(Skybox {
            texture: Arc::new(texture),
        })
    }

    pub fn set_active(&self) {
        SKYBOX_TEXTURE.with(|tex| {
            *tex.borrow_mut() = Some(Arc::clone(&self.texture));
        });
    }

    pub fn clear_active() {
        SKYBOX_TEXTURE.with(|tex| {
            *tex.borrow_mut() = None;
        });
    }
}

pub fn skybox_shader_textured(
    _v1: &Vertex,
    _v2: &Vertex,
    _v3: &Vertex,
    model_position: Vec3,
    _world_position: Vec3,
    _normal: Vec3,
    _tex_coords: nalgebra_glm::Vec2,
) -> Color {
    SKYBOX_TEXTURE.with(|tex_opt| {
        if let Some(texture) = tex_opt.borrow().as_ref() {
            // Convert 3D position to UV coordinates for equirectangular mapping
            let pos = model_position.normalize();

            // Calculate UV coordinates from spherical coordinates
            // For equirectangular projection:
            // u = atan2(z, x) / (2 * PI) + 0.5
            // v = acos(y) / PI
            let u = pos.z.atan2(pos.x) / (2.0 * std::f32::consts::PI) + 0.5;
            let v = pos.y.acos() / std::f32::consts::PI;

            // Sample texture with bilinear filtering
            let color_hex = texture.sample_bilinear(u, v);

            // Convert hex to Color
            Color::from_hex(color_hex)
        } else {
            // Fallback to procedural if texture not set
            skybox_shader_procedural(
                _v1,
                _v2,
                _v3,
                model_position,
                _world_position,
                _normal,
                _tex_coords,
            )
        }
    })
}

pub fn generate_skybox_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let size = 800.0; // Keep the cube relatively close to reduce raster area

    // Create a simple sphere for the skybox (inverted normals)
    let segments = 16;

    for i in 0..=segments {
        let v = i as f32 / segments as f32;
        let theta = v * std::f32::consts::PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for j in 0..=segments {
            let u = j as f32 / segments as f32;
            let phi = u * 2.0 * std::f32::consts::PI;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = size * sin_theta * cos_phi;
            let y = size * cos_theta;
            let z = size * sin_theta * sin_phi;

            let position = Vec3::new(x, y, z);
            let normal = -position.normalize(); // Inverted normals for skybox

            vertices.push(Vertex::new(position, normal, nalgebra_glm::Vec2::new(u, v)));
        }
    }

    // Generate triangles
    let mut indexed_vertices = Vec::new();

    for i in 0..segments {
        for j in 0..segments {
            let current = (i * (segments + 1) + j) as usize;
            let next = (i * (segments + 1) + j + 1) as usize;
            let below = ((i + 1) * (segments + 1) + j) as usize;
            let below_next = ((i + 1) * (segments + 1) + j + 1) as usize;

            // First triangle
            indexed_vertices.push(vertices[current].clone());
            indexed_vertices.push(vertices[next].clone());
            indexed_vertices.push(vertices[below].clone());

            // Second triangle
            indexed_vertices.push(vertices[next].clone());
            indexed_vertices.push(vertices[below_next].clone());
            indexed_vertices.push(vertices[below].clone());
        }
    }

    indexed_vertices
}

// Fallback procedural shader if texture loading fails
pub fn skybox_shader_procedural(
    _v1: &Vertex,
    _v2: &Vertex,
    _v3: &Vertex,
    model_position: Vec3,
    _world_position: Vec3,
    _normal: Vec3,
    _tex_coords: nalgebra_glm::Vec2,
) -> Color {
    // Create starfield effect
    let pos = model_position.normalize();

    // Use hash function to create random stars
    let hash = |n: f32| -> f32 {
        let x = (n * 12.9898).sin() * 43758.5453;
        x - x.floor()
    };

    let star_hash = hash(pos.x * 100.0 + pos.y * 200.0 + pos.z * 300.0);

    if star_hash > 0.995 {
        // Bright star
        let brightness = (star_hash - 0.995) / 0.005;
        Color::from_float(1.0, 1.0, 1.0 * brightness)
    } else if star_hash > 0.98 {
        // Dim star
        let brightness = (star_hash - 0.98) / 0.015 * 0.5;
        Color::from_float(brightness, brightness, brightness)
    } else {
        // Deep space
        Color::from_float(0.01, 0.01, 0.02)
    }
}
