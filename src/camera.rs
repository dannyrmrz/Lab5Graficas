use nalgebra_glm::{Mat4, Vec3};

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
    pub warp_start: Option<Vec3>,
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
            warp_start: None,
            warp_progress: 0.0,
            is_warping: false,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        nalgebra_glm::look_at(&self.position, &self.target, &self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        nalgebra_glm::perspective(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_warping {
            self.warp_progress = (self.warp_progress + delta_time * 2.0).min(1.0);
            if let (Some(start), Some(target)) = (self.warp_start.clone(), self.warp_target.clone())
            {
                if self.warp_progress >= 1.0 {
                    self.position = target;
                    self.is_warping = false;
                    self.warp_progress = 0.0;
                    self.warp_start = None;
                    self.warp_target = None;
                    self.sync_from_target();
                } else {
                    // Smooth interpolation with easing
                    let t = self.ease_in_out_cubic(self.warp_progress);
                    self.position = start * (1.0 - t) + target * t;
                }
            } else {
                self.is_warping = false;
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
        self.warp_start = Some(self.position);
        self.warp_progress = 0.0;
        self.is_warping = true;
    }

    pub fn follow_body(&mut self, body_position: Vec3, offset: Vec3) {
        self.target = body_position;
        self.position = body_position + offset;
        self.sync_from_target();
    }

    pub fn move_forward(&mut self, distance: f32) {
        let direction = self.forward_direction();
        self.position = self.position + direction * distance;
        self.update_direction();
    }

    pub fn move_right(&mut self, distance: f32) {
        let forward = self.forward_direction();
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

    fn sync_from_target(&mut self) {
        let direction = self.target - self.position;
        let distance = direction.magnitude();
        if distance > 0.0 {
            let dir_norm = direction / distance;
            self.distance = distance;
            self.yaw = dir_norm.z.atan2(dir_norm.x);
            self.pitch = dir_norm.y.asin();
        }
    }

    pub fn forward_direction(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
    }

    pub fn check_collision(&self, center: Vec3, radius: f32) -> bool {
        let distance = (self.position - center).magnitude();
        distance < radius + 10.0 // 10.0 is safety margin
    }
}
