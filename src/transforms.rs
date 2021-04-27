use crate::Angle;

pub fn translate(x_old: f32, y_old: f32, tx: f32, ty: f32) -> (f32, f32) {
    (x_old + tx, y_old + ty)
}

pub fn rotate(x_old: f32, y_old: f32, x_pivot: f32, y_pivot: f32, angle: Angle) -> (f32, f32) {
    match angle {
        Angle::DEGREE(deg) => {
            let rad = (std::f32::consts::PI / 180.0) * deg;
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            (x_new + x_pivot, y_new + y_pivot)
        }
        Angle::RADIAN(rad) => {
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            (x_new + x_pivot, y_new + y_pivot)
        }
    }
}

pub fn shear_x(x_old: f32, y_old: f32, x_ref: f32, y_ref: f32, shx: f32) -> (f32, f32) {
    let x_old = x_old - x_ref;
    let y_old = y_old - y_ref;

    let x_new = x_old + y_old * shx + x_ref;
    let y_new = y_old + y_ref;
    (x_new, y_new)
}
pub fn shear_y(x_old: f32, y_old: f32, x_ref: f32, y_ref: f32, shy: f32) -> (f32, f32) {
    let x_old = x_old - x_ref;
    let y_old = y_old - y_ref;

    let x_new = x_old + x_ref;
    let y_new = y_old + x_old * shy + y_ref;

    (x_new, y_new)
}
