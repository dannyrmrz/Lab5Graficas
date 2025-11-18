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
mod solar_system;
mod camera;
mod ship;
mod skybox;
mod orbit;
mod texture;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle_with_shader;
use shaders::vertex_shader;
use sphere::{generate_sphere, generate_ring};
use solar_system::SolarSystem;
use camera::Camera;
use ship::load_ship;
use skybox::{generate_skybox_vertices, skybox_shader_procedural, skybox_shader_textured, Skybox};
use orbit::{generate_orbit_path, orbit_shader};
use fragment_shaders::ship_shader_metallic;
use line::line;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub screen_width: f32,
    pub screen_height: f32,
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
    let mut debug_count = 0;
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        
        // Debug: Check first few transformed vertices
        if debug_count < 3 {
            let pos = transformed.transformed_position;
            if pos.x.is_nan() || pos.y.is_nan() || pos.z.is_nan() {
                eprintln!("Warning: NaN in transformed position at index {}", debug_count);
            }
            if pos.x < -1000.0 || pos.x > 3000.0 || pos.y < -1000.0 || pos.y > 3000.0 {
                eprintln!("Warning: Transformed position out of expected range: {:?}", pos);
            }
            debug_count += 1;
        }
        
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
        // Limit fragments to avoid hanging
        if fragments.len() > 500000 {
            eprintln!("Warning: Too many fragments ({}), stopping rasterization", fragments.len());
            break;
        }
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


fn main() {
    println!("Initializing application...");
    let window_width = 1200;
    let window_height = 800;
    let framebuffer_width = 1200;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    println!("Creating framebuffer...");
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    println!("Creating window...");
    let mut window = Window::new(
        "Sistema Solar - Simulación Completa",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x000011);
    println!("Framebuffer initialized");

    // Initialize solar system
    println!("Initializing solar system...");
    let mut solar_system = SolarSystem::new();
    
    // Initialize camera
    println!("Initializing camera...");
    let mut camera = Camera::new(framebuffer_width as f32, framebuffer_height as f32);
    
    // Generate geometry
    println!("Generating geometry...");
    let sphere_segments = 40;
    let star_sphere = generate_sphere(1.0, sphere_segments);
    let planet_sphere = generate_sphere(1.0, sphere_segments);
    let gas_giant_sphere = generate_sphere(1.0, sphere_segments);
    let moon_sphere = generate_sphere(1.0, 25);
    let _ring = generate_ring(1.2, 2.0, 60);
    println!("Geometry generated");
    // Load ship model from OBJ file (with fallback to procedural model)
    let ship_model = load_ship().unwrap_or_else(|e| {
        eprintln!("Error loading ship model: {}", e);
        Vec::new()
    });
    println!("Generating skybox vertices...");
    let skybox_vertices = generate_skybox_vertices();
    println!("Skybox vertices generated: {} vertices", skybox_vertices.len());
    
    // Load skybox texture
    println!("Loading skybox texture...");
    let skybox = match Skybox::load("assets/models/skybox.jpg") {
        Ok(skybox) => {
            println!("Skybox texture loaded successfully from assets/models/skybox.jpg");
            skybox.set_active();
            Some(skybox)
        }
        Err(e) => {
            eprintln!("Warning: Could not load skybox texture: {}", e);
            eprintln!("Using procedural skybox shader instead.");
            None
        }
    };
    let skybox_shader = if skybox.is_some() {
        skybox_shader_textured
    } else {
        skybox_shader_procedural
    };
    
    // Generate orbit paths
    println!("Generating orbit paths...");
    let mut orbit_paths = Vec::new();
    for body in &solar_system.bodies {
        if body.orbit_radius > 0.0 {
            orbit_paths.push(generate_orbit_path(body.orbit_radius, 100));
        }
    }
    println!("Orbit paths generated: {} paths", orbit_paths.len());

    let mut time = 0.0f32;
    let mut prev_keys = [false; 10];
    let mut current_body_index = 0;
    let mut show_orbits = true;
    let mut show_ship = true;
    let mut camera_mode = 0; // 0 = free, 1 = follow, 2 = orbit

    println!("Starting render loop. Camera position: {:?}, target: {:?}", camera.position, camera.target);
    println!("Press ESC to exit, WASD to move, arrow keys to rotate, 1-6 to warp to planets");

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Handle input
        let keys = [
            window.is_key_down(Key::Key1),
            window.is_key_down(Key::Key2),
            window.is_key_down(Key::Key3),
            window.is_key_down(Key::Key4),
            window.is_key_down(Key::Key5),
            window.is_key_down(Key::Key6),
            window.is_key_down(Key::Key0),
            window.is_key_down(Key::O),
            window.is_key_down(Key::S),
            window.is_key_down(Key::C),
        ];

        // Warp to planets (1-6)
        for i in 0..6 {
            if keys[i] && !prev_keys[i] {
                if let Some(body) = solar_system.get_body_by_index(i) {
                    let pos = body.get_position(solar_system.time);
                    let offset = Vec3::new(0.0, body.radius * 3.0, body.radius * 5.0);
                    camera.start_warp(pos + offset);
                    camera.target = pos;
                    current_body_index = i;
                }
            }
        }

        // Toggle orbits
        if keys[7] && !prev_keys[7] {
            show_orbits = !show_orbits;
        }

        // Toggle ship
        if keys[8] && !prev_keys[8] {
            show_ship = !show_ship;
        }

        // Toggle camera mode
        if keys[9] && !prev_keys[9] {
            camera_mode = (camera_mode + 1) % 3;
        }

        prev_keys = keys;

        // Camera movement (3D)
        let move_speed = 5.0;
        if window.is_key_down(Key::W) {
            camera.move_forward(move_speed);
        }
        if window.is_key_down(Key::S) && !keys[8] { // Don't conflict with ship toggle
            camera.move_forward(-move_speed);
        }
        if window.is_key_down(Key::A) {
            camera.move_right(-move_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(move_speed);
        }
        if window.is_key_down(Key::Q) {
            camera.move_up(move_speed);
        }
        if window.is_key_down(Key::E) {
            camera.move_up(-move_speed);
        }
        if window.is_key_down(Key::Left) {
            camera.rotate_yaw(-0.05);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate_yaw(0.05);
        }
        if window.is_key_down(Key::Up) {
            camera.rotate_pitch(0.05);
        }
        if window.is_key_down(Key::Down) {
            camera.rotate_pitch(-0.05);
        }

        // Update systems
        solar_system.update(0.01);
        camera.update(0.01);

        // Update camera based on mode
        if camera_mode == 1 && !camera.is_warping {
            // Follow mode
            if let Some(body) = solar_system.get_body_by_index(current_body_index) {
                let pos = body.get_position(solar_system.time);
                let offset = Vec3::new(0.0, body.radius * 3.0, body.radius * 5.0);
                camera.follow_body(pos, offset);
            }
        } else if camera_mode == 2 && !camera.is_warping {
            // Orbit mode
            if let Some(body) = solar_system.get_body_by_index(current_body_index) {
                let pos = body.get_position(solar_system.time);
                let angle = time * 0.3;
                let distance = body.radius * 8.0;
                let offset = Vec3::new(
                    distance * angle.cos(),
                    body.radius * 2.0,
                    distance * angle.sin(),
                );
                camera.follow_body(pos, offset);
            }
        }

        // Check collisions
        for body in &solar_system.bodies {
            let pos = body.get_position(solar_system.time);
            if camera.check_collision(pos, body.radius) {
                // Push camera away
                let direction = (camera.position - pos).normalize();
                camera.position = pos + direction * (body.radius + 15.0);
            }
        }

        framebuffer.clear();

        // Get view and projection matrices
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix();
        
        // Debug: Print first frame info
        if time < 0.02 {
            println!("First frame - rendering skybox with {} vertices", skybox_vertices.len());
            println!("View matrix: {:?}", view_matrix);
            println!("Projection matrix: {:?}", projection_matrix);
        }

        // Render skybox first (rotates with camera but doesn't translate)
        // Create view matrix without translation for skybox
        let skybox_view = {
            let _eye = Vec3::new(0.0, 0.0, 0.0); // No translation
            let center = camera.target - camera.position; // Direction only
            let up = camera.up;
            let f = center.normalize();
            let s = f.cross(&up).normalize();
            let u = s.cross(&f);
            Mat4::new(
                s.x, u.x, -f.x, 0.0,
                s.y, u.y, -f.y, 0.0,
                s.z, u.z, -f.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            )
        };
        let skybox_uniforms = Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix: skybox_view,
            projection_matrix,
            screen_width: framebuffer_width as f32,
            screen_height: framebuffer_height as f32,
        };
        if time < 0.02 {
            println!("About to render skybox...");
        }
        render(&mut framebuffer, &skybox_uniforms, &skybox_vertices, skybox_shader);
        if time < 0.02 {
            println!("Skybox rendered, continuing...");
        }

        // Render orbit paths
        if show_orbits {
            for (i, orbit_path) in orbit_paths.iter().enumerate() {
                if let Some(body) = solar_system.get_body_by_index(i) {
                    if body.orbit_radius > 0.0 {
                        // Render orbit as line segments
                        for j in 0..orbit_path.len() - 1 {
                            let v1 = &orbit_path[j];
                            let v2 = &orbit_path[j + 1];
                            
                            let orbit_uniforms = Uniforms {
                                model_matrix: Mat4::identity(),
                                view_matrix,
                                projection_matrix,
                                screen_width: framebuffer_width as f32,
                                screen_height: framebuffer_height as f32,
                            };
                            
                            // Transform vertices
                            let transformed_v1 = vertex_shader(v1, &orbit_uniforms);
                            let transformed_v2 = vertex_shader(v2, &orbit_uniforms);
                            
                            // Draw line using line rendering with orbit color
                            let orbit_color = orbit_shader(v1, v2, v1, v1.position, v1.normal, v1.tex_coords);
                            let line_fragments = line::line(&transformed_v1, &transformed_v2);
                            for fragment in line_fragments {
                                let x = fragment.position.x as usize;
                                let y = fragment.position.y as usize;
                                if x < framebuffer.width && y < framebuffer.height {
                                    let color = orbit_color.to_hex();
                                    framebuffer.set_current_color(color);
                                    framebuffer.point(x, y, fragment.depth);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Render all celestial bodies
        for (_body_index, body) in solar_system.bodies.iter().enumerate() {
            let body_pos = body.get_position(solar_system.time);
            let body_rotation = body.get_rotation(solar_system.time);

            // Get the appropriate sphere model
            let sphere_model = match body.body_type {
                solar_system::CelestialBodyType::Star => &star_sphere,
                solar_system::CelestialBodyType::RockyPlanet => &planet_sphere,
                solar_system::CelestialBodyType::GasGiant => &gas_giant_sphere,
                _ => &planet_sphere,
            };

            // Render main body
            let model_matrix = create_model_matrix(body_pos, body.radius, body_rotation);
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                screen_width: framebuffer_width as f32,
                screen_height: framebuffer_height as f32,
            };
            render(&mut framebuffer, &uniforms, sphere_model, body.shader);

            // Render rings if present
            if body.has_rings {
                let ring_model = generate_ring(body.ring_inner / body.radius, body.ring_outer / body.radius, 60);
                let ring_rotation = Vec3::new(0.0, 0.0, solar_system.time * 0.1);
                let ring_matrix = create_model_matrix(body_pos, body.radius, ring_rotation);
                let ring_uniforms = Uniforms {
                    model_matrix: ring_matrix,
                    view_matrix,
                    projection_matrix,
                    screen_width: framebuffer_width as f32,
                    screen_height: framebuffer_height as f32,
                };
                render(&mut framebuffer, &ring_uniforms, &ring_model, fragment_shaders::ring_shader);
            }

            // Render moons
            for moon in &body.moons {
                let moon_angle = moon.initial_angle + solar_system.time * moon.orbit_speed;
                let moon_offset = Vec3::new(
                    moon.orbit_radius * moon_angle.cos(),
                    0.0,
                    moon.orbit_radius * moon_angle.sin(),
                );
                let moon_pos = body_pos + moon_offset;
                let moon_rotation = Vec3::new(
                    solar_system.time * 0.4,
                    solar_system.time * 0.4,
                    0.0,
                );
                let moon_matrix = create_model_matrix(moon_pos, moon.radius, moon_rotation);
                let moon_uniforms = Uniforms {
                    model_matrix: moon_matrix,
                    view_matrix,
                    projection_matrix,
                    screen_width: framebuffer_width as f32,
                    screen_height: framebuffer_height as f32,
                };
                render(&mut framebuffer, &moon_uniforms, &moon_sphere, fragment_shaders::moon_shader);
            }
        }

        // Render ship if enabled
        if show_ship {
            let ship_offset = Vec3::new(0.0, -20.0, -30.0); // Behind and below camera
            let ship_pos = camera.position + ship_offset;
            let ship_rotation = Vec3::new(0.0, camera.yaw + std::f32::consts::PI, 0.0);
            let ship_matrix = create_model_matrix(ship_pos, 5.0, ship_rotation);
            let ship_uniforms = Uniforms {
                model_matrix: ship_matrix,
                view_matrix,
                projection_matrix,
                screen_width: framebuffer_width as f32,
                screen_height: framebuffer_height as f32,
            };
            // Ship shader with metallic look
            // Use a simple shader function that doesn't capture variables
            let ship_shader = ship_shader_metallic;
            render(&mut framebuffer, &ship_uniforms, &ship_model, ship_shader);
        }

        // Debug: Check if framebuffer has any non-black pixels
        if time < 0.02 {
            let non_black_pixels: usize = framebuffer.buffer.iter()
                .filter(|&&pixel| pixel != 0x000000 && pixel != 0x000011)
                .count();
            println!("First frame - Non-black pixels in framebuffer: {}", non_black_pixels);
            println!("About to update window...");
        }
        
        // Update window - this is critical for minifb
        window.update();
        
        // Update window with framebuffer
        if let Err(e) = window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height) {
            eprintln!("Error updating window: {:?}", e);
        }
        
        if time < 0.02 {
            println!("Window updated, frame complete");
        }
        
        // Limit update rate
        std::thread::sleep(frame_delay);
        time += 0.01;
    }
}
