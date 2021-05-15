#[cfg(test)]
use generative::prelude::*;
#[test]
fn test_thick_line() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![Point::new(128.0, 384.0), Point::new(384.0, 128.0)];
    let color = (242_u8, 145_u8, 10_u8, 255_u8);
    let mut line = Line::new(points, color, 1);
    line.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/line_thick.png");
}
