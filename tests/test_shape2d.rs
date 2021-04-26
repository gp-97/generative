#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape::shape2d;
use photon_rs::native::save_image;

#[test]
fn test_line_x() {
    let mut canvas = Canvas::new(512, 512);
    let line = shape2d::Line::new(vec![(100.0, 213.0), (100.0, 342.0)], (180_u8, 56_u8, 210_u8, 255_u8), 1);
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_line_same_X.png");
}
#[test]
fn test_line_y() {
    let mut canvas = Canvas::new(512, 512);
    let line = shape2d::Line::new(vec![(100.0, 213.0), (200.0, 213.0)], (180_u8, 56_u8, 210_u8, 255_u8), 1);
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_line_same_Y.png");
}
#[test]
fn test_line_xy() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255_u8));
    let line = shape2d::Line::new(vec![(100.0, 255.0), (200.0, 511.0)], (180_u8, 56_u8, 210_u8, 255_u8), 1);
    line.draw(&mut canvas);

    let img = Canvas::to_photon(&canvas);
    save_image(img, "assets/canvas_line_XY.png");
}
#[test]
fn test_rectangle() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255_u8));
    let rect = shape2d::Rectangle::new([(32.0, 32.0), (480.0, 480.0)], (140, 30, 255, 255), 2);
    rect.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_rectangle.png");
}

#[test]
fn test_square() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255_u8));
    let sqr = shape2d::Square::new((50.0, 128.0), 256.0, (140, 30, 255, 255), 2);
    sqr.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_square.png");
}

#[test]
fn test_quad() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let p: Vec<(f32, f32)> = vec![(200.0, 33.5), (19.0, 324.0), (443.0, 229.0), (335.0, 267.0)];
    let quad = shape2d::Quad::new(p, (182, 56, 10, 255), 2);
    quad.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_quad.png");
}

#[test]
fn test_line_from_segments() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let line = shape2d::Line::new(
        vec![(200.0, 33.5), (19.0, 324.0), (443.0, 229.0), (335.0, 267.0)],
        (180_u8, 156_u8, 10_u8, 255_u8),
        1,
    );
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_from_segments.png");
}
#[test]
fn test_circle() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let circle = shape2d::Circle::new((256.0, 256.0), 180.0, (255, 144, 31, 128), 2);
    circle.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "assets/canvas_circle.png");
}
