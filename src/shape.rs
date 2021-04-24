pub mod shape2d {
    use crate::canvas;
    use std::mem::swap;

    pub fn line(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
        if x1 == x2 {
            let mut y_start = y1;
            let mut y_end = y2;
            if y2 < y1 {
                y_start = y2;
                y_end = y1;
            }
            while y_start <= y_end {
                canvas.set_pixel_at(x1 as usize, y_start as usize, color);
                y_start += 1.0;
            }
        } else if y1 == y2 {
            let mut x_start = x1;
            let mut x_end = x2;
            if x2 < x1 {
                x_start = x2;
                x_end = x1;
            }
            while x_start <= x_end {
                canvas.set_pixel_at(x_start as usize, y1 as usize, color);
                x_start += 1.0;
            }
        } else {
            // All octant Bresenham's line algorithm using integer incremental errors
            let mut x1 = x1;
            let mut y1 = y1;

            let dx = (x2 - x1).abs();
            let mut sx = 1.0;
            if x2 < x1 {
                sx = -1.0;
            }
            let dy = ((y2 - y1).abs()) * -1.0;
            let mut sy = 1.0;
            if y2 < y1 {
                sy = -1.0;
            }
            let mut err = dx + dy;

            loop {
                canvas.set_pixel_at(x1 as usize, y1 as usize, color);
                if x1 == x2 && y1 == y2 {
                    break;
                }
                let e2 = 2.0 * err;
                if e2 >= dy {
                    err += dy;
                    x1 += sx;
                }
                if e2 <= dx {
                    err += dx;
                    y1 += sy;
                }
            }
        }
    }

    pub fn line_aa(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
        if x1 == x2 || y1 == y2 {
            line(canvas, x1, y1, x2, y2, color);
        } else {
            let mut p1 = (x1, y1);
            let mut p2 = (x2, y2);
            let steep = (y2 - y1).abs() > (x2 - x1).abs();
            if steep {
                swap(&mut p1.0, &mut p1.1);
                swap(&mut p2.0, &mut p2.1);
            }
            if y1 > y2 {
                swap(&mut p1, &mut p2);
            }
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let gradient = dy / dx;

            let xpxl1 = x1;
            let xpxl2 = x2;
            let mut intersect = y1;

            if !steep {
                let mut x_start = xpxl1;
                while x_start <= xpxl2 {
                    canvas.set_pixel_at(x_start as usize, f32::floor(intersect) as usize, color);
                    canvas.set_pixel_at(x_start as usize, f32::floor(intersect) as usize - 1, color);
                    intersect += gradient;
                    x_start += 1.0;
                }
            } else {
                let mut x_start = xpxl1;
                while x_start <= xpxl2 {
                    canvas.set_pixel_at(f32::floor(intersect) as usize, x_start as usize, color);
                    canvas.set_pixel_at(f32::floor(intersect) as usize - 1, x_start as usize, color);
                    intersect += gradient;
                    x_start += 1.0;
                }
            }
        }
    }

    pub fn rectangle(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
        line(canvas, x1, y1, x1, y2, color);
        line(canvas, x1, y2, x2, y2, color);
        line(canvas, x2, y2, x2, y1, color);
        line(canvas, x2, y1, x1, y1, color);
    }
}
