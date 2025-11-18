use nalgebra_glm::{Vec2, Vec3};

use crate::color::Color;
use crate::fragment::Fragment;
use crate::fragment_shaders::FragmentShader;
use crate::line::line;
use crate::vertex::Vertex;

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Draw the three sides of the triangle
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));

    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    triangle_with_shader(v1, v2, v3, |_, _, _, _, _, _, _| Color::new(100, 100, 100))
}

pub fn triangle_with_shader(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    fragment_shader: FragmentShader,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    // Clamp bounding box to screen bounds to avoid rendering off-screen
    // We'll use reasonable screen bounds (assuming max 2000x2000 for safety)
    let screen_max_x = 2000i32;
    let screen_max_y = 2000i32;
    let screen_min_x = -100i32;
    let screen_min_y = -100i32;

    let clamped_min_x = min_x.max(screen_min_x);
    let clamped_min_y = min_y.max(screen_min_y);
    let clamped_max_x = max_x.min(screen_max_x);
    let clamped_max_y = max_y.min(screen_max_y);

    // Skip if triangle is completely outside screen
    if clamped_min_x > clamped_max_x || clamped_min_y > clamped_max_y {
        return fragments;
    }

    let triangle_area = edge_function(&a, &b, &c);

    // Skip if triangle area is too small or invalid
    if triangle_area.abs() < 0.0001 {
        return fragments;
    }

    // Iterate over each pixel in the clamped bounding box
    for y in clamped_min_y..=clamped_max_y {
        for x in clamped_min_x..=clamped_max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            // Calculate barycentric coordinates
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            // Check if the point is inside the triangle
            if w1 >= 0.0 && w1 <= 1.0 && w2 >= 0.0 && w2 <= 1.0 && w3 >= 0.0 && w3 <= 1.0 {
                // Interpolate normal
                let mut normal = Vec3::new(
                    v1.transformed_normal.x * w1
                        + v2.transformed_normal.x * w2
                        + v3.transformed_normal.x * w3,
                    v1.transformed_normal.y * w1
                        + v2.transformed_normal.y * w2
                        + v3.transformed_normal.y * w3,
                    v1.transformed_normal.z * w1
                        + v2.transformed_normal.z * w2
                        + v3.transformed_normal.z * w3,
                );
                if normal.magnitude_squared() > 0.0 {
                    normal = normal.normalize();
                }

                // Interpolate position in model space
                let position = Vec3::new(
                    v1.position.x * w1 + v2.position.x * w2 + v3.position.x * w3,
                    v1.position.y * w1 + v2.position.y * w2 + v3.position.y * w3,
                    v1.position.z * w1 + v2.position.z * w2 + v3.position.z * w3,
                );

                // Interpolate world position
                let world_position = Vec3::new(
                    v1.world_position.x * w1 + v2.world_position.x * w2 + v3.world_position.x * w3,
                    v1.world_position.y * w1 + v2.world_position.y * w2 + v3.world_position.y * w3,
                    v1.world_position.z * w1 + v2.world_position.z * w2 + v3.world_position.z * w3,
                );

                // Interpolate texture coordinates
                let tex_coords = Vec2::new(
                    v1.tex_coords.x * w1 + v2.tex_coords.x * w2 + v3.tex_coords.x * w3,
                    v1.tex_coords.y * w1 + v2.tex_coords.y * w2 + v3.tex_coords.y * w3,
                );

                // Use fragment shader to calculate color
                let color =
                    fragment_shader(v1, v2, v3, position, world_position, normal, tex_coords);

                // Interpolate depth
                let depth = a.z * w1 + b.z * w2 + c.z * w3;

                fragments.push(Fragment::new(x as f32, y as f32, color, depth));
            }
        }
    }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    // Clamp coordinates to reasonable range to avoid generating too many fragments
    let clamp_coord = |x: f32| x.clamp(-10000.0, 20000.0);
    let v1_x = clamp_coord(v1.x);
    let v1_y = clamp_coord(v1.y);
    let v2_x = clamp_coord(v2.x);
    let v2_y = clamp_coord(v2.y);
    let v3_x = clamp_coord(v3.x);
    let v3_y = clamp_coord(v3.y);

    let min_x = v1_x.min(v2_x).min(v3_x).floor() as i32;
    let min_y = v1_y.min(v2_y).min(v3_y).floor() as i32;
    let max_x = v1_x.max(v2_x).max(v3_x).ceil() as i32;
    let max_y = v1_y.max(v2_y).max(v3_y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
