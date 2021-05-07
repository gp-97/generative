pub mod shape2d {
    use std::vec;

    use crate::{canvas, transforms};
    use crate::{Pixel, Point, Transform};
    pub struct Line {
        points: Vec<Point>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Line {
        pub fn new(points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
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

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            for pixel in self.state.iter_mut() {
                let point = pixel.get_point();
                let color = self.color;
                pixel.set_color(self.color);
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }
        fn calculate(&mut self) {
            if !self.points.is_empty() {
                if self.points.len() == 1 {
                    let point = self.points[0];
                    let pixel = Pixel::new(point, self.color);
                    self.state.push(pixel);
                } else {
                    for i in 0..(self.points.len() - 1) {
                        let p1 = self.points[i];
                        let p2 = self.points[i + 1];

                        let x1 = p1.get_x();
                        let y1 = p1.get_y();
                        let x2 = p2.get_x();
                        let y2 = p2.get_y();

                        if x1 == x2 {
                            let x1 = x1 as isize;
                            let mut y_start = y1 as isize;
                            let mut y_end = y2 as isize;

                            if y2 < y1 {
                                y_start = y2 as isize;
                                y_end = y1 as isize;
                            }
                            while y_start <= y_end {
                                let pixel = Pixel::new(Point::from((x1 as f32, y_start as f32)), self.color);
                                self.state.push(pixel);
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
                                let pixel = Pixel::new(Point::from((x_start as f32, y1 as f32)), self.color);
                                self.state.push(pixel);
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
                                let pixel = Pixel::new(Point::from((x1 as f32, y1 as f32)), self.color);
                                self.state.push(pixel);
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
                        *point = transforms::translate(point, tx, ty);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate(point, &point_pivot, angle);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x(point, &point_ref, shx);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y(point, &point_ref, shy);
                    }
                }
            }
            self.state.clear();
            self.calculate();
        }
    }

    pub struct Rectangle {
        points: [Point; 2],
        color: (u8, u8, u8, u8),
        thickness: u8,
        vertices: Vec<Pixel>,
        state: Vec<Pixel>,
    }

    impl Rectangle {
        pub fn new(points: [Point; 2], color: (u8, u8, u8, u8), thickness: u8) -> Self {
            // let p1 = points[0]; // (x1, y1)
            // let p2 = points[1]; // (x2, y2)
            // let p3 = Point::new(p1.get_x(), p2.get_y()); //(x1, y2)
            // let p4 = Point::new(p2.get_x(), p1.get_y()); //(x2, y1)

            // let mut vertices = vec![];

            // vertices.push(Pixel::new(p1, color));
            // vertices.push(Pixel::new(p3, color));
            // vertices.push(Pixel::new(p2, color));
            // vertices.push(Pixel::new(p4, color));

            let mut rect = Self {
                points,
                color,
                thickness,
                vertices: vec![],
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

        pub fn get_vertices(&self) -> Vec<Pixel> {
            self.vertices.clone()
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            for pixel in self.state.iter_mut() {
                let point = pixel.get_point();
                let color = self.color;
                pixel.set_color(self.color);
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        fn calculate(&mut self) {
            if self.points.len() == 2 {
                let p1 = self.points[0]; // (x1, y1)
                let p2 = self.points[1]; // (x2, y2)
                let p3 = Point::new(p1.get_x(), p2.get_y()); //(x1, y2)
                let p4 = Point::new(p2.get_x(), p1.get_y()); //(x2, y1)

                let line = Line::new(vec![p1, p3], self.color, self.thickness);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                let line = Line::new(vec![p3, p2], self.color, self.thickness);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                let line = Line::new(vec![p2, p4], self.color, self.thickness);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                let line = Line::new(vec![p4, p1], self.color, self.thickness);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }

                self.vertices.clear();

                self.vertices.push(Pixel::new(p1, self.color));
                self.vertices.push(Pixel::new(p3, self.color));
                self.vertices.push(Pixel::new(p2, self.color));
                self.vertices.push(Pixel::new(p4, self.color));
            }
        }
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::translate(&pixel.get_point(), tx, ty);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::translate(&self.vertices[i].get_point(), tx, ty);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::rotate(&pixel.get_point(), &point_pivot, angle);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::rotate(&self.vertices[i].get_point(), &point_pivot, angle);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_x(&self.vertices[i].get_point(), &point_ref, shx);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_y(&self.vertices[i].get_point(), &point_ref, shy);
                        self.vertices[i].set_point(vpoint);
                    }
                }
            }
        }
    }

    pub struct Square {
        points: Point,
        edge: f32,
        color: (u8, u8, u8, u8),
        thickness: u8,
        vertices: Vec<Pixel>,
        state: Vec<Pixel>,
    }

    impl Square {
        pub fn new(points: Point, edge: f32, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let mut square = Self {
                points,
                edge,
                color,
                thickness,
                state: vec![],
                vertices: vec![],
            };
            square.calculate();
            square
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            for pixel in self.state.iter_mut() {
                let point = pixel.get_point();
                let color = self.color;
                pixel.set_color(self.color);
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn get_vertices(&self) -> Vec<Pixel> {
            self.vertices.clone()
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        fn calculate(&mut self) {
            let p1 = self.points;
            let p2 = Point::new(p1.get_x() + self.edge, p1.get_y() + self.edge);

            let rect = Rectangle::new([p1, p2], self.color, self.thickness);
            for pixel in rect.state.iter() {
                self.state.push(*pixel);
            }
            self.vertices = rect.get_vertices();
        }
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::translate(&pixel.get_point(), tx, ty);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::translate(&self.vertices[i].get_point(), tx, ty);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::rotate(&pixel.get_point(), &point_pivot, angle);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::rotate(&self.vertices[i].get_point(), &point_pivot, angle);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_x(&self.vertices[i].get_point(), &point_ref, shx);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, self.color);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_y(&self.vertices[i].get_point(), &point_ref, shy);
                        self.vertices[i].set_point(vpoint);
                    }
                }
            }
        }
    }

    pub struct Polygon {
        points: Vec<Point>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        vertices: Vec<Pixel>,
        state: Vec<Pixel>,
    }

    impl Polygon {
        pub fn new(points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
            let mut vertices: Vec<Pixel> = vec![];
            for point in points.iter() {
                vertices.push(Pixel::new(*point, color));
            }
            let mut poly = Self {
                points,
                color,
                thickness,
                vertices,
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

        pub fn get_vertices(&self) -> Vec<Pixel> {
            self.vertices.clone()
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            for pixel in self.state.iter_mut() {
                let point = pixel.get_point();
                let color = self.color;
                pixel.set_color(self.color);
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        fn calculate(&mut self) {
            let p_first = self.points[0];
            self.points.push(p_first);
            let line = Line::new(self.points.clone(), self.color, self.thickness);
            for pixel in line.state.iter() {
                self.state.push(*pixel);
            }
        }
        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::translate(point, tx, ty);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::translate(&self.vertices[i].get_point(), tx, ty);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate(point, &point_pivot, angle);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::rotate(&self.vertices[i].get_point(), &point_pivot, angle);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x(point, &point_ref, shx);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_x(&self.vertices[i].get_point(), &point_ref, shx);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y(point, &point_ref, shy);
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_y(&self.vertices[i].get_point(), &point_ref, shy);
                        self.vertices[i].set_point(vpoint);
                    }
                }
            }
            self.state.clear();
            self.calculate();
        }
    }

    pub struct Circle {
        point_center: Point,
        radius: f32,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Circle {
        pub fn new(point_center: Point, radius: f32, color: (u8, u8, u8, u8), thickness: u8) -> Self {
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

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            for pixel in self.state.iter_mut() {
                let point = pixel.get_point();
                let color = self.color;
                pixel.set_color(self.color);
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        fn calculate(&mut self) {
            let radius = self.radius as isize;
            let mut x = radius;
            let mut y = 0_isize;
            let mut d = 1 - radius;
            let xc = self.point_center.get_x() as isize;
            let yc = self.point_center.get_y() as isize;

            if radius > 0 {
                let point = Point::new((x + xc) as f32, (-y + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);
                let point = Point::new((y + xc) as f32, (x + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);
                let point = Point::new((-y + xc) as f32, (x + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);
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
                let point = Point::new((x + xc) as f32, (y + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);

                let point = Point::new((-x + xc) as f32, (y + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);

                let point = Point::new((x + xc) as f32, (-y + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);

                let point = Point::new((-x + xc) as f32, (-y + yc) as f32);
                let pixel = Pixel::new(point, self.get_color());
                self.state.push(pixel);

                if x != y {
                    let point = Point::new((y + xc) as f32, (x + yc) as f32);
                    let pixel = Pixel::new(point, self.get_color());
                    self.state.push(pixel);

                    let point = Point::new((-y + xc) as f32, (x + yc) as f32);
                    let pixel = Pixel::new(point, self.get_color());
                    self.state.push(pixel);

                    let point = Point::new((y + xc) as f32, (-x + yc) as f32);
                    let pixel = Pixel::new(point, self.get_color());
                    self.state.push(pixel);

                    let point = Point::new((-y + xc) as f32, (-x + yc) as f32);
                    let pixel = Pixel::new(point, self.get_color());
                    self.state.push(pixel);
                }
            }
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    self.point_center = transforms::translate(&self.point_center, tx, ty);
                    self.state.clear();
                    self.calculate();
                }
                Transform::ROTATE(point_pivot, angle) => {
                    self.point_center = transforms::rotate(&self.point_center, &point_pivot, angle);
                    self.state.clear();
                    self.calculate();
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    self.point_center = transforms::shear_x(&self.point_center, &point_ref, shx);
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    self.point_center = transforms::shear_y(&self.point_center, &point_ref, shy);
                }
            }
        }
    }
}

pub mod curve {
    use super::shape2d::Line;
    use crate::helpers::{comb, linspace};
    use crate::{canvas, transforms};
    use crate::{Pixel, Point, Spline, Transform};
    pub struct Curve {
        points: Vec<Point>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        npoints: u32,
        spline: Spline,
        state: Vec<Pixel>,
    }

    impl Curve {
        pub fn new(points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8, npoints: u32, spline: Spline) -> Self {
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
                    &self.points[i],
                    &self.points[i + 1],
                    &self.points[i + 2],
                    &self.points[i + 3],
                );
                curve.append(&mut c);
            }
            self.state = curve;
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            let points = self.state.iter().map(|pixel| pixel.get_point()).collect();
            let mut line = Line::new(points, self.color, self.thickness);
            line.draw(canvas);
        }
        fn knot_j(&self, knot_i: f32, pi: &Point, pj: &Point, alpha: f32) -> f32 {
            ((pj.get_x() - pi.get_x()).powf(2.0) + (pj.get_y() - pi.get_y()).powf(2.0)).powf(alpha) + knot_i
        }
        fn catmull_rom_spline(&self, p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> Vec<Pixel> {
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

            let p0 = (p0.get_x(), p0.get_y());
            let p1 = (p1.get_x(), p1.get_y());
            let p2 = (p2.get_x(), p2.get_y());
            let p3 = (p3.get_x(), p3.get_y());

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
                let point = Point::from(val);
                let pixel = Pixel::new(point, self.color);
                c.push(pixel);
            }
            c
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::translate(point, tx, ty);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate(point, &point_pivot, angle);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x(point, &point_ref, shx);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y(point, &point_ref, shy);
                    }
                }
            }
            self.state.clear();
            self.calculate();
        }
    }

    pub struct Bezier {
        npoints: u32,
        control_points: Vec<Point>,
        degree: usize,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Bezier {
        pub fn new(npoints: u32, control_points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8) -> Self {
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
                    let x = point.get_x();
                    let y = point.get_y();
                    let j = self.blend(i, *t);
                    p.0 += x * j;
                    p.1 += y * j;
                }
                let point = Point::from(p);
                let pixel = Pixel::new(point, self.color);
                self.state.push(pixel);
            }
        }

        pub fn draw(&self, canvas: &mut canvas::Canvas) {
            let points = self.state.iter().map(|pixel| pixel.get_point()).collect();
            let mut line = Line::new(points, self.color, self.thickness);
            line.draw(canvas);
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.control_points.iter_mut() {
                        *point = transforms::translate(point, tx, ty);
                    }
                    self.state.clear();
                    self.calculate();
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for point in self.control_points.iter_mut() {
                        *point = transforms::rotate(point, &point_pivot, angle);
                    }
                    self.state.clear();
                    self.calculate();
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, self.color);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, self.color);
                    }
                }
            }
        }
    }
}
