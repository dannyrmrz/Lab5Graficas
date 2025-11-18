use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Mat4, Vec3};
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

mod camera;
mod color;
mod fragment;
mod fragment_shaders;
mod framebuffer;
mod line;
mod obj;
mod orbit;
mod shaders;
mod ship;
mod skybox;
mod solar_system;
mod sphere;
mod texture;
mod triangle;
mod vertex;

use camera::Camera;
use framebuffer::Framebuffer;
use line::line;
use orbit::{generate_orbit_path, orbit_shader};
use shaders::vertex_shader;
use ship::load_ship;
use skybox::{generate_skybox_vertices, skybox_shader_procedural, skybox_shader_textured, Skybox};
use solar_system::SolarSystem;
use sphere::{generate_ring, generate_sphere};
use triangle::triangle_with_shader;
use vertex::Vertex;
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
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    transform_matrix * rotation_matrix
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    fragment_shader: fragment_shaders::FragmentShader,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    let mut debug_count = 0;
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);

        // Debug: Check first few transformed vertices
        if debug_count < 3 {
            let pos = transformed.transformed_position;
            if pos.x.is_nan() || pos.y.is_nan() || pos.z.is_nan() {
                eprintln!(
                    "Warning: NaN in transformed position at index {}",
                    debug_count
                );
            }
            if pos.x < -1000.0 || pos.x > 3000.0 || pos.y < -1000.0 || pos.y > 3000.0 {
                eprintln!(
                    "Warning: Transformed position out of expected range: {:?}",
                    pos
                );
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

    // Rasterization + Fragment Processing Stage
    for tri in &triangles {
        let triangle_fragments = triangle_with_shader(&tri[0], &tri[1], &tri[2], fragment_shader);
        for fragment in triangle_fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;
            if x < framebuffer.width && y < framebuffer.height {
                let color = fragment.color.to_hex();
                framebuffer.set_current_color(color);
                framebuffer.point(x, y, fragment.depth);
            }
        }
    }
}

fn main() {
    #[cfg(target_family = "unix")]
    {
        // Force X11 backend when Wayland server capabilities are insufficient (WSL case)
        if std::env::var("WINIT_UNIX_BACKEND").is_err() {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    println!("Initializing application...");
    let window_width = 900;
    let window_height = 600;
    let framebuffer_width = 900;
    let framebuffer_height = 600;
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
    let sphere_segments = 20;
    let star_sphere = generate_sphere(1.0, sphere_segments);
    let planet_sphere = generate_sphere(1.0, sphere_segments);
    let gas_giant_sphere = generate_sphere(1.0, sphere_segments);
    let moon_sphere = generate_sphere(1.0, 16);
    let _ring = generate_ring(1.2, 2.0, 36);
    println!("Geometry generated");

    println!("Loading ship mesh...");
    let ship_mesh = match load_ship() {
        Ok(mesh) => {
            println!("Ship mesh ready with {} vertices", mesh.len());
            mesh
        }
        Err(err) => {
            eprintln!("Warning: ship mesh fallback failed: {}", err);
            Vec::new()
        }
    };
    println!("Generating skybox vertices...");
    let skybox_vertices = generate_skybox_vertices();
    println!(
        "Skybox vertices generated: {} vertices",
        skybox_vertices.len()
    );

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
            orbit_paths.push(generate_orbit_path(body.orbit_radius, 48));
        }
    }
    println!("Orbit paths generated: {} paths", orbit_paths.len());

    let mut time = 0.0f32;
    let mut prev_warp_keys = [false; 6];
    let mut prev_orbit_toggle = false;
    let mut prev_camera_toggle = false;
    let mut prev_skybox_toggle = false;
    let mut prev_ship_toggle = false;
    let mut current_body_index = 0;
    let mut show_orbits = true;
    let mut show_skybox = true;
    let mut show_ship = true;
    let mut camera_mode = 0; // 0 = free, 1 = follow, 2 = orbit

    println!(
        "Starting render loop. Camera position: {:?}, target: {:?}",
        camera.position, camera.target
    );
    println!("Controles: ESC sale, WASD/QE mueven la cámara, flechas rotan, teclas 1-6 hacen warp, O alterna órbitas, C cambia modo de cámara, B alterna skybox, N muestra/oculta la nave");

    while window.is_open() {
        // Poll OS events before reading keyboard state so inputs respond immediately
        window.update();

        if window.is_key_down(Key::Escape) {
            break;
        }

        // Handle input
        let warp_keys = [
            window.is_key_down(Key::Key1),
            window.is_key_down(Key::Key2),
            window.is_key_down(Key::Key3),
            window.is_key_down(Key::Key4),
            window.is_key_down(Key::Key5),
            window.is_key_down(Key::Key6),
        ];

        for (i, &pressed) in warp_keys.iter().enumerate() {
            if pressed && !prev_warp_keys[i] {
                if let Some(body) = solar_system.get_body_by_index(i) {
                    let pos = body.get_position(solar_system.time);
                    let offset = Vec3::new(0.0, body.radius * 3.0, body.radius * 5.0);
                    camera.start_warp(pos + offset);
                    camera.target = pos;
                    current_body_index = i;
                }
            }
        }
        prev_warp_keys.copy_from_slice(&warp_keys);

        let orbit_toggle = window.is_key_down(Key::O);
        if orbit_toggle && !prev_orbit_toggle {
            show_orbits = !show_orbits;
        }
        prev_orbit_toggle = orbit_toggle;

        let camera_toggle = window.is_key_down(Key::C);
        if camera_toggle && !prev_camera_toggle {
            camera_mode = (camera_mode + 1) % 3;
        }
        prev_camera_toggle = camera_toggle;

        let skybox_toggle = window.is_key_down(Key::B);
        if skybox_toggle && !prev_skybox_toggle {
            show_skybox = !show_skybox;
        }
        prev_skybox_toggle = skybox_toggle;

        let ship_toggle = window.is_key_down(Key::N);
        if ship_toggle && !prev_ship_toggle {
            show_ship = !show_ship;
        }
        prev_ship_toggle = ship_toggle;

        // Camera movement (3D)
        let move_speed = 5.0;
        if window.is_key_down(Key::W) {
            camera.move_forward(move_speed);
        }
        if window.is_key_down(Key::S) {
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

        // Position ship relative to camera direction and keep it outside planets
        let forward_dir = camera.forward_direction().normalize();
        let up_dir = camera.up.normalize();
        let right_dir = forward_dir.cross(&up_dir).normalize();
        let mut ship_position = camera.position
            + forward_dir * 140.0  // push ship ahead of the camera so we are no longer inside it
            - up_dir * 25.0        // drop it slightly to keep cockpit below the view axis
            + right_dir * 12.0;    // small lateral offset so it does not cover the reticle
        for body in &solar_system.bodies {
            let pos = body.get_position(solar_system.time);
            let to_ship = ship_position - pos;
            let distance = to_ship.magnitude();
            let min_distance = body.radius + 8.0;
            if distance > 0.0 && distance < min_distance {
                ship_position = pos + to_ship.normalize() * min_distance;
            }
        }
        let ship_rotation = Vec3::new(-camera.pitch, camera.yaw - FRAC_PI_2, 0.0);

        framebuffer.clear();

        // Get view and projection matrices
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix();

        // Debug: Print first frame info
        if time < 0.02 {
            println!(
                "First frame - rendering skybox with {} vertices",
                skybox_vertices.len()
            );
            println!("View matrix: {:?}", view_matrix);
            println!("Projection matrix: {:?}", projection_matrix);
        }

        if show_skybox {
            // Render skybox first (rotates with camera but doesn't translate)
            // Create view matrix without translation for skybox
            let skybox_view = {
                let center = camera.target - camera.position; // Direction only
                let up = camera.up;
                let f = center.normalize();
                let s = f.cross(&up).normalize();
                let u = s.cross(&f);
                Mat4::new(
                    s.x, u.x, -f.x, 0.0, s.y, u.y, -f.y, 0.0, s.z, u.z, -f.z, 0.0, 0.0, 0.0, 0.0,
                    1.0,
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
            render(
                &mut framebuffer,
                &skybox_uniforms,
                &skybox_vertices,
                skybox_shader,
            );
            if time < 0.02 {
                println!("Skybox rendered, continuing...");
            }
        } else {
            framebuffer.clear_to_color(0x000011);
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
                            let orbit_color = orbit_shader(
                                &transformed_v1,
                                &transformed_v2,
                                &transformed_v1,
                                v1.position,
                                transformed_v1.world_position,
                                v1.normal,
                                v1.tex_coords,
                            );
                            let line_fragments = line(&transformed_v1, &transformed_v2);
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
                let ring_model = generate_ring(
                    body.ring_inner / body.radius,
                    body.ring_outer / body.radius,
                    36,
                );
                let ring_rotation = Vec3::new(0.0, 0.0, solar_system.time * 0.1);
                let ring_matrix = create_model_matrix(body_pos, body.radius, ring_rotation);
                let ring_uniforms = Uniforms {
                    model_matrix: ring_matrix,
                    view_matrix,
                    projection_matrix,
                    screen_width: framebuffer_width as f32,
                    screen_height: framebuffer_height as f32,
                };
                render(
                    &mut framebuffer,
                    &ring_uniforms,
                    &ring_model,
                    fragment_shaders::ring_shader,
                );
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
                let moon_rotation =
                    Vec3::new(solar_system.time * 0.4, solar_system.time * 0.4, 0.0);
                let moon_matrix = create_model_matrix(moon_pos, moon.radius, moon_rotation);
                let moon_uniforms = Uniforms {
                    model_matrix: moon_matrix,
                    view_matrix,
                    projection_matrix,
                    screen_width: framebuffer_width as f32,
                    screen_height: framebuffer_height as f32,
                };
                render(
                    &mut framebuffer,
                    &moon_uniforms,
                    &moon_sphere,
                    fragment_shaders::moon_shader,
                );
            }
        }

        if show_ship && !ship_mesh.is_empty() {
            let ship_matrix = create_model_matrix(ship_position, 25.0, ship_rotation);
            let ship_uniforms = Uniforms {
                model_matrix: ship_matrix,
                view_matrix,
                projection_matrix,
                screen_width: framebuffer_width as f32,
                screen_height: framebuffer_height as f32,
            };
            render(
                &mut framebuffer,
                &ship_uniforms,
                &ship_mesh,
                fragment_shaders::ship_shader_metallic,
            );
        }

        // Debug: Check if framebuffer has any non-black pixels
        if time < 0.02 {
            let non_black_pixels: usize = framebuffer
                .buffer
                .iter()
                .filter(|&&pixel| pixel != 0x000000 && pixel != 0x000011)
                .count();
            println!(
                "First frame - Non-black pixels in framebuffer: {}",
                non_black_pixels
            );
            println!("About to update window...");
        }

        // Update window with framebuffer
        if let Err(e) =
            window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        {
            eprintln!("Error updating window: {:?}", e);
            #[cfg(target_family = "unix")]
            {
                if e.to_string().contains("Wayland") {
                    eprintln!("Hint: ejecuta el programa en Windows nativo o inicia un servidor X11 (por ejemplo VcXsrv) y exporta DISPLAY junto con WINIT_UNIX_BACKEND=x11 antes de lanzar `cargo run` en WSL.");
                }
            }
            eprintln!("Stopping render loop due to window error.");
            break;
        }

        if time < 0.02 {
            println!("Window updated, frame complete");
        }

        // Limit update rate
        std::thread::sleep(frame_delay);
        time += 0.01;
    }
}
