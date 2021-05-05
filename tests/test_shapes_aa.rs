#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape_aa::shape2d;
use generative::Point;
#[test]
fn test_line_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((255, 255, 255, 255));
    let points = vec![Point::new(27.25482, 29.72583), Point::new(29.72583, 301.25482)];
    let color = (192, 2, 50, 255);
    let mut line = shape2d::Line::new(points, color, 1, &canvas);
    line.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/line_aa.png");
}
#[test]
fn test_rectangle_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = [Point::new(128.0, 128.0), Point::new(384.0, 384.0)];
    let color = (0_u8, 255_u8, 255_u8, 255_u8);
    let mut rect = shape2d::Rectangle::new(points, color, 1, &canvas);
    rect.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/rectangle_aa.png");
}
#[test]
fn test_square_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = Point::new(128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut sqr = shape2d::Square::new(points, 256.0, color, 1, &canvas);
    sqr.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/square_aa.png");
}
#[test]
fn test_polygon_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![
        Point::new(80.0, 240.0),
        Point::new(140.0, 180.0),
        Point::new(100.0, 80.0),
        Point::new(60.0, 80.0),
        Point::new(20.0, 180.0),
    ];
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut penta = shape2d::Polygon::new(points, color, 1, &canvas);
    penta.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/polygon_aa.png");
}
#[test]
fn test_circle_aa() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut circle = shape2d::Circle::new(Point::new(128.0, 128.0), 175.0, color, 1);
    circle.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/circle_aa.png");
}
