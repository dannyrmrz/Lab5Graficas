use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position through model, view, and projection
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let world_position4 = uniforms.model_matrix * position;
  let world_position = Vec3::new(world_position4.x, world_position4.y, world_position4.z);
  let mvp = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix;
  let transformed = mvp * position;

  // Perform perspective division to get NDC coordinates (-1 to 1)
  let w = if transformed.w.abs() > 0.0001 { transformed.w } else { 1.0 };
  let ndc = Vec3::new(
    (transformed.x / w).clamp(-10.0, 10.0),  // Clamp to prevent extreme values
    (transformed.y / w).clamp(-10.0, 10.0),
    (transformed.z / w).clamp(-10.0, 10.0)
  );
  
  // Convert NDC to screen coordinates
  // Convert from NDC [-1, 1] to screen [0, width/height]
  // Y is inverted because screen Y increases downward
  // Clamp NDC to valid range before conversion
  let ndc_x = ndc.x.clamp(-1.0, 1.0);
  let ndc_y = ndc.y.clamp(-1.0, 1.0);
  
  let transformed_position = Vec3::new(
    (ndc_x + 1.0) * 0.5 * uniforms.screen_width,
    (1.0 - ndc_y) * 0.5 * uniforms.screen_height,  // Invert Y
    ndc.z
  );

  // Transform normal (only through model matrix, not view/projection)
  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());
  let mut transformed_normal = normal_matrix * vertex.normal;
  if transformed_normal.magnitude_squared() > 0.0 {
    transformed_normal = transformed_normal.normalize();
  }

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal,
    world_position,
  }
}

