use crate::Angle;
use crate::Point;

pub fn translate(point_old: &Point, tx: f32, ty: f32) -> Point {
    let x_old = point_old.get_x();
    let y_old = point_old.get_y();
    Point::from((x_old + tx, y_old + ty))
}

pub fn rotate(point_old: &Point, point_pivot: &Point, angle: Angle) -> Point {
    match angle {
        Angle::DEGREE(deg) => {
            let rad = (std::f32::consts::PI / 180.0) * deg;
            let x_old = point_old.get_x();
            let y_old = point_old.get_y();
            let x_pivot = point_pivot.get_x();
            let y_pivot = point_pivot.get_y();
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            Point::from((x_new + x_pivot, y_new + y_pivot))
        }
        Angle::RADIAN(rad) => {
            let x_old = point_old.get_x();
            let y_old = point_old.get_y();
            let x_pivot = point_pivot.get_x();
            let y_pivot = point_pivot.get_y();
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            Point::from((x_new + x_pivot, y_new + y_pivot))
        }
    }
}

pub fn shear_x(point_old: &Point, point_ref: &Point, shx: f32) -> Point {
    let x_old = point_old.get_x();
    let y_old = point_old.get_y();
    let x_ref = point_ref.get_x();
    let y_ref = point_ref.get_y();

    let x_old = x_old - x_ref;
    let y_old = y_old - y_ref;

    let x_new = x_old + y_old * shx + x_ref;
    let y_new = y_old + y_ref;
    Point::from((x_new, y_new))
}
pub fn shear_y(point_old: &Point, point_ref: &Point, shy: f32) -> Point {
    let x_old = point_old.get_x();
    let y_old = point_old.get_y();
    let x_ref = point_ref.get_x();
    let y_ref = point_ref.get_y();

    let x_old = x_old - x_ref;
    let y_old = y_old - y_ref;

    let x_new = x_old + x_ref;
    let y_new = y_old + x_old * shy + y_ref;

    Point::from((x_new, y_new))
}
