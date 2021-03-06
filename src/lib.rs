pub mod canvas;
pub mod helpers;
pub mod prelude;
pub mod shape;
pub mod shape_aa;
pub mod transforms;

use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum Angle {
    DEGREE(f32),
    RADIAN(f32),
}

impl Angle {
    pub fn get_angle_rad(angle: Angle) -> f32 {
        match angle {
            Angle::DEGREE(ang) => (std::f32::consts::PI / 180.0) * ang,
            Angle::RADIAN(ang) => ang,
        }
    }
    pub fn get_angle_deg(angle: Angle) -> f32 {
        match angle {
            Angle::DEGREE(ang) => ang,
            Angle::RADIAN(ang) => (180.0 / std::f32::consts::PI) * ang,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Transform {
    TRANSLATE(f32, f32),
    ROTATE(Point, Angle),
    ShearX(Point, f32),
    ShearY(Point, f32),
}

#[derive(Copy, Clone)]
pub enum Spline {
    UNIFORM,
    CENTRIPETAL,
    CHORDAL,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn get_unit_vec(&self) -> Point {
        let x = self.x / helpers::euclid_dist(&self, &Point::new(0.0, 0.0));
        let y = self.y / helpers::euclid_dist(&self, &Point::new(0.0, 0.0));
        Self { x, y }
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn euclid_dist_square(&self, other: &Self) -> f32 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)
    }
}

impl From<(f32, f32)> for Point {
    fn from(point_tuple: (f32, f32)) -> Point {
        Point {
            x: point_tuple.0,
            y: point_tuple.1,
        }
    }
}

impl From<Vec<f32>> for Point {
    fn from(point_vec: Vec<f32>) -> Point {
        Point {
            x: point_vec[0],
            y: point_vec[1],
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pixel {
    point: Point,
    color: (u8, u8, u8, u8),
}

impl Pixel {
    pub fn new(point: Point, color: (u8, u8, u8, u8)) -> Self {
        Self { point, color }
    }
    pub fn set_point(&mut self, point: Point) {
        self.point = point;
    }
    pub fn set_color(&mut self, color: (u8, u8, u8, u8)) {
        self.color = color;
    }
    pub fn get_point(&self) -> Point {
        self.point.clone()
    }
    pub fn get_color(&self) -> (u8, u8, u8, u8) {
        self.color
    }
    pub fn is_within_canvas(self, canvas: &canvas::Canvas) -> bool {
        let x = self.point.x as usize;
        let y = self.point.y as usize;

        x < canvas.get_height() as usize && y < canvas.get_width() as usize
    }
}
