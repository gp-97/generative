#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape::shape2d;
use photon_rs::native::save_image;

#[test]
fn test_line_x() {
    let mut canvas = Canvas::new(512, 512);
    shape2d::line(&mut canvas, 100.0, 213.0, 100.0, 342.0, (180_u8, 56_u8, 210_u8, 255_u8));
    save_image(Canvas::to_photon(&canvas), "assets/canvas_line_same_X.png");
}
#[test]
fn test_line_y() {
    let mut canvas = Canvas::new(512, 512);
    shape2d::line(&mut canvas, 213.0, 100.0, 342.0, 100.0, (180_u8, 56_u8, 210_u8, 255_u8));
    save_image(Canvas::to_photon(&canvas), "assets/canvas_line_same_Y.png");
}
#[test]
fn test_line_xy() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255_u8));
    shape2d::line(&mut canvas, 60.0, 80.0, 400.0, 400.0, (180_u8, 56_u8, 210_u8, 255_u8));

    let img = Canvas::to_photon(&canvas);
    save_image(img, "assets/canvas_line_XY.png");
}
#[test]
fn test_line_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255_u8));
    shape2d::line_aa(&mut canvas, 100.0, 80.0, 50.0, 200.0, (180_u8, 56_u8, 210_u8, 255_u8));
    save_image(Canvas::to_photon(&canvas), "assets/canvas_lineAA_XY.png");
}
#[test]
fn test_rectangle() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 0_u8));

    // shape2d::rectangle(&mut canvas, 32.0, 32.0, 480.0, 480.0, (r, g, b, a));

    // shape2d::rectangle(&mut canvas, 32.0, 32.0, 100.0, 480.0, (r - 255, g + 255, b, a));
    // shape2d::rectangle(&mut canvas, 32.0, 32.0, 480.0, 100.0, (r - 255, g, b + 255, a));
    // shape2d::rectangle(&mut canvas, 412.0, 32.0, 480.0, 480.0, (r, g, b, a));
    // shape2d::rectangle(&mut canvas, 256.0, 412.0, 480.0, 480.0, (r - 128, g + 128, b, a));

    for i in (-15..16).step_by(1) {
        shape2d::rectangle(
            &mut canvas,
            32.0 + i as f32,
            32.0 + i as f32,
            100.0 + i as f32,
            480.0 + i as f32,
            (255, 211, 25, 255),
        );
        shape2d::rectangle(
            &mut canvas,
            32.0 + i as f32,
            32.0 + i as f32,
            480.0 + i as f32,
            100.0 + i as f32,
            (255, 144, 31, 255),
        );
        shape2d::rectangle(
            &mut canvas,
            412.0 + i as f32,
            32.0 + i as f32,
            480.0 + i as f32,
            480.0 + i as f32,
            (255, 41, 117, 255),
        );
        shape2d::rectangle(
            &mut canvas,
            256.0 + i as f32,
            412.0 + i as f32,
            480.0 + i as f32,
            480.0 + i as f32,
            (140, 30, 255, 255),
        );
    }
    save_image(Canvas::to_photon(&canvas), "assets/canvas_rectangle.png");
}
