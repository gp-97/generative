#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape_aa::shape2d;
#[test]
fn test_line_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![(300.0, 0.0), (311.0, 311.0)];
    let color = (192, 2, 50, 255);
    let line = shape2d::Line::new(points, color, 1);
    line.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/line_aa.png");
}
#[test]
fn test_rectangle_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points: [(f32, f32); 2] = [(128.0, 128.0), (384.0, 384.0)];
    let color = (0_u8, 255_u8, 255_u8, 255_u8);
    let rect = shape2d::Rectangle::new(points, color, 1);
    rect.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/rectangle_aa.png");
}
#[test]
fn test_square_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points: (f32, f32) = (128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let sqr = shape2d::Square::new(points, 256.0, color, 1);
    sqr.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/square_aa.png");
}
#[test]
fn test_polygon_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![
        (80.0, 240.0),
        (140.0, 180.0),
        (100.0, 80.0),
        (60.0, 80.0),
        (20.0, 180.0),
    ];
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let penta = shape2d::Polygon::new(points, color, 1);
    penta.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/polygon_aa.png");
}
#[test]
fn test_circle_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let circle = shape2d::Circle::new((128.0, 128.0), 175.0, color, 1);
    circle.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/circle_aa.png");
}
