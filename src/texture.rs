use std::path::Path;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>, // RGBA as u32
}

impl Texture {
    pub fn load(filename: &str) -> Result<Self, String> {
        let img = image::open(Path::new(filename))
            .map_err(|e| format!("Failed to open image {}: {}", filename, e))?;

        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        let mut data = Vec::with_capacity((width * height) as usize);

        for pixel in rgb_img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            // Pack as RGB (24-bit, with 0 in alpha)
            let color = (r << 16) | (g << 8) | b;
            data.push(color);
        }

        Ok(Texture {
            width: width as usize,
            height: height as usize,
            data,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> u32 {
        // Clamp coordinates to [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Convert to pixel coordinates
        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);

        // Sample the texture (simple nearest neighbor for now)
        let index = y * self.width + x;
        self.data[index]
    }

    pub fn sample_bilinear(&self, u: f32, v: f32) -> u32 {
        // Clamp coordinates to [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Convert to pixel coordinates with fractional part
        let fx = u * self.width as f32;
        let fy = v * self.height as f32;

        let x0 = (fx as usize).min(self.width - 1);
        let y0 = (fy as usize).min(self.height - 1);
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let frac_x = fx - x0 as f32;
        let frac_y = fy - y0 as f32;

        // Sample four corners
        let c00 = self.data[y0 * self.width + x0];
        let c10 = self.data[y0 * self.width + x1];
        let c01 = self.data[y1 * self.width + x0];
        let c11 = self.data[y1 * self.width + x1];

        // Extract RGB components
        let extract = |color: u32| -> (f32, f32, f32) {
            let r = ((color >> 16) & 0xFF) as f32 / 255.0;
            let g = ((color >> 8) & 0xFF) as f32 / 255.0;
            let b = (color & 0xFF) as f32 / 255.0;
            (r, g, b)
        };

        let (r00, g00, b00) = extract(c00);
        let (r10, g10, b10) = extract(c10);
        let (r01, g01, b01) = extract(c01);
        let (r11, g11, b11) = extract(c11);

        // Bilinear interpolation
        let r0 = r00 * (1.0 - frac_x) + r10 * frac_x;
        let g0 = g00 * (1.0 - frac_x) + g10 * frac_x;
        let b0 = b00 * (1.0 - frac_x) + b10 * frac_x;

        let r1 = r01 * (1.0 - frac_x) + r11 * frac_x;
        let g1 = g01 * (1.0 - frac_x) + g11 * frac_x;
        let b1 = b01 * (1.0 - frac_x) + b11 * frac_x;

        let r = r0 * (1.0 - frac_y) + r1 * frac_y;
        let g = g0 * (1.0 - frac_y) + g1 * frac_y;
        let b = b0 * (1.0 - frac_y) + b1 * frac_y;

        // Pack back to u32
        let r_u8 = (r * 255.0) as u32;
        let g_u8 = (g * 255.0) as u32;
        let b_u8 = (b * 255.0) as u32;

        (r_u8 << 16) | (g_u8 << 8) | b_u8
    }
}
