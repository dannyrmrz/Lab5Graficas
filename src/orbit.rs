use crate::color::Color;
use crate::vertex::Vertex;
use nalgebra_glm::Vec3;

pub fn generate_orbit_path(radius: f32, segments: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        let y = 0.0; // On ecliptic plane

        let position = Vec3::new(x, y, z);
        let normal = Vec3::new(0.0, 1.0, 0.0);

        vertices.push(Vertex::new(
            position,
            normal,
            nalgebra_glm::Vec2::new(0.0, 0.0),
        ));
    }

    vertices
}

pub fn orbit_shader(
    _v1: &Vertex,
    _v2: &Vertex,
    _v3: &Vertex,
    _model_position: Vec3,
    _world_position: Vec3,
    _normal: Vec3,
    _tex_coords: nalgebra_glm::Vec2,
) -> Color {
    // Simple white/cyan orbit line
    Color::from_float(0.3, 0.6, 0.9)
}
