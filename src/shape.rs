pub mod shape2d {
    use crate::canvas;
    // use std::mem::swap;

    pub fn line(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
        if x1 == x2 {
            let x1 = x1 as isize;
            let mut y_start = y1 as isize;
            let mut y_end = y2 as isize;

            if y2 < y1 {
                y_start = y2 as isize;
                y_end = y1 as isize;
            }
            while y_start <= y_end {
                canvas.set_pixel_at(x1 as usize, y_start as usize, color);
                y_start += 1;
            }
        } else if y1 == y2 {
            let x1 = x1 as isize;
            let x2 = x2 as isize;
            let y1 = y1 as isize;
            let mut x_start = x1 as isize;
            let mut x_end = x2 as isize;
            if x2 < x1 {
                x_start = x2;
                x_end = x1;
            }
            while x_start <= x_end {
                canvas.set_pixel_at(x_start as usize, y1 as usize, color);
                x_start += 1;
            }
        } else {
            // All octant Bresenham's line algorithm using integer incremental errors
            let mut x1 = x1 as isize;
            let mut y1 = y1 as isize;
            let x2 = x2 as isize;
            let y2 = y2 as isize;

            let dx = (x2 - x1).abs();
            let mut sx = 1;
            if x2 < x1 {
                sx = -1;
            }
            let dy = ((y2 - y1).abs()) * -1;
            let mut sy = 1;
            if y2 < y1 {
                sy = -1;
            }
            let mut err = dx + dy;

            loop {
                canvas.set_pixel_at(x1 as usize, y1 as usize, color);
                if x1 == x2 && y1 == y2 {
                    break;
                }
                let e2 = err << 1;
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

    // fn fpart(val: f32) -> f32 {
    //     if val < 0.0 {
    //         val - f32::ceil(val)
    //     } else {
    //         val - f32::floor(val)
    //     }
    // }

    // fn rfpart(val: f32) -> f32 {
    //     1.0 - fpart(val)
    // }

    // fn put_pixel(canvas: &mut canvas::Canvas, x: f32, y: f32, c: f32, color: (u8, u8, u8, u8)) {
    //     let mut alpha = (c * 255.0) as isize;
    //     alpha = isize::clamp(alpha, 0, 255);
    //     let r = (color.0 as f32 * c) as u8;
    //     let g = (color.1 as f32 * c) as u8;
    //     let b = (color.2 as f32 * c) as u8;

    //     canvas.set_pixel_at(x as usize, y as usize, (r, g, b, alpha as u8));
    // }

    // pub fn line_aa(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
    //     if x1 == x2 || y1 == y2 {
    //         line(canvas, x1, y1, x2, y2, color);
    //     } else {
    //         let mut p1 = (x1, y1);
    //         let mut p2 = (x2, y2);
    //         let steep = (y2 - y1).abs() > (x2 - x1).abs();
    //         if steep {
    //             swap(&mut p1.0, &mut p1.1);
    //             swap(&mut p2.0, &mut p2.1);
    //         }
    //         if x1 > x2 {
    //             swap(&mut p1, &mut p2);
    //         }
    //         let dx = p2.0 - p1.0;
    //         let dy = p2.1 - p1.1;
    //         let gradient = dy / dx;

    //         let mut x_end = f32::round(p1.0);
    //         let mut y_end = p1.1 + gradient * (x_end - p1.0);
    //         let mut x_gap = rfpart(p1.0 + 0.5);

    //         let x_pxl1 = x_end;
    //         let y_pxl1 = (y_end as isize) as f32;

    //         if steep {
    //             put_pixel(canvas, y_pxl1, x_pxl1, rfpart(y_end) * x_gap, color);
    //             put_pixel(canvas, y_pxl1 + 1.0, x_pxl1, fpart(y_end) * x_gap, color);
    //         } else {
    //             put_pixel(canvas, x_pxl1, y_pxl1, rfpart(y_end) * x_gap, color);
    //             put_pixel(canvas, x_pxl1, y_pxl1 + 1.0, fpart(y_end) * x_gap, color);
    //         }

    //         let mut inter_y = y_end * gradient;
    //         x_end = f32::round(p1.0);
    //         y_end = p1.1 + gradient * (x_end - p1.0);
    //         x_gap = fpart(p1.0 + 0.5);

    //         let x_pxl2 = x_end;
    //         let y_pxl2 = (y_end as isize) as f32;

    //         if steep {
    //             put_pixel(canvas, y_pxl2, x_pxl2, rfpart(y_end) * x_gap, color);
    //             put_pixel(canvas, y_pxl2 + 1.0, x_pxl2, fpart(y_end) * x_gap, color);
    //         } else {
    //             put_pixel(canvas, x_pxl2, y_pxl2, rfpart(y_end) * x_gap, color);
    //             put_pixel(canvas, x_pxl2, y_pxl2 + 1.0, fpart(y_end) * x_gap, color);
    //         }

    //         if steep {
    //             let mut x = (x_pxl1 + 1.0) as isize;
    //             let x_end = x_pxl2 as isize;
    //             while x < x_end {
    //                 put_pixel(canvas, (inter_y as isize) as f32, x as f32, rfpart(inter_y), color);
    //                 put_pixel(canvas, (inter_y as isize + 1) as f32, x as f32, fpart(inter_y), color);
    //                 inter_y += gradient;
    //                 x += 1;
    //             }
    //         } else {
    //             let mut x = (x_pxl1 + 1.0) as isize;
    //             let x_end = x_pxl2 as isize;
    //             while x < x_end {
    //                 put_pixel(canvas, x as f32, (inter_y as isize) as f32, rfpart(inter_y), color);
    //                 put_pixel(canvas, x as f32, (inter_y as isize + 1) as f32, fpart(inter_y), color);
    //                 inter_y += gradient;
    //                 x += 1;
    //             }
    //         }
    //     }
    // }

    pub fn line_from_segments(canvas: &mut canvas::Canvas, points: &Vec<f32>, color: (u8, u8, u8, u8)) {
        for i in (0..(points.len() - 2)).step_by(2) {
            let x1 = points[i];
            let y1 = points[i + 1];
            let x2 = points[i + 2];
            let y2 = points[i + 3];
            line(canvas, x1, y1, x2, y2, color);
        }
    }

    pub fn rectangle(canvas: &mut canvas::Canvas, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8, u8)) {
        line(canvas, x1, y1, x1, y2, color);
        line(canvas, x1, y2, x2, y2, color);
        line(canvas, x2, y2, x2, y1, color);
        line(canvas, x2, y1, x1, y1, color);
    }

    pub fn square(canvas: &mut canvas::Canvas, x: f32, y: f32, edge: f32, color: (u8, u8, u8, u8)) {
        let x2 = x + edge;
        let y2 = y + edge;
        rectangle(canvas, x, y, x2, y2, color);
    }

    pub fn quad(canvas: &mut canvas::Canvas, points: &Vec<f32>, color: (u8, u8, u8, u8)) {
        if points.len() >= 2 {
            let mut points_new = points.clone();
            points_new.push(points[0]);
            points_new.push(points[1]);
            line_from_segments(canvas, &points_new, color);
        }
    }

    /// Mid Point Circle Algorithm
    pub fn circle(canvas: &mut canvas::Canvas, x: f32, y: f32, radius: f32, color: (u8, u8, u8, u8)) {
        let radius = radius as isize;
        let mut x0 = 0;
        let mut y0 = radius;
        let mut d = 3 - (radius << 1);

        let x = x as isize;
        let y = y as isize;

        put_symmetric_pixels(canvas, x, y, x0, y0, color);

        while x0 <= y0 {
            if d <= 0 {
                d += (x0 << 2) + 6;
            } else {
                d += (x0 << 2) - (y0 << 2) + 10;
                y0 -= 1;
            }
            x0 += 1;
            put_symmetric_pixels(canvas, x, y, x0, y0, color);
        }
    }

    fn put_symmetric_pixels(
        canvas: &mut canvas::Canvas,
        x: isize,
        y: isize,
        x_center: isize,
        y_center: isize,
        color: (u8, u8, u8, u8),
    ) {
        // Octant 1
        canvas.set_pixel_at(-(y + y_center) as usize, (x + x_center) as usize, color);
        canvas.set_pixel_at(-(-y + y_center) as usize, (x + x_center) as usize, color);
        canvas.set_pixel_at(-(-y + y_center) as usize, (-x + x_center) as usize, color);
        canvas.set_pixel_at(-(y + y_center) as usize, (-x + x_center) as usize, color);
        canvas.set_pixel_at(-(x + y_center) as usize, (y + x_center) as usize, color);
        canvas.set_pixel_at(-(-x + y_center) as usize, (y + x_center) as usize, color);
        canvas.set_pixel_at(-(-x + y_center) as usize, (-y + x_center) as usize, color);
        canvas.set_pixel_at(-(x + y_center) as usize, (-y + x_center) as usize, color);
        // Octant 2
        canvas.set_pixel_at(-(x + x_center) as usize, (y + y_center) as usize, color);
        canvas.set_pixel_at(-(x + x_center) as usize, (-y + y_center) as usize, color);
        canvas.set_pixel_at(-(-x + x_center) as usize, (-y + y_center) as usize, color);
        canvas.set_pixel_at(-(-x + x_center) as usize, (y + y_center) as usize, color);
        canvas.set_pixel_at(-(y + x_center) as usize, (x + y_center) as usize, color);
        canvas.set_pixel_at(-(y + x_center) as usize, (-x + y_center) as usize, color);
        canvas.set_pixel_at(-(-y + x_center) as usize, (-x + y_center) as usize, color);
        canvas.set_pixel_at(-(-y + x_center) as usize, (x + y_center) as usize, color);
        // Octant 3
        canvas.set_pixel_at((x + x_center) as usize, (y + y_center) as usize, color);
        canvas.set_pixel_at((x + x_center) as usize, (-y + y_center) as usize, color);
        canvas.set_pixel_at((-x + x_center) as usize, (-y + y_center) as usize, color);
        canvas.set_pixel_at((-x + x_center) as usize, (y + y_center) as usize, color);
        canvas.set_pixel_at((y + x_center) as usize, (x + y_center) as usize, color);
        canvas.set_pixel_at((y + x_center) as usize, (-x + y_center) as usize, color);
        canvas.set_pixel_at((-y + x_center) as usize, (-x + y_center) as usize, color);
        canvas.set_pixel_at((-y + x_center) as usize, (x + y_center) as usize, color);
        // Octant 4
        canvas.set_pixel_at((y + y_center) as usize, (x + x_center) as usize, color);
        canvas.set_pixel_at((-y + y_center) as usize, (x + x_center) as usize, color);
        canvas.set_pixel_at((-y + y_center) as usize, (-x + x_center) as usize, color);
        canvas.set_pixel_at((y + y_center) as usize, (-x + x_center) as usize, color);
        canvas.set_pixel_at((x + y_center) as usize, (y + x_center) as usize, color);
        canvas.set_pixel_at((-x + y_center) as usize, (y + x_center) as usize, color);
        canvas.set_pixel_at((-x + y_center) as usize, (-y + x_center) as usize, color);
        canvas.set_pixel_at((x + y_center) as usize, (-y + x_center) as usize, color);
        // Octant 5
        canvas.set_pixel_at((y + y_center) as usize, -(x + x_center) as usize, color);
        canvas.set_pixel_at((-y + y_center) as usize, -(x + x_center) as usize, color);
        canvas.set_pixel_at((-y + y_center) as usize, -(-x + x_center) as usize, color);
        canvas.set_pixel_at((y + y_center) as usize, -(-x + x_center) as usize, color);
        canvas.set_pixel_at((x + y_center) as usize, -(y + x_center) as usize, color);
        canvas.set_pixel_at((-x + y_center) as usize, -(y + x_center) as usize, color);
        canvas.set_pixel_at((-x + y_center) as usize, -(-y + x_center) as usize, color);
        canvas.set_pixel_at((x + y_center) as usize, -(-y + x_center) as usize, color);
        // Octant 6
        canvas.set_pixel_at((x + x_center) as usize, -(y + y_center) as usize, color);
        canvas.set_pixel_at((x + x_center) as usize, -(-y + y_center) as usize, color);
        canvas.set_pixel_at((-x + x_center) as usize, -(-y + y_center) as usize, color);
        canvas.set_pixel_at((-x + x_center) as usize, -(y + y_center) as usize, color);
        canvas.set_pixel_at((y + x_center) as usize, -(x + y_center) as usize, color);
        canvas.set_pixel_at((y + x_center) as usize, -(-x + y_center) as usize, color);
        canvas.set_pixel_at((-y + x_center) as usize, -(-x + y_center) as usize, color);
        canvas.set_pixel_at((-y + x_center) as usize, -(x + y_center) as usize, color);
        // Octant 7
        canvas.set_pixel_at(-(x + x_center) as usize, -(y + y_center) as usize, color);
        canvas.set_pixel_at(-(x + x_center) as usize, -(-y + y_center) as usize, color);
        canvas.set_pixel_at(-(-x + x_center) as usize, -(-y + y_center) as usize, color);
        canvas.set_pixel_at(-(-x + x_center) as usize, -(y + y_center) as usize, color);
        canvas.set_pixel_at(-(y + x_center) as usize, -(x + y_center) as usize, color);
        canvas.set_pixel_at(-(y + x_center) as usize, -(-x + y_center) as usize, color);
        canvas.set_pixel_at(-(-y + x_center) as usize, -(-x + y_center) as usize, color);
        canvas.set_pixel_at(-(-y + x_center) as usize, -(x + y_center) as usize, color);
        // Octant 8
        canvas.set_pixel_at(-(y + y_center) as usize, -(x + x_center) as usize, color);
        canvas.set_pixel_at(-(-y + y_center) as usize, -(x + x_center) as usize, color);
        canvas.set_pixel_at(-(-y + y_center) as usize, -(-x + x_center) as usize, color);
        canvas.set_pixel_at(-(y + y_center) as usize, -(-x + x_center) as usize, color);
        canvas.set_pixel_at(-(x + y_center) as usize, -(y + x_center) as usize, color);
        canvas.set_pixel_at(-(-x + y_center) as usize, -(y + x_center) as usize, color);
        canvas.set_pixel_at(-(-x + y_center) as usize, -(-y + x_center) as usize, color);
        canvas.set_pixel_at(-(x + y_center) as usize, -(-y + x_center) as usize, color);
    }
}
