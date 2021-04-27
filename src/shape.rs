pub mod shape2d {
    use crate::canvas;
    use crate::transforms;
    use crate::Transform;
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

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
                }
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
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::translate((*point).0, (*point).1, tx, ty);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                    }
                }
            }
            self.state.clear();
            self.calculate();
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

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
                }
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
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::translate((*point).0, (*point).1, tx, ty);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                    }
                }
            }
            // self.state.clear();
            // self.calculate();
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
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
                }
            }
        }

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
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
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::translate((*point).0, (*point).1, tx, ty);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                    }
                }
            }
        }
    }

    pub struct Polygon {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Polygon {
        pub fn new(points: Vec<(f32, f32)>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let mut poly = Self {
                points,
                color,
                thickness,
                state: vec![],
            };
            poly.calculate();
            poly
        }

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
                }
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
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::translate((*point).0, (*point).1, tx, ty);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                    }
                }
            }
            self.state.clear();
            self.calculate();
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

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, self.color);
                }
            }
        }

        fn calculate(&mut self) {
            let radius = self.radius as isize;
            let mut x = radius;
            let mut y = 0_isize;
            let mut d = 1 - radius;
            let xc = self.point_center.0 as isize;
            let yc = self.point_center.1 as isize;

            if radius > 0 {
                self.state.push(((x + xc) as f32, (-y + yc) as f32));
                self.state.push(((y + xc) as f32, (x + yc) as f32));
                self.state.push(((-y + xc) as f32, (x + yc) as f32));
            }

            while x > y {
                y += 1;
                if d <= 0 {
                    d += (y << 1) + 1;
                } else {
                    x -= 1;
                    d += (y << 1) - (x << 1) + 1;
                }
                if x < y {
                    break;
                }
                self.state.push(((x + xc) as f32, (y + yc) as f32));
                self.state.push(((-x + xc) as f32, (y + yc) as f32));
                self.state.push(((x + xc) as f32, (-y + yc) as f32));
                self.state.push(((-x + xc) as f32, (-y + yc) as f32));

                if x != y {
                    self.state.push(((y + xc) as f32, (x + yc) as f32));
                    self.state.push(((-y + xc) as f32, (x + yc) as f32));
                    self.state.push(((y + xc) as f32, (-x + yc) as f32));
                    self.state.push(((-y + xc) as f32, (-x + yc) as f32));
                }
            }
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    self.point_center = transforms::translate(self.point_center.0, self.point_center.1, tx, ty);
                    self.state.clear();
                    self.calculate();
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    self.point_center =
                        transforms::rotate(self.point_center.0, self.point_center.1, x_pivot, y_pivot, angle);
                    self.state.clear();
                    self.calculate();
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                    }
                    self.point_center =
                        transforms::shear_x(self.point_center.0, self.point_center.1, x_ref, y_ref, shx);
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.state.iter_mut() {
                        *point = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                    }
                    self.point_center =
                        transforms::shear_y(self.point_center.0, self.point_center.1, x_ref, y_ref, shy);
                }
            }
        }
    }
}
