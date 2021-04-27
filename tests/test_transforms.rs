#[cfg(test)]
use generative::canvas::Canvas;
use generative::transforms;
use generative::Angle;
use photon_rs::native::save_image;
#[test]
fn test_translation() {
    let mut canvas = Canvas::new(10, 10);
    let x_pxl = 1_f32;
    let y_pxl = 1_f32;
    let (x_pxl, y_pxl) = transforms::translate(x_pxl, y_pxl, 5.0, 5.0);
    canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 56, 210, 255));
    save_image(Canvas::to_photon(&canvas), "assets/canvas_translate_point.png");
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
        let (x_pxl, y_pxl) = transforms::rotate(x_pxl, y_pxl, 511.0, 511.0, Angle::DEGREE(-1.0 * i as f32));
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "assets/canvas_rotate_point.png");
}
#[test]
fn test_shear_x() {
    let mut canvas = Canvas::new(1024, 1024);
    let x_pxl = 411_f32;
    let y_pxl = 411_f32;
    let mut i = 0.0;
    while i <= 360.0 {
        let (x_pxl, y_pxl) = transforms::rotate(x_pxl, y_pxl, 311.0, 511.0, Angle::DEGREE(-1.0 * i as f32));
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        let (x_pxl, y_pxl) = transforms::shear_x(x_pxl, y_pxl, 0.0, 0.0, 0.85);
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 156, 10, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "assets/canvas_ShearX.png");
}
#[test]
fn test_shear_y() {
    let mut canvas = Canvas::new(1024, 1024);
    let x_pxl = 411_f32;
    let y_pxl = 411_f32;
    let mut i = 0.0;
    while i <= 360.0 {
        let (x_pxl, y_pxl) = transforms::rotate(x_pxl, y_pxl, 511.0, 311.0, Angle::DEGREE(-1.0 * i as f32));
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (10, 156, 210, 255));
        let (x_pxl, y_pxl) = transforms::shear_y(x_pxl, y_pxl, 511.0, 511.0, 0.85);
        canvas.set_pixel_at(x_pxl as usize, y_pxl as usize, (180, 156, 10, 255));
        i += 0.1;
    }
    save_image(Canvas::to_photon(&canvas), "assets/canvas_ShearY.png");
}
