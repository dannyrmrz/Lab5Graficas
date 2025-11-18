use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    // Clamp coordinates to reasonable range to avoid overflow
    let start_x = start.x.clamp(-10000.0, 20000.0);
    let start_y = start.y.clamp(-10000.0, 20000.0);
    let end_x = end.x.clamp(-10000.0, 20000.0);
    let end_y = end.y.clamp(-10000.0, 20000.0);
    
    let mut x0 = start_x as i32;
    let mut y0 = start_y as i32;
    let x1 = end_x as i32;
    let y1 = end_y as i32;

    let dx = (x1.saturating_sub(x0)).abs();
    let dy = (y1.saturating_sub(y0)).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        // Avoid division by zero
        let denom = if (end_x - start_x).abs() > 0.0001 {
            (end_x - start_x)
        } else {
            1.0
        };
        let z = start.z + (end.z - start.z) * (x0 as f32 - start_x) / denom;
        fragments.push(Fragment::new(x0 as f32, y0 as f32, Color::new(255, 255, 255), z));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}
