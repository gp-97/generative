pub mod shape2d {
    use crate::canvas;
    use crate::transforms;
    use crate::Transform;
    use std::mem::swap;
    pub struct Line {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32, (u8, u8, u8, u8))>,
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
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, (*point).2);
                }
            }
        }

        fn ipart(&self, x: f32) -> isize {
            x as isize
        }
        fn round(&self, x: f32) -> isize {
            self.ipart(x + 0.5)
        }
        fn fpart(&self, x: f32) -> f32 {
            x - self.ipart(x) as f32
        }
        fn rfpart(&self, x: f32) -> f32 {
            1.0 - self.fpart(x)
        }

        fn build_state(&mut self, x: isize, y: isize, c: f32) {
            let r = (self.color.0 as f32 * c) as u8;
            let g = (self.color.1 as f32 * c) as u8;
            let b = (self.color.2 as f32 * c) as u8;
            let a = self.color.3;
            self.state.push((x as f32, y as f32, (r, g, b, a)));
        }

        fn calculate(&mut self) {
            if !self.points.is_empty() {
                if self.points.len() == 1 {
                    let x = self.points[0].0;
                    let y = self.points[0].1;
                    self.state.push((x, y, self.color));
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
                                self.state.push((x1 as f32, y_start as f32, self.color));
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
                                self.state.push((x_start as f32, y1 as f32, self.color));
                                x_start += 1;
                            }
                        } else {
                            // Anti-aliased line generation
                            let mut x1 = x1;
                            let mut y1 = y1;
                            let mut x2 = x2;
                            let mut y2 = y2;

                            let dx = x2 - x1;
                            let dy = y2 - y1;

                            if dx.abs() > dy.abs() {
                                if x2 < x1 {
                                    swap(&mut x1, &mut x2);
                                    swap(&mut y1, &mut y2);
                                }
                                let gradient = dy / dx;
                                let mut xend = self.round(x1) as f32;
                                let mut yend = y1 + gradient * (xend - x1);
                                let mut xgap = self.rfpart(x1 + 0.5);

                                let xpxl1 = xend as isize;
                                let ypxl1 = self.ipart(yend);

                                self.build_state(xpxl1, ypxl1, self.rfpart(yend) * xgap);
                                self.build_state(xpxl1, ypxl1 + 1, self.fpart(yend) * xgap);

                                let mut intery = yend + gradient;
                                xend = self.round(x2) as f32;
                                yend = y2 + gradient * (xend - x2);
                                xgap = self.fpart(x2 + 0.5);

                                let xpxl2 = xend as isize;
                                let ypxl2 = self.ipart(yend);

                                self.build_state(xpxl2, ypxl2, self.rfpart(yend) * xgap);
                                self.build_state(xpxl2, ypxl2 + 1, self.fpart(yend) * xgap);

                                let mut x = xpxl1 + 1;
                                while x < xpxl2 {
                                    self.build_state(x, self.ipart(intery), self.rfpart(intery));
                                    self.build_state(x, self.ipart(intery) + 1, self.fpart(intery));
                                    intery += gradient;
                                    x += 1;
                                }
                            } else {
                                if y2 < y1 {
                                    swap(&mut x1, &mut x2);
                                    swap(&mut y1, &mut y2);
                                }

                                let gradient = dx / dy;
                                let mut yend = self.round(y1) as f32;
                                let mut xend = x1 + gradient * (yend - y1);
                                let mut ygap = self.rfpart(y1 + 0.5);

                                let ypxl1 = yend as isize;
                                let xpxl1 = self.ipart(xend);

                                self.build_state(xpxl1, ypxl1, self.rfpart(xend) * ygap);
                                self.build_state(xpxl1, ypxl1 + 1, self.fpart(xend) * ygap);

                                let mut interx = xend + gradient;

                                yend = self.round(y2) as f32;
                                xend = x2 + gradient * (yend - y2);
                                ygap = self.fpart(y2 + 0.5);

                                let ypxl2 = yend as isize;
                                let xpxl2 = self.ipart(xend);

                                self.build_state(xpxl2, ypxl2, self.rfpart(xend) * ygap);
                                self.build_state(xpxl2, ypxl2 + 1, self.fpart(xend) * ygap);

                                let mut y = ypxl1 + 1;
                                while y < ypxl2 {
                                    self.build_state(self.ipart(interx), y, self.rfpart(interx));
                                    self.build_state(self.ipart(interx) + 1, y, self.fpart(interx));
                                    interx += gradient;
                                    y += 1;
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
        vertices: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32, (u8, u8, u8, u8))>,
    }

    impl Rectangle {
        pub fn new(points: [(f32, f32); 2], color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let x1 = points[0].0;
            let y1 = points[0].1;
            let x2 = points[1].0;
            let y2 = points[1].1;
            let vertices = vec![(x1, y1), (x1, y2), (x2, y2), (x2, y1)];
            let mut rect = Self {
                points,
                vertices,
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

        fn get_vertices(&self) -> Vec<(f32, f32)> {
            self.vertices.clone()
        }

        fn set_vertices(&mut self, vertices: Vec<(f32, f32)>) {
            self.vertices = vertices;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            for point in self.state.iter() {
                if (*point).0 >= 0.0 && (*point).1 >= 0.0 {
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, (*point).2);
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
                        let (x, y) = transforms::translate((*point).0, (*point).1, tx, ty);
                        *point = (x, y, (*point).2);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    let mut points = self.get_vertices();
                    for point in points.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                    self.set_vertices(points.clone());
                    self.state.clear();

                    let line = Line::new(vec![points[0], points[1]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[1], points[2]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[2], points[3]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[3], points[0]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.state.iter_mut() {
                        let (x, y) = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                        *point = (x, y, (*point).2);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.state.iter_mut() {
                        let (x, y) = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                        *point = (x, y, (*point).2);
                    }
                }
            }
        }
    }

    pub struct Square {
        points: (f32, f32),
        edge: f32,
        vertices: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32, (u8, u8, u8, u8))>,
    }

    impl Square {
        pub fn new(points: (f32, f32), edge: f32, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let x1 = points.0;
            let y1 = points.1;
            let x2 = x1 + edge;
            let y2 = y1 + edge;
            let vertices = vec![(x1, y1), (x1, y2), (x2, y2), (x2, y1)];
            let mut square = Self {
                points,
                edge,
                vertices,
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
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, (*point).2);
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

        fn set_vertices(&mut self, vertices: Vec<(f32, f32)>) {
            self.vertices = vertices;
        }

        fn get_vertices(&self) -> Vec<(f32, f32)> {
            self.vertices.clone()
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
                        let (x, y) = transforms::translate((*point).0, (*point).1, tx, ty);
                        *point = (x, y, (*point).2);
                    }
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    let mut points = self.get_vertices();
                    for point in points.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                    self.set_vertices(points.clone());
                    self.state.clear();

                    let line = Line::new(vec![points[0], points[1]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[1], points[2]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[2], points[3]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                    let line = Line::new(vec![points[3], points[0]], self.color, self.thickness);
                    for point in line.state.iter() {
                        self.state.push(*point);
                    }
                }
                Transform::ShearX(x_ref, y_ref, shx) => {
                    for point in self.state.iter_mut() {
                        let (x, y) = transforms::shear_x((*point).0, (*point).1, x_ref, y_ref, shx);
                        *point = (x, y, (*point).2);
                    }
                }
                Transform::ShearY(x_ref, y_ref, shy) => {
                    for point in self.state.iter_mut() {
                        let (x, y) = transforms::shear_y((*point).0, (*point).1, x_ref, y_ref, shy);
                        *point = (x, y, (*point).2);
                    }
                }
            }
        }
    }

    pub struct Polygon {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32, (u8, u8, u8, u8))>,
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
                    canvas.set_pixel_at((*point).0 as usize, (*point).1 as usize, (*point).2);
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

pub mod curve {
    use super::shape2d::Line;
    use crate::canvas;
    use crate::helpers::comb;
    use crate::helpers::linspace;
    use crate::transforms;
    use crate::Spline;
    use crate::Transform;
    pub struct Curve {
        points: Vec<(f32, f32)>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        npoints: u32,
        spline: Spline,
        state: Vec<(f32, f32)>,
    }

    impl Curve {
        pub fn new(
            points: Vec<(f32, f32)>,
            color: (u8, u8, u8, u8),
            thickness: u8,
            npoints: u32,
            spline: Spline,
        ) -> Self {
            let mut curve = Self {
                points,
                color,
                thickness,
                npoints,
                spline,
                state: vec![],
            };
            curve.calculate();
            curve
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

        /// https://en.wikipedia.org/wiki/Centripetal_Catmull%E2%80%93Rom_spline
        pub fn calculate(&mut self) {
            let size = self.points.len();

            let mut curve = vec![];
            for i in 0..(size - 3) {
                let mut c = self.catmull_rom_spline(
                    self.points[i],
                    self.points[i + 1],
                    self.points[i + 2],
                    self.points[i + 3],
                );
                curve.append(&mut c);
            }
            self.state = curve;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            let drawable = self.state.clone();
            let line = Line::new(drawable, self.color, self.thickness);
            line.draw(canvas);
        }
        fn knot_j(&self, knot_i: f32, pi: (f32, f32), pj: (f32, f32), alpha: f32) -> f32 {
            let (xi, yi) = pi;
            let (xj, yj) = pj;
            ((xj - xi).powf(2.0) + (yj - yi).powf(2.0)).powf(alpha) + knot_i
        }
        fn catmull_rom_spline(
            &self,
            p0: (f32, f32),
            p1: (f32, f32),
            p2: (f32, f32),
            p3: (f32, f32),
        ) -> Vec<(f32, f32)> {
            let mut alpha = match self.spline {
                Spline::UNIFORM => 0.0,
                Spline::CENTRIPETAL => 0.5,
                Spline::CHORDAL => 1.0,
            };

            alpha /= 2.0;

            let t0 = 0.0;
            let t1 = self.knot_j(t0, p0, p1, alpha);
            let t2 = self.knot_j(t1, p1, p2, alpha);
            let t3 = self.knot_j(t2, p2, p3, alpha);

            let t_lin = linspace(t1, t2, self.npoints);
            let mut c = vec![];

            for i in 0..t_lin.len() {
                let t = t_lin[i];
                let ca10 = (t1 - t) / (t1 - t0);
                let ca11 = (t - t0) / (t1 - t0);

                let ca20 = (t2 - t) / (t2 - t1);
                let ca21 = (t - t1) / (t2 - t1);

                let ca30 = (t3 - t) / (t3 - t2);
                let ca31 = (t - t2) / (t3 - t2);

                let a1 = (ca10 * p0.0 + ca11 * p1.0, ca10 * p0.1 + ca11 * p1.1);
                let a2 = (ca20 * p1.0 + ca21 * p2.0, ca20 * p1.1 + ca21 * p2.1);
                let a3 = (ca30 * p2.0 + ca31 * p3.0, ca30 * p2.1 + ca31 * p2.1);

                let cb10 = (t2 - t) / (t2 - t0);
                let cb11 = (t - t0) / (t2 - t0);
                let cb20 = (t3 - t) / (t3 - t1);
                let cb21 = (t - t1) / (t3 - t1);

                let b1 = (cb10 * a1.0 + cb11 * a2.0, cb10 * a1.1 + cb11 * a2.1);
                let b2 = (cb20 * a2.0 + cb21 * a3.0, cb20 * a2.1 + cb21 * a3.1);

                let cc0 = ca20;
                let cc1 = ca21;

                let val = (cc0 * b1.0 + cc1 * b2.0, cc0 * b1.1 + cc1 * b2.1);
                c.push(val);
            }
            c
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

    pub struct Bezier {
        npoints: u32,
        control_points: Vec<(f32, f32)>,
        degree: usize,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<(f32, f32)>,
    }

    impl Bezier {
        pub fn new(npoints: u32, control_points: Vec<(f32, f32)>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let degree = control_points.len() - 1;
            let mut bezier = Self {
                npoints,
                control_points,
                degree,
                color,
                thickness,
                state: vec![],
            };
            bezier.calculate();
            bezier
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

        fn blend(&self, i: usize, t: f32) -> f32 {
            let j = comb(self.degree, i) as f32 * t.powf(i as f32) * (1.0 - t).powf((self.degree - i) as f32);
            j
        }

        pub fn calculate(&mut self) {
            let t_lin = linspace(0.0, 1.0, self.npoints);
            for t in t_lin.iter() {
                let mut p = (0.0, 0.0);
                for (i, point) in self.control_points.iter().enumerate() {
                    let (x, y) = *point;
                    let j = self.blend(i, *t);
                    p.0 += x * j;
                    p.1 += y * j;
                }
                self.state.push(p);
            }
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            let drawable = self.state.clone();
            let line = Line::new(drawable, self.color, self.thickness);
            line.draw(canvas);
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.control_points.iter_mut() {
                        *point = transforms::translate((*point).0, (*point).1, tx, ty);
                    }
                    self.state.clear();
                    self.calculate();
                }
                Transform::ROTATE(x_pivot, y_pivot, angle) => {
                    for point in self.control_points.iter_mut() {
                        *point = transforms::rotate((*point).0, (*point).1, x_pivot, y_pivot, angle);
                    }
                    self.state.clear();
                    self.calculate();
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
}
