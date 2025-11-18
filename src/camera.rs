use nalgebra_glm::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub follow_target: Option<usize>, // Index of body to follow
    pub warp_target: Option<Vec3>,
    pub warp_progress: f32,
    pub is_warping: bool,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let position = Vec3::new(0.0, 200.0, 500.0);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let direction = (target - position).normalize();
        
        // Calculate yaw and pitch from direction
        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();
        let distance = (target - position).magnitude();
        
        Camera {
            position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 60.0,
            near: 0.1,
            far: 10000.0,
            aspect: width / height,
            yaw,
            pitch,
            distance,
            follow_target: None,
            warp_target: None,
            warp_progress: 0.0,
            is_warping: false,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let eye = self.position;
        let center = self.target;
        let up = self.up;

        let f = (center - eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        Mat4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -s.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0,
        )
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        let f = 1.0 / (self.fov.to_radians() / 2.0).tan();
        let range = self.far - self.near;

        Mat4::new(
            f / self.aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, -(self.far + self.near) / range, -1.0,
            0.0, 0.0, -(2.0 * self.far * self.near) / range, 0.0,
        )
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_warping {
            self.warp_progress += delta_time * 2.0; // Warp speed
            if let Some(target) = self.warp_target {
                if self.warp_progress >= 1.0 {
                    self.position = target;
                    self.is_warping = false;
                    self.warp_progress = 0.0;
                    self.warp_target = None;
                } else {
                    // Smooth interpolation with easing
                    let t = self.ease_in_out_cubic(self.warp_progress);
                    self.position = self.position * (1.0 - t) + target * t;
                }
            }
        }
    }

    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powf(3.0) / 2.0
        }
    }

    pub fn start_warp(&mut self, target: Vec3) {
        self.warp_target = Some(target);
        self.warp_progress = 0.0;
        self.is_warping = true;
    }

    pub fn follow_body(&mut self, body_position: Vec3, offset: Vec3) {
        self.target = body_position;
        self.position = body_position + offset;
    }

    pub fn move_forward(&mut self, distance: f32) {
        let direction = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        self.position = self.position + direction * distance;
        self.update_direction();
    }

    pub fn move_right(&mut self, distance: f32) {
        let forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        let right = forward.cross(&self.up).normalize();
        self.position = self.position + right * distance;
        self.update_direction();
    }

    pub fn move_up(&mut self, distance: f32) {
        self.position = self.position + self.up * distance;
        self.update_direction();
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        self.yaw += angle;
        self.update_direction();
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        self.pitch = (self.pitch + angle).clamp(-1.57, 1.57); // Clamp to -90 to 90 degrees
        self.update_direction();
    }

    fn update_direction(&mut self) {
        let direction = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        self.target = self.position + direction * self.distance;
    }

    pub fn check_collision(&self, center: Vec3, radius: f32) -> bool {
        let distance = (self.position - center).magnitude();
        distance < radius + 10.0 // 10.0 is safety margin
    }
}

