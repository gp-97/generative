pub mod shape2d {
    use crate::canvas;

    pub struct Line {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Line {
        pub fn new(points: Vec<(f32, f32)>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let line = Self {
                points,
                color,
                thickness,
                state: vec![],
            };
            line
        }
        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            if !self.points.is_empty() {
                if self.points.len() == 1 {
                    let x = self.points[0].0;
                    let y = self.points[0].1;
                    canvas.set_pixel_at(x as usize, y as usize, self.color);
                    self.state.push((x, y));
                } else {
                    for i in 0..(self.points.len() - 1) {
                        let x1 = self.points[i].0;
                        let y1 = self.points[i].1;
                        let x2 = self.points[i + 1].0;
                        let y2 = self.points[i + 1].1;

                        if x1 == x2 {
                            let x1 = x1 as isize;
                            let mut y_start = y1 as isize;
                            let mut y_end = y2 as isize;

                            if y2 < y1 {
                                y_start = y2 as isize;
                                y_end = y1 as isize;
                            }
                            while y_start <= y_end {
                                canvas.set_pixel_at(x1 as usize, y_start as usize, self.color);
                                self.state.push((x1 as f32, y_start as f32));
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
                                canvas.set_pixel_at(x_start as usize, y1 as usize, self.color);
                                self.state.push((x_start as f32, y1 as f32));
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
                                canvas.set_pixel_at(x1 as usize, y1 as usize, self.color);
                                self.state.push((x1 as f32, y1 as f32));
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
                }
            }
        }
    }

    pub struct Rectangle {
        points: [(f32, f32); 2],
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Rectangle {
        pub fn new(points: [(f32, f32); 2], color: (u8, u8, u8, u8), thickness: u8) -> Self {
            Self {
                points,
                color,
                thickness,
                state: vec![],
            }
        }
        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            if self.points.len() == 2 {
                let x1 = self.points[0].0;
                let y1 = self.points[0].1;
                let x2 = self.points[1].0;
                let y2 = self.points[1].1;

                let mut line = Line::new(vec![(x1, y1), (x1, y2)], self.color, self.thickness);
                line.draw(canvas);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let mut line = Line::new(vec![(x1, y2), (x2, y2)], self.color, self.thickness);
                line.draw(canvas);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let mut line = Line::new(vec![(x2, y2), (x2, y1)], self.color, self.thickness);
                line.draw(canvas);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let mut line = Line::new(vec![(x2, y1), (x1, y1)], self.color, self.thickness);
                line.draw(canvas);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
            }
        }
    }

    pub struct Square {
        points: (f32, f32),
        edge: f32,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Square {
        pub fn new(points: (f32, f32), edge: f32, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            Self {
                points,
                edge,
                color,
                thickness,
                state: vec![],
            }
        }
        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            let x1 = self.points.0;
            let y1 = self.points.1;
            let x2 = x1 + self.edge;
            let y2 = y1 + self.edge;

            let mut rect = Rectangle::new([(x1, y1), (x2, y2)], self.color, self.thickness);
            rect.draw(canvas);
            for point in rect.state.iter() {
                self.state.push(*point);
            }
        }
    }

    pub struct Quad {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Quad {
        pub fn new(points: Vec<(f32, f32)>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            Self {
                points,
                color,
                thickness,
                state: vec![],
            }
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            let p_first = self.points[0];
            self.points.push(p_first);
            let mut line = Line::new(self.points.clone(), self.color, self.thickness);
            line.draw(canvas);
            for point in line.state.iter() {
                self.state.push(*point);
            }
        }
    }

    pub struct Circle {
        point_center: (f32, f32),
        radius: f32,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Circle {
        pub fn new(point_center: (f32, f32), radius: f32, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            Self {
                point_center,
                radius,
                color,
                thickness,
                state: vec![],
            }
        }
        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            let radius = self.radius as isize;
            let mut x0 = 0;
            let mut y0 = radius;
            let mut d = 3 - (radius << 1);

            let x = self.point_center.0 as isize;
            let y = self.point_center.1 as isize;

            self.put_symmetric_pixels(canvas, x0, y0);

            while x0 <= y0 {
                if d <= 0 {
                    d += (x0 << 2) + 6;
                } else {
                    d += (x0 << 2) - (y0 << 2) + 10;
                    y0 -= 1;
                }
                x0 += 1;
                self.put_symmetric_pixels(canvas, x0, y0);
            }
        }

        fn put_symmetric_pixels(&mut self, canvas: &mut canvas::Canvas, x_center: isize, y_center: isize) {
            let x = self.point_center.0 as isize;
            let y = self.point_center.1 as isize;
            let color = self.color;
            // Octant 1
            canvas.set_pixel_at(-(y + y_center) as usize, (x + x_center) as usize, color);
            canvas.set_pixel_at(-(-y + y_center) as usize, (x + x_center) as usize, color);
            canvas.set_pixel_at(-(-y + y_center) as usize, (-x + x_center) as usize, color);
            canvas.set_pixel_at(-(y + y_center) as usize, (-x + x_center) as usize, color);
            canvas.set_pixel_at(-(x + y_center) as usize, (y + x_center) as usize, color);
            canvas.set_pixel_at(-(-x + y_center) as usize, (y + x_center) as usize, color);
            canvas.set_pixel_at(-(-x + y_center) as usize, (-y + x_center) as usize, color);
            canvas.set_pixel_at(-(x + y_center) as usize, (-y + x_center) as usize, color);

            self.state.push((-(y + y_center) as f32, (x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, (x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, (-x + x_center) as f32));
            self.state.push((-(y + y_center) as f32, (-x + x_center) as f32));
            self.state.push((-(x + y_center) as f32, (y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, (y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, (-y + x_center) as f32));
            self.state.push((-(x + y_center) as f32, (-y + x_center) as f32));

            // Octant 2
            canvas.set_pixel_at(-(x + x_center) as usize, (y + y_center) as usize, color);
            canvas.set_pixel_at(-(x + x_center) as usize, (-y + y_center) as usize, color);
            canvas.set_pixel_at(-(-x + x_center) as usize, (-y + y_center) as usize, color);
            canvas.set_pixel_at(-(-x + x_center) as usize, (y + y_center) as usize, color);
            canvas.set_pixel_at(-(y + x_center) as usize, (x + y_center) as usize, color);
            canvas.set_pixel_at(-(y + x_center) as usize, (-x + y_center) as usize, color);
            canvas.set_pixel_at(-(-y + x_center) as usize, (-x + y_center) as usize, color);
            canvas.set_pixel_at(-(-y + x_center) as usize, (x + y_center) as usize, color);

            self.state.push((-(x + x_center) as f32, (y + y_center) as f32));
            self.state.push((-(x + x_center) as f32, (-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, (-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, (y + y_center) as f32));
            self.state.push((-(y + x_center) as f32, (x + y_center) as f32));
            self.state.push((-(y + x_center) as f32, (-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, (-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, (x + y_center) as f32));

            // Octant 3
            canvas.set_pixel_at((x + x_center) as usize, (y + y_center) as usize, color);
            canvas.set_pixel_at((x + x_center) as usize, (-y + y_center) as usize, color);
            canvas.set_pixel_at((-x + x_center) as usize, (-y + y_center) as usize, color);
            canvas.set_pixel_at((-x + x_center) as usize, (y + y_center) as usize, color);
            canvas.set_pixel_at((y + x_center) as usize, (x + y_center) as usize, color);
            canvas.set_pixel_at((y + x_center) as usize, (-x + y_center) as usize, color);
            canvas.set_pixel_at((-y + x_center) as usize, (-x + y_center) as usize, color);
            canvas.set_pixel_at((-y + x_center) as usize, (x + y_center) as usize, color);

            self.state.push(((x + x_center) as f32, (y + y_center) as f32));
            self.state.push(((x + x_center) as f32, (-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, (-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, (y + y_center) as f32));
            self.state.push(((y + x_center) as f32, (x + y_center) as f32));
            self.state.push(((y + x_center) as f32, (-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, (-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, (x + y_center) as f32));

            // Octant 4
            canvas.set_pixel_at((y + y_center) as usize, (x + x_center) as usize, color);
            canvas.set_pixel_at((-y + y_center) as usize, (x + x_center) as usize, color);
            canvas.set_pixel_at((-y + y_center) as usize, (-x + x_center) as usize, color);
            canvas.set_pixel_at((y + y_center) as usize, (-x + x_center) as usize, color);
            canvas.set_pixel_at((x + y_center) as usize, (y + x_center) as usize, color);
            canvas.set_pixel_at((-x + y_center) as usize, (y + x_center) as usize, color);
            canvas.set_pixel_at((-x + y_center) as usize, (-y + x_center) as usize, color);
            canvas.set_pixel_at((x + y_center) as usize, (-y + x_center) as usize, color);

            self.state.push(((y + y_center) as f32, (x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, (x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, (-x + x_center) as f32));
            self.state.push(((y + y_center) as f32, (-x + x_center) as f32));
            self.state.push(((x + y_center) as f32, (y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, (y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, (-y + x_center) as f32));
            self.state.push(((x + y_center) as f32, (-y + x_center) as f32));

            // Octant 5
            canvas.set_pixel_at((y + y_center) as usize, -(x + x_center) as usize, color);
            canvas.set_pixel_at((-y + y_center) as usize, -(x + x_center) as usize, color);
            canvas.set_pixel_at((-y + y_center) as usize, -(-x + x_center) as usize, color);
            canvas.set_pixel_at((y + y_center) as usize, -(-x + x_center) as usize, color);
            canvas.set_pixel_at((x + y_center) as usize, -(y + x_center) as usize, color);
            canvas.set_pixel_at((-x + y_center) as usize, -(y + x_center) as usize, color);
            canvas.set_pixel_at((-x + y_center) as usize, -(-y + x_center) as usize, color);
            canvas.set_pixel_at((x + y_center) as usize, -(-y + x_center) as usize, color);

            self.state.push(((y + y_center) as f32, -(x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, -(x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push(((y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push(((x + y_center) as f32, -(y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, -(y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, -(-y + x_center) as f32));
            self.state.push(((x + y_center) as f32, -(-y + x_center) as f32));

            // Octant 6
            canvas.set_pixel_at((x + x_center) as usize, -(y + y_center) as usize, color);
            canvas.set_pixel_at((x + x_center) as usize, -(-y + y_center) as usize, color);
            canvas.set_pixel_at((-x + x_center) as usize, -(-y + y_center) as usize, color);
            canvas.set_pixel_at((-x + x_center) as usize, -(y + y_center) as usize, color);
            canvas.set_pixel_at((y + x_center) as usize, -(x + y_center) as usize, color);
            canvas.set_pixel_at((y + x_center) as usize, -(-x + y_center) as usize, color);
            canvas.set_pixel_at((-y + x_center) as usize, -(-x + y_center) as usize, color);
            canvas.set_pixel_at((-y + x_center) as usize, -(x + y_center) as usize, color);

            self.state.push(((x + x_center) as f32, -(y + y_center) as f32));
            self.state.push(((x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, -(y + y_center) as f32));
            self.state.push(((y + x_center) as f32, -(x + y_center) as f32));
            self.state.push(((y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, -(x + y_center) as f32));

            // Octant 7
            canvas.set_pixel_at(-(x + x_center) as usize, -(y + y_center) as usize, color);
            canvas.set_pixel_at(-(x + x_center) as usize, -(-y + y_center) as usize, color);
            canvas.set_pixel_at(-(-x + x_center) as usize, -(-y + y_center) as usize, color);
            canvas.set_pixel_at(-(-x + x_center) as usize, -(y + y_center) as usize, color);
            canvas.set_pixel_at(-(y + x_center) as usize, -(x + y_center) as usize, color);
            canvas.set_pixel_at(-(y + x_center) as usize, -(-x + y_center) as usize, color);
            canvas.set_pixel_at(-(-y + x_center) as usize, -(-x + y_center) as usize, color);
            canvas.set_pixel_at(-(-y + x_center) as usize, -(x + y_center) as usize, color);

            self.state.push((-(x + x_center) as f32, -(y + y_center) as f32));
            self.state.push((-(x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, -(y + y_center) as f32));
            self.state.push((-(y + x_center) as f32, -(x + y_center) as f32));
            self.state.push((-(y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, -(x + y_center) as f32));

            // Octant 8
            canvas.set_pixel_at(-(y + y_center) as usize, -(x + x_center) as usize, color);
            canvas.set_pixel_at(-(-y + y_center) as usize, -(x + x_center) as usize, color);
            canvas.set_pixel_at(-(-y + y_center) as usize, -(-x + x_center) as usize, color);
            canvas.set_pixel_at(-(y + y_center) as usize, -(-x + x_center) as usize, color);
            canvas.set_pixel_at(-(x + y_center) as usize, -(y + x_center) as usize, color);
            canvas.set_pixel_at(-(-x + y_center) as usize, -(y + x_center) as usize, color);
            canvas.set_pixel_at(-(-x + y_center) as usize, -(-y + x_center) as usize, color);
            canvas.set_pixel_at(-(x + y_center) as usize, -(-y + x_center) as usize, color);

            self.state.push((-(y + y_center) as f32, -(x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, -(x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push((-(y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push((-(x + y_center) as f32, -(y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, -(y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, -(-y + x_center) as f32));
            self.state.push((-(x + y_center) as f32, -(-y + x_center) as f32));
        }
    }
}
