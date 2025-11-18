use crate::fragment_shaders::FragmentShader;
use crate::fragment_shaders::{
    gas_giant_shader, moon_shader, ring_shader, rocky_planet_shader, star_shader,
};
use nalgebra_glm::Vec3;

#[derive(Clone, Copy)]
pub enum CelestialBodyType {
    Star,
    RockyPlanet,
    GasGiant,
    Moon,
    Ring,
}

pub struct CelestialBody {
    pub name: String,
    pub body_type: CelestialBodyType,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub initial_angle: f32,
    pub shader: FragmentShader,
    pub has_rings: bool,
    pub ring_inner: f32,
    pub ring_outer: f32,
    pub moons: Vec<Moon>,
}

pub struct Moon {
    pub name: String,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub initial_angle: f32,
}

impl CelestialBody {
    pub fn new(
        name: String,
        body_type: CelestialBodyType,
        radius: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        initial_angle: f32,
    ) -> Self {
        let shader = match body_type {
            CelestialBodyType::Star => star_shader,
            CelestialBodyType::RockyPlanet => rocky_planet_shader,
            CelestialBodyType::GasGiant => gas_giant_shader,
            CelestialBodyType::Moon => moon_shader,
            CelestialBodyType::Ring => ring_shader,
        };

        CelestialBody {
            name,
            body_type,
            radius,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            initial_angle,
            shader,
            has_rings: false,
            ring_inner: 0.0,
            ring_outer: 0.0,
            moons: Vec::new(),
        }
    }

    pub fn with_rings(mut self, inner: f32, outer: f32) -> Self {
        self.has_rings = true;
        self.ring_inner = inner;
        self.ring_outer = outer;
        self
    }

    pub fn add_moon(mut self, moon: Moon) -> Self {
        self.moons.push(moon);
        self
    }

    pub fn get_position(&self, time: f32) -> Vec3 {
        let angle = self.initial_angle + time * self.orbit_speed;
        Vec3::new(
            self.orbit_radius * angle.cos(),
            0.0, // All planets on ecliptic plane (y = 0)
            self.orbit_radius * angle.sin(),
        )
    }

    pub fn get_rotation(&self, time: f32) -> Vec3 {
        Vec3::new(
            time * self.rotation_speed,
            time * self.rotation_speed * 0.7,
            0.0,
        )
    }
}

pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub time: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        let mut system = SolarSystem {
            bodies: Vec::new(),
            time: 0.0,
        };

        // Create a fictional solar system
        // Star (Sun)
        let star = CelestialBody::new(
            "Sol".to_string(),
            CelestialBodyType::Star,
            50.0,
            0.0,
            0.0,
            0.5,
            0.0,
        );
        system.bodies.push(star);

        // Planet 1: Rocky planet close to sun
        let planet1 = CelestialBody::new(
            "Mercurio".to_string(),
            CelestialBodyType::RockyPlanet,
            15.0,
            200.0,
            0.8,
            0.6,
            0.0,
        )
        .add_moon(Moon {
            name: "Luna 1".to_string(),
            radius: 5.0,
            orbit_radius: 40.0,
            orbit_speed: 1.2,
            initial_angle: 0.0,
        });
        system.bodies.push(planet1);

        // Planet 2: Rocky planet
        let planet2 = CelestialBody::new(
            "Terra".to_string(),
            CelestialBodyType::RockyPlanet,
            18.0,
            350.0,
            0.5,
            0.4,
            1.5,
        )
        .add_moon(Moon {
            name: "Luna".to_string(),
            radius: 6.0,
            orbit_radius: 50.0,
            orbit_speed: 0.9,
            initial_angle: 0.0,
        });
        system.bodies.push(planet2);

        // Planet 3: Gas giant
        let planet3 = CelestialBody::new(
            "Jupiter".to_string(),
            CelestialBodyType::GasGiant,
            35.0,
            550.0,
            0.3,
            0.25,
            3.0,
        )
        .with_rings(40.0, 60.0)
        .add_moon(Moon {
            name: "Io".to_string(),
            radius: 8.0,
            orbit_radius: 70.0,
            orbit_speed: 1.5,
            initial_angle: 0.0,
        })
        .add_moon(Moon {
            name: "Europa".to_string(),
            radius: 7.0,
            orbit_radius: 90.0,
            orbit_speed: 1.2,
            initial_angle: 2.0,
        });
        system.bodies.push(planet3);

        // Planet 4: Another rocky planet
        let planet4 = CelestialBody::new(
            "Marte".to_string(),
            CelestialBodyType::RockyPlanet,
            12.0,
            750.0,
            0.25,
            0.35,
            4.5,
        );
        system.bodies.push(planet4);

        // Planet 5: Outer gas giant
        let planet5 = CelestialBody::new(
            "Saturno".to_string(),
            CelestialBodyType::GasGiant,
            30.0,
            950.0,
            0.2,
            0.2,
            6.0,
        )
        .with_rings(35.0, 55.0);
        system.bodies.push(planet5);

        system
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    pub fn get_body_by_index(&self, index: usize) -> Option<&CelestialBody> {
        self.bodies.get(index)
    }

    pub fn get_body_count(&self) -> usize {
        self.bodies.len()
    }
}
