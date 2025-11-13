use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod sphere;
mod fragment_shaders;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle_with_shader;
use shaders::vertex_shader;
use sphere::{generate_sphere, generate_ring};
use fragment_shaders::{star_shader, rocky_planet_shader, gas_giant_shader, moon_shader, ring_shader};


pub struct Uniforms {
    model_matrix: Mat4,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], fragment_shader: fragment_shaders::FragmentShader) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle_with_shader(&tri[0], &tri[1], &tri[2], fragment_shader));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment.color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

#[derive(Clone, Copy)]
enum ShaderMode {
    Star,
    RockyPlanet,
    GasGiant,
    All,
}

fn main() {
    let window_width = 1200;
    let window_height = 800;
    let framebuffer_width = 1200;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar - Shaders de Planetas",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x000011);

    // Generate spheres
    let sphere_segments = 50;
    let star_sphere = generate_sphere(1.0, sphere_segments);
    let planet_sphere = generate_sphere(1.0, sphere_segments);
    let gas_giant_sphere = generate_sphere(1.0, sphere_segments);
    let moon_sphere = generate_sphere(0.3, 30);
    let ring = generate_ring(1.2, 2.0, 60);

    let mut shader_mode = ShaderMode::All;
    let mut time = 0.0f32;
    let mut prev_key1 = false;
    let mut prev_key2 = false;
    let mut prev_key3 = false;
    let mut prev_key0 = false;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Switch shader mode with number keys (with debouncing)
        let key1 = window.is_key_down(Key::Key1);
        let key2 = window.is_key_down(Key::Key2);
        let key3 = window.is_key_down(Key::Key3);
        let key0 = window.is_key_down(Key::Key0);

        if key1 && !prev_key1 {
            shader_mode = ShaderMode::Star;
        }
        if key2 && !prev_key2 {
            shader_mode = ShaderMode::RockyPlanet;
        }
        if key3 && !prev_key3 {
            shader_mode = ShaderMode::GasGiant;
        }
        if key0 && !prev_key0 {
            shader_mode = ShaderMode::All;
        }

        prev_key1 = key1;
        prev_key2 = key2;
        prev_key3 = key3;
        prev_key0 = key0;

        framebuffer.clear();
        time += 0.01;

        match shader_mode {
            ShaderMode::Star => {
                // Render star in center
                let model_matrix = create_model_matrix(
                    Vec3::new(600.0, 400.0, 0.0),
                    150.0,
                    Vec3::new(time * 0.5, time * 0.3, 0.0)
                );
                let uniforms = Uniforms { model_matrix };
                render(&mut framebuffer, &uniforms, &star_sphere, star_shader);
            }
            ShaderMode::RockyPlanet => {
                // Render rocky planet with moon
                let planet_matrix = create_model_matrix(
                    Vec3::new(600.0, 400.0, 0.0),
                    120.0,
                    Vec3::new(time * 0.3, time * 0.5, 0.0)
                );
                let planet_uniforms = Uniforms { model_matrix: planet_matrix };
                render(&mut framebuffer, &planet_uniforms, &planet_sphere, rocky_planet_shader);

                // Render moon orbiting the planet
                let moon_distance = 200.0;
                let moon_angle = time * 0.8;
                let moon_x = 600.0 + moon_distance * moon_angle.cos();
                let moon_y = 400.0 + moon_distance * moon_angle.sin();
                let moon_matrix = create_model_matrix(
                    Vec3::new(moon_x, moon_y, 0.0),
                    40.0,
                    Vec3::new(time * 0.4, time * 0.4, 0.0)
                );
                let moon_uniforms = Uniforms { model_matrix: moon_matrix };
                render(&mut framebuffer, &moon_uniforms, &moon_sphere, moon_shader);
            }
            ShaderMode::GasGiant => {
                // Render gas giant with rings
                let planet_matrix = create_model_matrix(
                    Vec3::new(600.0, 400.0, 0.0),
                    140.0,
                    Vec3::new(time * 0.2, time * 0.4, 0.0)
                );
                let planet_uniforms = Uniforms { model_matrix: planet_matrix };
                render(&mut framebuffer, &planet_uniforms, &gas_giant_sphere, gas_giant_shader);

                // Render rings
                let ring_matrix = create_model_matrix(
                    Vec3::new(600.0, 400.0, 0.0),
                    140.0,
                    Vec3::new(0.0, 0.0, time * 0.1)
                );
                let ring_uniforms = Uniforms { model_matrix: ring_matrix };
                render(&mut framebuffer, &ring_uniforms, &ring, ring_shader);
            }
            ShaderMode::All => {
                // Render all three planets side by side
                // Star (left)
                let star_matrix = create_model_matrix(
                    Vec3::new(250.0, 400.0, 0.0),
                    100.0,
                    Vec3::new(time * 0.5, time * 0.3, 0.0)
                );
                let star_uniforms = Uniforms { model_matrix: star_matrix };
                render(&mut framebuffer, &star_uniforms, &star_sphere, star_shader);

                // Rocky Planet (center) with moon
                let planet_matrix = create_model_matrix(
                    Vec3::new(600.0, 400.0, 0.0),
                    90.0,
                    Vec3::new(time * 0.3, time * 0.5, 0.0)
                );
                let planet_uniforms = Uniforms { model_matrix: planet_matrix };
                render(&mut framebuffer, &planet_uniforms, &planet_sphere, rocky_planet_shader);

                // Moon
                let moon_distance = 150.0;
                let moon_angle = time * 0.8;
                let moon_x = 600.0 + moon_distance * moon_angle.cos();
                let moon_y = 400.0 + moon_distance * moon_angle.sin();
                let moon_matrix = create_model_matrix(
                    Vec3::new(moon_x, moon_y, 0.0),
                    30.0,
                    Vec3::new(time * 0.4, time * 0.4, 0.0)
                );
                let moon_uniforms = Uniforms { model_matrix: moon_matrix };
                render(&mut framebuffer, &moon_uniforms, &moon_sphere, moon_shader);

                // Gas Giant (right) with rings
                let gas_matrix = create_model_matrix(
                    Vec3::new(950.0, 400.0, 0.0),
                    110.0,
                    Vec3::new(time * 0.2, time * 0.4, 0.0)
                );
                let gas_uniforms = Uniforms { model_matrix: gas_matrix };
                render(&mut framebuffer, &gas_uniforms, &gas_giant_sphere, gas_giant_shader);

                // Rings for gas giant
                let ring_matrix = create_model_matrix(
                    Vec3::new(950.0, 400.0, 0.0),
                    110.0,
                    Vec3::new(0.0, 0.0, time * 0.1)
                );
                let ring_uniforms = Uniforms { model_matrix: ring_matrix };
                render(&mut framebuffer, &ring_uniforms, &ring, ring_shader);
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

