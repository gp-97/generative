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
            let mut line = Self {
                points,
                color,
                thickness,
                state: vec![],
            };
            line.calculate();
            line
        }
        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
            }
        }
        fn calculate(&mut self) {
            if !self.points.is_empty() {
                if self.points.len() == 1 {
                    let x = self.points[0].0;
                    let y = self.points[0].1;
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
            let mut rect = Self {
                points,
                color,
                thickness,
                state: vec![],
            };
            rect.calculate();
            rect
        }
        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
            }
        }

        fn calculate(&mut self) {
            if self.points.len() == 2 {
                let x1 = self.points[0].0;
                let y1 = self.points[0].1;
                let x2 = self.points[1].0;
                let y2 = self.points[1].1;

                let line = Line::new(vec![(x1, y1), (x1, y2)], self.color, self.thickness);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let line = Line::new(vec![(x1, y2), (x2, y2)], self.color, self.thickness);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let line = Line::new(vec![(x2, y2), (x2, y1)], self.color, self.thickness);
                for point in line.state.iter() {
                    self.state.push(*point);
                }
                let line = Line::new(vec![(x2, y1), (x1, y1)], self.color, self.thickness);
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
            let mut square = Self {
                points,
                edge,
                color,
                thickness,
                state: vec![],
            };
            square.calculate();
            square
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
            }
        }

        fn calculate(&mut self) {
            let x1 = self.points.0;
            let y1 = self.points.1;
            let x2 = x1 + self.edge;
            let y2 = y1 + self.edge;

            let rect = Rectangle::new([(x1, y1), (x2, y2)], self.color, self.thickness);
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
            let mut quad = Self {
                points,
                color,
                thickness,
                state: vec![],
            };
            quad.calculate();
            quad
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
            }
        }

        fn calculate(&mut self) {
            let p_first = self.points[0];
            self.points.push(p_first);
            let line = Line::new(self.points.clone(), self.color, self.thickness);
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
            let mut circle = Self {
                point_center,
                radius,
                color,
                thickness,
                state: vec![],
            };
            circle.calculate();
            circle
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
            }
        }

        fn calculate(&mut self) {
            let radius = self.radius as isize;
            let mut x0 = 0;
            let mut y0 = radius;
            let mut d = 3 - (radius << 1);

            self.put_symmetric_pixels(x0, y0);

            while x0 <= y0 {
                if d <= 0 {
                    d += (x0 << 2) + 6;
                } else {
                    d += (x0 << 2) - (y0 << 2) + 10;
                    y0 -= 1;
                }
                x0 += 1;
                self.put_symmetric_pixels(x0, y0);
            }
        }

        fn put_symmetric_pixels(&mut self, x_center: isize, y_center: isize) {
            let x = self.point_center.0 as isize;
            let y = self.point_center.1 as isize;
            // Octant 1

            self.state.push((-(y + y_center) as f32, (x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, (x + x_center) as f32));
            self.state.push((-(-y + y_center) as f32, (-x + x_center) as f32));
            self.state.push((-(y + y_center) as f32, (-x + x_center) as f32));
            self.state.push((-(x + y_center) as f32, (y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, (y + x_center) as f32));
            self.state.push((-(-x + y_center) as f32, (-y + x_center) as f32));
            self.state.push((-(x + y_center) as f32, (-y + x_center) as f32));

            // Octant 2

            self.state.push((-(x + x_center) as f32, (y + y_center) as f32));
            self.state.push((-(x + x_center) as f32, (-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, (-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, (y + y_center) as f32));
            self.state.push((-(y + x_center) as f32, (x + y_center) as f32));
            self.state.push((-(y + x_center) as f32, (-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, (-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, (x + y_center) as f32));

            // Octant 3

            self.state.push(((x + x_center) as f32, (y + y_center) as f32));
            self.state.push(((x + x_center) as f32, (-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, (-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, (y + y_center) as f32));
            self.state.push(((y + x_center) as f32, (x + y_center) as f32));
            self.state.push(((y + x_center) as f32, (-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, (-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, (x + y_center) as f32));

            // Octant 4

            self.state.push(((y + y_center) as f32, (x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, (x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, (-x + x_center) as f32));
            self.state.push(((y + y_center) as f32, (-x + x_center) as f32));
            self.state.push(((x + y_center) as f32, (y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, (y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, (-y + x_center) as f32));
            self.state.push(((x + y_center) as f32, (-y + x_center) as f32));

            // Octant 5

            self.state.push(((y + y_center) as f32, -(x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, -(x + x_center) as f32));
            self.state.push(((-y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push(((y + y_center) as f32, -(-x + x_center) as f32));
            self.state.push(((x + y_center) as f32, -(y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, -(y + x_center) as f32));
            self.state.push(((-x + y_center) as f32, -(-y + x_center) as f32));
            self.state.push(((x + y_center) as f32, -(-y + x_center) as f32));

            // Octant 6

            self.state.push(((x + x_center) as f32, -(y + y_center) as f32));
            self.state.push(((x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push(((-x + x_center) as f32, -(y + y_center) as f32));
            self.state.push(((y + x_center) as f32, -(x + y_center) as f32));
            self.state.push(((y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push(((-y + x_center) as f32, -(x + y_center) as f32));

            // Octant 7

            self.state.push((-(x + x_center) as f32, -(y + y_center) as f32));
            self.state.push((-(x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, -(-y + y_center) as f32));
            self.state.push((-(-x + x_center) as f32, -(y + y_center) as f32));
            self.state.push((-(y + x_center) as f32, -(x + y_center) as f32));
            self.state.push((-(y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, -(-x + y_center) as f32));
            self.state.push((-(-y + x_center) as f32, -(x + y_center) as f32));

            // Octant 8

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
