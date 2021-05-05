#[cfg(test)]
use generative::canvas::Canvas;
use generative::transforms;
use generative::{Angle, Point};
use photon_rs::native::save_image;
#[test]
fn test_translation() {
    let mut canvas = Canvas::new(10, 10);
    let x_pxl = 1_f32;
    let y_pxl = 1_f32;
    let point = transforms::translate(&Point::new(x_pxl, y_pxl), 5.0, 5.0);
    canvas.set_pixel_at(point.get_x() as usize, point.get_y() as usize, (180, 56, 210, 255));
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_translate_point.png");
}

#[test]
fn test_rotation() {
    let mut canvas = Canvas::new(1024, 1024);
    canvas.fill((0, 0, 0, 255));
    let x_pxl = 411_f32;
    let y_pxl = 411_f32;
    canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 56, 210, 255));

    let mut i = 0.0;
    while i <= 360.0 {
        let point = transforms::rotate(
            &Point::new(x_pxl, y_pxl),
            &Point::new(511.0, 511.0),
            Angle::DEGREE(-1.0 * i as f32),
        );
        let x_pxl = point.get_x();
        let y_pxl = point.get_y();
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_rotate_point.png");
}
#[test]
fn test_shear_x() {
    let mut canvas = Canvas::new(1024, 1024);
    let x_pxl = 411_f32;
    let y_pxl = 411_f32;
    let mut i = 0.0;
    while i <= 360.0 {
        let point = transforms::rotate(
            &Point::new(x_pxl, y_pxl),
            &Point::new(311.0, 511.0),
            Angle::DEGREE(-1.0 * i as f32),
        );
        let x_pxl = point.get_x();
        let y_pxl = point.get_y();
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        let point = transforms::shear_x(&Point::new(x_pxl, y_pxl), &Point::new(0.0, 0.0), 0.85);
        let x_pxl = point.get_x();
        let y_pxl = point.get_y();
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 156, 10, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_ShearX.png");
}
#[test]
fn test_shear_y() {
    let mut canvas = Canvas::new(1024, 1024);
    let x_pxl = 411_f32;
    let y_pxl = 411_f32;
    let mut i = 0.0;
    while i <= 360.0 {
        let point = transforms::rotate(
            &Point::new(x_pxl, y_pxl),
            &Point::new(511.0, 311.0),
            Angle::DEGREE(-1.0 * i as f32),
        );
        let x_pxl = point.get_x();
        let y_pxl = point.get_y();
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        let point = transforms::shear_y(&Point::new(x_pxl, y_pxl), &Point::new(511.0, 511.0), 0.85);
        let x_pxl = point.get_x();
        let y_pxl = point.get_y();
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 156, 10, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_ShearY.png");
}
