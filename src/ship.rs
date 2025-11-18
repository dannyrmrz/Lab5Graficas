use crate::obj::Obj;
use crate::vertex::Vertex;

pub fn load_ship() -> Result<Vec<Vertex>, String> {
    // Try to load the ship model from OBJ file
    match Obj::load("assets/models/Nave.obj") {
        Ok(obj) => Ok(obj.get_vertex_array()),
        Err(e) => {
            eprintln!(
                "Warning: Could not load ship model from assets/models/Nave.obj: {:?}",
                e
            );
            eprintln!("Falling back to procedural ship model.");
            // Fallback to procedural model if OBJ loading fails
            Ok(generate_ship_fallback())
        }
    }
}

fn generate_ship_fallback() -> Vec<Vertex> {
    use nalgebra_glm::Vec3;
    let mut vertices = Vec::new();

    // Simple ship model: a triangular prism shape (fallback)
    // Front face (pointed forward)
    let front = Vec3::new(0.0, 0.0, 1.0);
    let top_front = Vec3::new(0.0, 0.5, 0.5);
    let bottom_front = Vec3::new(0.0, -0.5, 0.5);
    let left_front = Vec3::new(-0.3, 0.0, 0.5);
    let right_front = Vec3::new(0.3, 0.0, 0.5);

    // Back face
    let back_center = Vec3::new(0.0, 0.0, -1.0);
    let top_back = Vec3::new(0.0, 0.3, -0.5);
    let bottom_back = Vec3::new(0.0, -0.3, -0.5);
    let left_back = Vec3::new(-0.2, 0.0, -0.5);
    let right_back = Vec3::new(0.2, 0.0, -0.5);

    // Helper function to create vertex
    let make_vertex =
        |pos: Vec3, normal: Vec3| Vertex::new(pos, normal, nalgebra_glm::Vec2::new(0.0, 0.0));

    // Front triangle
    let front_normal = Vec3::new(0.0, 0.0, 1.0);
    vertices.push(make_vertex(front, front_normal));
    vertices.push(make_vertex(top_front, front_normal));
    vertices.push(make_vertex(left_front, front_normal));

    vertices.push(make_vertex(front, front_normal));
    vertices.push(make_vertex(left_front, front_normal));
    vertices.push(make_vertex(bottom_front, front_normal));

    vertices.push(make_vertex(front, front_normal));
    vertices.push(make_vertex(bottom_front, front_normal));
    vertices.push(make_vertex(right_front, front_normal));

    vertices.push(make_vertex(front, front_normal));
    vertices.push(make_vertex(right_front, front_normal));
    vertices.push(make_vertex(top_front, front_normal));

    // Back face
    let back_normal = Vec3::new(0.0, 0.0, -1.0);
    vertices.push(make_vertex(back_center, back_normal));
    vertices.push(make_vertex(left_back, back_normal));
    vertices.push(make_vertex(top_back, back_normal));

    vertices.push(make_vertex(back_center, back_normal));
    vertices.push(make_vertex(bottom_back, back_normal));
    vertices.push(make_vertex(left_back, back_normal));

    vertices.push(make_vertex(back_center, back_normal));
    vertices.push(make_vertex(right_back, back_normal));
    vertices.push(make_vertex(bottom_back, back_normal));

    vertices.push(make_vertex(back_center, back_normal));
    vertices.push(make_vertex(top_back, back_normal));
    vertices.push(make_vertex(right_back, back_normal));

    // Top face
    let top_normal = Vec3::new(0.0, 1.0, 0.0);
    vertices.push(make_vertex(top_front, top_normal));
    vertices.push(make_vertex(right_front, top_normal));
    vertices.push(make_vertex(right_back, top_normal));

    vertices.push(make_vertex(top_front, top_normal));
    vertices.push(make_vertex(right_back, top_normal));
    vertices.push(make_vertex(top_back, top_normal));

    vertices.push(make_vertex(top_front, top_normal));
    vertices.push(make_vertex(top_back, top_normal));
    vertices.push(make_vertex(left_back, top_normal));

    vertices.push(make_vertex(top_front, top_normal));
    vertices.push(make_vertex(left_back, top_normal));
    vertices.push(make_vertex(left_front, top_normal));

    // Bottom face
    let bottom_normal = Vec3::new(0.0, -1.0, 0.0);
    vertices.push(make_vertex(bottom_front, bottom_normal));
    vertices.push(make_vertex(left_back, bottom_normal));
    vertices.push(make_vertex(right_back, bottom_normal));

    vertices.push(make_vertex(bottom_front, bottom_normal));
    vertices.push(make_vertex(right_back, bottom_normal));
    vertices.push(make_vertex(right_front, bottom_normal));

    vertices.push(make_vertex(bottom_front, bottom_normal));
    vertices.push(make_vertex(left_front, bottom_normal));
    vertices.push(make_vertex(left_back, bottom_normal));

    // Left side
    let left_normal = Vec3::new(-1.0, 0.0, 0.0);
    vertices.push(make_vertex(left_front, left_normal));
    vertices.push(make_vertex(left_back, left_normal));
    vertices.push(make_vertex(bottom_front, left_normal));

    vertices.push(make_vertex(left_front, left_normal));
    vertices.push(make_vertex(top_front, left_normal));
    vertices.push(make_vertex(left_back, left_normal));

    // Right side
    let right_normal = Vec3::new(1.0, 0.0, 0.0);
    vertices.push(make_vertex(right_front, right_normal));
    vertices.push(make_vertex(bottom_front, right_normal));
    vertices.push(make_vertex(right_back, right_normal));

    vertices.push(make_vertex(right_front, right_normal));
    vertices.push(make_vertex(right_back, right_normal));
    vertices.push(make_vertex(top_front, right_normal));

    vertices
}
