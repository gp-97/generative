#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape::curve::Curve;
use generative::Spline;
#[test]
fn test_curve_init() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let spline = Spline::CENTRIPETAL;
    let points = vec![
        (0.0, 150.0),
        (200.0, 200.0),
        (300.0, 100.0),
        (400.0, 50.0),
        (500.0, 100.0),
        (60.0, 200.0),
    ];
    let curve = Curve::new(points, (255, 102, 0, 255), 1, 1000, spline);
    curve.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/curve_init.png");
}
