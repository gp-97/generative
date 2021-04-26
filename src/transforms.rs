use crate::canvas::Canvas;
use crate::Angle;
pub fn translate(canvas: &Canvas, x_old: f32, y_old: f32, tx: f32, ty: f32) -> (f32, f32) {
    if (x_old + tx) >= 0.0 && (y_old + ty) >= 0.0 {
        (x_old + tx, y_old + ty)
    } else {
        (canvas.get_height() as f32, canvas.get_width() as f32)
    }
}

pub fn rotate(canvas: &Canvas, x_old: f32, y_old: f32, x_pivot: f32, y_pivot: f32, angle: Angle) -> (f32, f32) {
    match angle {
        Angle::DEGREE(deg) => {
            let rad = (std::f32::consts::PI / 180.0) * deg;
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            if (x_old + x_pivot) >= 0.0 && (y_new + y_pivot) >= 0.0 {
                (x_new + x_pivot, y_new + y_pivot)
            } else {
                (canvas.get_height() as f32, canvas.get_width() as f32)
            }
        }
        Angle::RADIAN(rad) => {
            let x_old = x_old - x_pivot;
            let y_old = y_old - y_pivot;
            let x_new = x_old * f32::cos(rad) - y_old * f32::sin(rad);
            let y_new = x_old * f32::sin(rad) + y_old * f32::cos(rad);

            if (x_old + x_pivot) >= 0.0 && (y_new + y_pivot) >= 0.0 {
                (x_new + x_pivot, y_new + y_pivot)
            } else {
                (canvas.get_height() as f32, canvas.get_width() as f32)
            }
        }
    }
}

pub fn shear_x(canvas: &Canvas, x_old: f32, y_old: f32, shx: f32) -> (f32, f32) {
    if (x_old + y_old * shx) >= 0.0 && y_old >= 0.0 {
        (x_old + y_old * shx, y_old)
    } else {
        (canvas.get_height() as f32, canvas.get_width() as f32)
    }
}
pub fn shear_y(canvas: &Canvas, x_old: f32, y_old: f32, shy: f32) -> (f32, f32) {
    if (y_old + x_old * shy) >= 0.0 && x_old >= 0.0 {
        (x_old, y_old + x_old * shy)
    } else {
        (canvas.get_height() as f32, canvas.get_width() as f32)
    }
}
