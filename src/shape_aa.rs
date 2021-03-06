pub mod shape2d {
    use crate::{canvas, transforms};
    use crate::{Pixel, Point, Transform};
    use std::{mem::swap, vec};
    pub struct Line {
        points: Vec<Point>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        end_points: Vec<Pixel>,
        state: Vec<Pixel>,
    }

    impl Line {
        pub fn new(points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8, canvas: &canvas::Canvas) -> Self {
            let mut end_points: Vec<Pixel> = vec![];
            for point in points.iter() {
                end_points.push(Pixel::new(*point, color));
            }
            let mut line = Self {
                points,
                color,
                thickness,
                end_points,
                state: vec![],
            };
            line.calculate(canvas);
            line
        }

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn get_end_point(&self) -> Vec<Pixel> {
            self.end_points.clone()
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            self.calculate(canvas);
            for pixel in self.state.iter() {
                let point = pixel.get_point();
                let color = pixel.get_color();

                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
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

        fn build_state(&mut self, x: isize, y: isize, c: f32, canvas: &canvas::Canvas) {
            let (r_bkg, g_bkg, b_bkg, _a_bkg) = match canvas.get_pixel_at(x as usize, y as usize) {
                Some(pixel) => pixel.get_color(),
                None => return,
            };

            let r = (self.color.0 as f32 * c + (1.0 - c) * r_bkg as f32) as u8;
            let g = (self.color.1 as f32 * c + (1.0 - c) * g_bkg as f32) as u8;
            let b = (self.color.2 as f32 * c + (1.0 - c) * b_bkg as f32) as u8;
            let a = self.color.3;

            let point = Point::new(x as f32, y as f32);
            let color = (r, g, b, a);
            let pixel = Pixel::new(point, color);
            self.state.push(pixel);
        }

        fn calculate(&mut self, canvas: &canvas::Canvas) {
            if !self.points.is_empty() {
                if self.points.len() == 1 {
                    self.state.push(Pixel::new(self.points[0].clone(), self.color));
                } else {
                    for i in 0..(self.points.len() - 1) {
                        let x1 = self.points[i].get_x();
                        let y1 = self.points[i].get_y();

                        let x2 = self.points[i + 1].get_x();
                        let y2 = self.points[i + 1].get_y();

                        if x1 == x2 {
                            let x1 = x1 as isize;
                            let mut y_start = y1 as isize;
                            let mut y_end = y2 as isize;

                            if y2 < y1 {
                                y_start = y2 as isize;
                                y_end = y1 as isize;
                            }
                            while y_start <= y_end {
                                let point = Point::new(x1 as f32, y_start as f32);
                                let pixel = Pixel::new(point, self.color);
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
                                let point = Point::new(x_start as f32, y1 as f32);
                                let pixel = Pixel::new(point, self.color);
                                self.state.push(pixel);
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

                                self.build_state(xpxl1, ypxl1, self.rfpart(yend) * xgap, canvas);
                                self.build_state(xpxl1, ypxl1 + 1, self.fpart(yend) * xgap, canvas);

                                let mut intery = yend + gradient;
                                xend = self.round(x2) as f32;
                                yend = y2 + gradient * (xend - x2);
                                xgap = self.fpart(x2 + 0.5);

                                let xpxl2 = xend as isize;
                                let ypxl2 = self.ipart(yend);

                                self.build_state(xpxl2, ypxl2, self.rfpart(yend) * xgap, canvas);
                                self.build_state(xpxl2, ypxl2 + 1, self.fpart(yend) * xgap, canvas);

                                let mut x = xpxl1 + 1;
                                while x < xpxl2 {
                                    self.build_state(x, self.ipart(intery), self.rfpart(intery), canvas);
                                    self.build_state(x, self.ipart(intery) + 1, self.fpart(intery), canvas);
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

                                self.build_state(xpxl1, ypxl1, self.rfpart(xend) * ygap, canvas);
                                self.build_state(xpxl1, ypxl1 + 1, self.fpart(xend) * ygap, canvas);

                                let mut interx = xend + gradient;

                                yend = self.round(y2) as f32;
                                xend = x2 + gradient * (yend - y2);
                                ygap = self.fpart(y2 + 0.5);

                                let ypxl2 = yend as isize;
                                let xpxl2 = self.ipart(xend);

                                self.build_state(xpxl2, ypxl2, self.rfpart(xend) * ygap, canvas);
                                self.build_state(xpxl2, ypxl2 + 1, self.fpart(xend) * ygap, canvas);

                                let mut y = ypxl1 + 1;
                                while y < ypxl2 {
                                    self.build_state(self.ipart(interx), y, self.rfpart(interx), canvas);
                                    self.build_state(self.ipart(interx) + 1, y, self.fpart(interx), canvas);
                                    interx += gradient;
                                    y += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        pub fn transform(&mut self, operation: Transform, canvas: &canvas::Canvas) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::translate(point, tx, ty);
                    }
                    for i in 0..self.end_points.len() {
                        let epoint = transforms::translate(&self.end_points[i].get_point(), tx, ty);
                        self.end_points[i].set_point(epoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::rotate(point, &point_pivot, angle);
                    }
                    for i in 0..self.end_points.len() {
                        let epoint = transforms::rotate(&self.end_points[i].get_point(), &point_pivot, angle);
                        self.end_points[i].set_point(epoint);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_x(point, &point_ref, shx);
                    }
                    for i in 0..self.end_points.len() {
                        let epoint = transforms::shear_x(&self.end_points[i].get_point(), &point_ref, shx);
                        self.end_points[i].set_point(epoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for point in self.points.iter_mut() {
                        *point = transforms::shear_y(point, &point_ref, shy);
                    }
                    for i in 0..self.end_points.len() {
                        let epoint = transforms::shear_y(&self.end_points[i].get_point(), &point_ref, shy);
                        self.end_points[i].set_point(epoint);
                    }
                }
            }
            self.state.clear();
            self.calculate(canvas);
        }
    }
    pub struct Rectangle {
        points: [Point; 2],
        vertices: Vec<Pixel>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Rectangle {
        pub fn new(points: [Point; 2], color: (u8, u8, u8, u8), thickness: u8, canvas: &canvas::Canvas) -> Self {
            let x1 = points[0].get_x();
            let y1 = points[0].get_y();
            let x2 = points[1].get_x();
            let y2 = points[1].get_y();
            let vertices = vec![
                Pixel::new(Point::new(x1, y1), color),
                Pixel::new(Point::new(x1, y2), color),
                Pixel::new(Point::new(x2, y2), color),
                Pixel::new(Point::new(x2, y1), color),
            ];
            let mut rect = Self {
                points,
                vertices,
                color,
                thickness,
                state: vec![],
            };
            rect.calculate(canvas);
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

        pub fn set_points(&mut self, points: [Point; 2]) {
            self.points = points;
        }

        pub fn get_points(&self) -> [Point; 2] {
            self.points.clone()
        }

        pub fn get_vertices(&self) -> Vec<Pixel> {
            self.vertices.clone()
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            self.calculate(canvas);
            for pixel in self.state.iter() {
                let point = pixel.get_point();
                let color = pixel.get_color();

                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        fn calculate(&mut self, canvas: &canvas::Canvas) {
            if self.points.len() == 2 {
                let p1 = self.vertices[0].get_point();
                let p2 = self.vertices[2].get_point();
                let p3 = self.vertices[1].get_point();
                let p4 = self.vertices[3].get_point();

                let line = Line::new(vec![p1, p3], self.color, self.thickness, canvas);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                self.vertices[0] = line.get_end_point()[0];
                self.vertices[1] = line.get_end_point()[1];
                let line = Line::new(vec![p3, p2], self.color, self.thickness, canvas);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                let line = Line::new(vec![p2, p4], self.color, self.thickness, canvas);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
                self.vertices[2] = line.get_end_point()[0];
                self.vertices[3] = line.get_end_point()[1];
                let line = Line::new(vec![p4, p1], self.color, self.thickness, canvas);
                for pixel in line.state.iter() {
                    self.state.push(*pixel);
                }
            }
        }
        pub fn transform(&mut self, operation: Transform, canvas: &canvas::Canvas) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::translate(&pixel.get_point(), tx, ty);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::translate(&self.vertices[i].get_point(), tx, ty);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    let mut points = vec![];
                    for pixel in self.vertices.iter_mut() {
                        let point = transforms::rotate(&pixel.get_point(), &point_pivot, angle);
                        points.push(point.clone());
                        pixel.set_point(point);
                    }
                    // let end_points = [points[0], points[2]];
                    // self.set_points(end_points);
                    self.state.clear();

                    let line = Line::new(vec![points[0], points[1]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[1], points[2]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[2], points[3]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[3], points[0]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_x(&self.vertices[i].get_point(), &point_ref, shx);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, pixel.get_color());
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
        vertices: Vec<Pixel>,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Square {
        pub fn new(points: Point, edge: f32, color: (u8, u8, u8, u8), thickness: u8, canvas: &canvas::Canvas) -> Self {
            let x1 = points.get_x();
            let y1 = points.get_y();
            let x2 = x1 + edge;
            let y2 = y1 + edge;
            let vertices = vec![
                Pixel::new(Point::new(x1, y1), color),
                Pixel::new(Point::new(x1, y2), color),
                Pixel::new(Point::new(x2, y2), color),
                Pixel::new(Point::new(x2, y1), color),
            ];
            let mut square = Self {
                points,
                edge,
                vertices,
                color,
                thickness,
                state: vec![],
            };
            square.calculate(canvas);
            square
        }

        pub fn draw(&mut self, canvas: &mut canvas::Canvas) {
            self.calculate(canvas);
            for pixel in self.state.iter() {
                let point = pixel.get_point();
                let color = pixel.get_color();
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

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn set_vertices(&mut self, vertices: Vec<Pixel>) {
            self.vertices = vertices;
        }

        pub fn get_vertices(&self) -> Vec<Pixel> {
            self.vertices.clone()
        }

        fn calculate(&mut self, canvas: &canvas::Canvas) {
            let x1 = self.points.get_x();
            let y1 = self.points.get_y();
            let x2 = x1 + self.edge;
            let y2 = y1 + self.edge;

            let rect = Rectangle::new(
                [Point::new(x1, y1), Point::new(x2, y2)],
                self.color,
                self.thickness,
                canvas,
            );
            for pixel in rect.state.iter() {
                self.state.push(*pixel);
            }
        }
        pub fn transform(&mut self, operation: Transform, canvas: &canvas::Canvas) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::translate(&pixel.get_point(), tx, ty);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::translate(&self.vertices[i].get_point(), tx, ty);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ROTATE(point_pivot, angle) => {
                    let mut points: Vec<Point> = vec![];
                    for pixel in self.vertices.iter_mut() {
                        let new_point = transforms::rotate(&pixel.get_point(), &point_pivot, angle);
                        points.push(new_point);
                        pixel.set_point(new_point);
                    }
                    self.state.clear();

                    let line = Line::new(vec![points[0], points[1]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[1], points[2]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[2], points[3]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                    let line = Line::new(vec![points[3], points[0]], self.color, self.thickness, canvas);
                    for pixel in line.state.iter() {
                        self.state.push(*pixel);
                    }
                }
                Transform::ShearX(point_ref, shx) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_x(&pixel.get_point(), &point_ref, shx);
                        *pixel = Pixel::new(point, pixel.get_color());
                    }
                    for i in 0..self.vertices.len() {
                        let vpoint = transforms::shear_x(&self.vertices[i].get_point(), &point_ref, shx);
                        self.vertices[i].set_point(vpoint);
                    }
                }
                Transform::ShearY(point_ref, shy) => {
                    for pixel in self.state.iter_mut() {
                        let point = transforms::shear_y(&pixel.get_point(), &point_ref, shy);
                        *pixel = Pixel::new(point, pixel.get_color());
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
        pub fn new(points: Vec<Point>, color: (u8, u8, u8, u8), thickness: u8, canvas: &canvas::Canvas) -> Self {
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
            poly.calculate(canvas);
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
            self.calculate(canvas);
            for pixel in self.state.iter() {
                let point = pixel.get_point();
                let color = pixel.get_color();
                let x = point.get_x();
                let y = point.get_y();
                if x >= 0.0 && y >= 0.0 {
                    canvas.set_pixel_at(x as usize, y as usize, color);
                }
            }
        }

        fn calculate(&mut self, canvas: &canvas::Canvas) {
            let p_first = self.points[0];
            self.points.push(p_first);
            let line = Line::new(self.points.clone(), self.color, self.thickness, canvas);
            for point in line.state.iter() {
                self.state.push(*point);
            }
        }
        pub fn transform(&mut self, operation: Transform, canvas: &canvas::Canvas) {
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
            self.calculate(canvas);
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

    pub struct Ellipse {
        center: Point,
        minor_radius: f32,
        major_radius: f32,
        color: (u8, u8, u8, u8),
        thickness: u8,
        state: Vec<Pixel>,
    }

    impl Ellipse {
        pub fn new(
            center: Point,
            minor_radius: f32,
            major_radius: f32,
            color: (u8, u8, u8, u8),
            thickness: u8,
        ) -> Self {
            let mut ellipse = Self {
                center,
                minor_radius,
                major_radius,
                color,
                thickness,
                state: vec![],
            };
            ellipse.calculate();
            ellipse
        }

        pub fn set_center(&mut self, center: Point) {
            self.center = center;
        }

        pub fn set_major_radius(&mut self, major_radius: f32) {
            self.major_radius = major_radius;
        }

        pub fn set_minor_radius(&mut self, minor_radius: f32) {
            self.minor_radius = minor_radius;
        }

        pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
            self.color = color;
        }

        pub fn set_thickness(&mut self, thickness: u8) {
            self.thickness = thickness;
        }

        pub fn get_center(&self) -> Point {
            self.center.clone()
        }

        pub fn get_major_radius(&self) -> f32 {
            self.major_radius
        }

        pub fn get_minor_radius(&self) -> f32 {
            self.minor_radius
        }

        pub fn get_color(&self) -> (u8, u8, u8, u8) {
            self.color
        }

        pub fn get_thickness(&self) -> u8 {
            self.thickness
        }

        pub fn get_state(&self) -> Vec<Pixel> {
            self.state.clone()
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
            let rx = self.minor_radius;
            let ry = self.major_radius;
            let xc = self.center.get_x();
            let yc = self.center.get_y();

            let mut x = 0_f32;
            let mut y = ry;

            let mut dl = rx * ry - rx * rx * ry + 0.25 * rx * rx;
            let mut dx = 2.0 * ry * ry * x;
            let mut dy = 2.0 * rx * rx * y;

            while dx < dy {
                let point = Point::new(x + xc, y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(-x + xc, y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(x + xc, -y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(-x + xc, -y + yc);
                self.state.push(Pixel::new(point, self.color));

                if dl < 0.0 {
                    x += 1.0;
                    dx += 2.0 * ry * ry;
                    dl += dx + (ry * ry);
                } else {
                    x += 1.0;
                    y -= 1.0;
                    dx += 2.0 * ry * ry;
                    dy -= 2.0 * rx * rx;
                    dl += dx - dy + ry * ry;
                }
            }

            let mut d2 = ry * ry * (x + 0.5) * (x + 0.5) + rx * rx * (y - 1.0) * (y - 1.0) - rx * rx * ry * ry;

            while y >= 0.0 {
                let point = Point::new(x + xc, y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(-x + xc, y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(x + xc, -y + yc);
                self.state.push(Pixel::new(point, self.color));
                let point = Point::new(-x + xc, -y + yc);
                self.state.push(Pixel::new(point, self.color));

                if d2 > 0.0 {
                    y -= 1.0;
                    dy -= 2.0 * rx * rx;
                    d2 += rx * rx - dy;
                } else {
                    y -= 1.0;
                    x += 1.0;
                    dx += 2.0 * ry * ry;
                    dy -= 2.0 * rx * rx;
                    d2 += dx - dy + rx * rx;
                }
            }
        }

        pub fn transform(&mut self, operation: Transform) {
            match operation {
                Transform::TRANSLATE(tx, ty) => {
                    self.center = transforms::translate(&self.center, tx, ty);
                    self.state.clear();
                    self.calculate();
                }
                Transform::ROTATE(point_pivot, angle) => {
                    self.center = transforms::rotate(&self.center, &point_pivot, angle);
                    let new_state = self
                        .state
                        .iter_mut()
                        .map(|pixel| {
                            Pixel::new(
                                transforms::rotate(&pixel.get_point(), &point_pivot, angle),
                                pixel.get_color(),
                            )
                        })
                        .collect();
                    self.state = new_state;
                }
                Transform::ShearX(point_ref, shx) => {
                    self.center = transforms::shear_x(&self.center, &point_ref, shx);
                    let new_state = self
                        .state
                        .iter_mut()
                        .map(|pixel| {
                            Pixel::new(
                                transforms::shear_x(&pixel.get_point(), &point_ref, shx),
                                pixel.get_color(),
                            )
                        })
                        .collect();
                    self.state = new_state;
                }
                Transform::ShearY(point_ref, shy) => {
                    self.center = transforms::shear_y(&self.center, &point_ref, shy);
                    let new_state = self
                        .state
                        .iter_mut()
                        .map(|pixel| {
                            Pixel::new(
                                transforms::shear_y(&pixel.get_point(), &point_ref, shy),
                                pixel.get_color(),
                            )
                        })
                        .collect();
                    self.state = new_state;
                }
            }
        }
    }
}

pub mod curve {
    use super::shape2d::Line;
    use crate::{canvas, transforms};
    use crate::{helpers::comb, helpers::linspace};
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
            let mut npoints = npoints;
            if npoints > 10 {
                npoints = 10;
            }
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
            let points = self.state.iter().map(|pixel| pixel.get_point()).collect();
            let mut line = Line::new(points, self.get_color(), self.thickness, canvas);
            line.draw(canvas);
        }
        fn knot_j(&self, knot_i: f32, pi: Point, pj: Point, alpha: f32) -> f32 {
            ((pj.get_x() - pi.get_x()).powf(2.0) + (pj.get_y() - pi.get_y()).powf(2.0)).powf(alpha) + knot_i
        }
        fn catmull_rom_spline(&self, p0: Point, p1: Point, p2: Point, p3: Point) -> Vec<Pixel> {
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
            let mut line = Line::new(points, self.color, self.thickness, canvas);
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
