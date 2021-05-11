pub mod canvas;
pub mod helpers;
pub mod prelude;
pub mod shape;
pub mod shape_aa;
pub mod transforms;

#[derive(Copy, Clone)]
pub enum Angle {
    DEGREE(f32),
    RADIAN(f32),
}

#[derive(Copy, Clone)]
pub enum Transform {
    TRANSLATE(f32, f32),
    ROTATE(f32, f32, Angle),
    ShearX(f32, f32, f32),
    ShearY(f32, f32, f32),
}

#[derive(Copy, Clone)]
pub enum Spline {
    UNIFORM,
    CENTRIPETAL,
    CHORDAL,
}
