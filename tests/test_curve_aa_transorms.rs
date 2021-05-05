#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape_aa::curve::{Bezier, Curve};
use generative::{Angle, Point, Spline, Transform};

#[test]
fn test_curve_translate() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let spline = Spline::CENTRIPETAL;
    let points = vec![
        Point::new(0.0, 150.0),
        Point::new(200.0, 200.0),
        Point::new(300.0, 100.0),
        Point::new(400.0, 50.0),
        Point::new(500.0, 100.0),
        Point::new(60.0, 200.0),
    ];
    let mut curve = Curve::new(points, (255, 102, 0, 255), 1, 1000, spline);
    curve.draw(&mut canvas);
    curve.set_color((180, 2, 20, 255));
    curve.transform(Transform::TRANSLATE(-20.0, 350.0));
    curve.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/curve_aa_translate.png");
}
#[test]
fn test_curve_rotate() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let spline = Spline::CENTRIPETAL;
    let points = vec![
        Point::new(0.0, 150.0),
        Point::new(200.0, 200.0),
        Point::new(300.0, 100.0),
        Point::new(400.0, 50.0),
        Point::new(500.0, 100.0),
        Point::new(60.0, 200.0),
    ];
    let mut curve = Curve::new(points, (255, 102, 0, 255), 1, 1000, spline);
    curve.draw(&mut canvas);
    curve.set_color((180, 2, 20, 255));
    curve.transform(Transform::ROTATE(Point::new(200.0, 200.0), Angle::DEGREE(90.0)));
    curve.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/curve_aa_rotate.png");
}
#[test]
fn test_curve_shear_x() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let spline = Spline::CENTRIPETAL;
    let points = vec![
        Point::new(0.0, 150.0),
        Point::new(200.0, 200.0),
        Point::new(300.0, 100.0),
        Point::new(400.0, 50.0),
        Point::new(500.0, 100.0),
        Point::new(60.0, 200.0),
    ];
    let mut curve = Curve::new(points, (255, 102, 0, 255), 1, 1000, spline);
    curve.draw(&mut canvas);
    curve.set_color((180, 2, 20, 255));
    curve.transform(Transform::ShearX(Point::new(200.0, 200.0), 2.0));
    curve.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/curve_aa_shear_x.png");
}
#[test]
fn test_curve_shear_y() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let spline = Spline::CENTRIPETAL;
    let points = vec![
        Point::new(0.0, 150.0),
        Point::new(200.0, 200.0),
        Point::new(300.0, 100.0),
        Point::new(400.0, 50.0),
        Point::new(500.0, 100.0),
        Point::new(60.0, 200.0),
    ];
    let mut curve = Curve::new(points, (255, 102, 0, 255), 1, 1000, spline);
    curve.draw(&mut canvas);
    curve.set_color((180, 2, 20, 255));
    curve.transform(Transform::ShearY(Point::new(200.0, 200.0), 2.0));
    curve.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/curve_aa_shear_y.png");
}
#[test]
fn test_bezier_translate() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 511.0),
        Point::new(511.0, 0.0),
        Point::new(511.0, 511.0),
    ];
    let mut bz = Bezier::new(100, points, (255, 102, 0, 255), 1);
    bz.draw(&mut canvas);
    bz.set_color((180, 2, 50, 255));
    bz.transform(Transform::TRANSLATE(100.0, 100.0));
    bz.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/bezier_aa_translate.png");
}
#[test]
fn test_bezier_rotate() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 511.0),
        Point::new(511.0, 0.0),
        Point::new(511.0, 511.0),
    ];
    let mut bz = Bezier::new(100, points, (255, 102, 0, 255), 1);
    bz.draw(&mut canvas);
    bz.set_color((180, 2, 50, 255));
    bz.transform(Transform::ROTATE(Point::new(256.0, 256.0), Angle::DEGREE(-45.0)));
    bz.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/bezier_aa_rotate.png");
}
#[test]
fn test_bezier_shear_x() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 511.0),
        Point::new(511.0, 0.0),
        Point::new(511.0, 511.0),
    ];
    let mut bz = Bezier::new(100, points, (255, 102, 0, 255), 1);
    bz.draw(&mut canvas);
    bz.set_color((180, 2, 50, 255));
    bz.transform(Transform::ShearX(Point::new(256.0, 256.0), 2.0));
    bz.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/bezier_aa_shear_x.png");
}
#[test]
fn test_bezier_shear_y() {
    let mut canvas = Canvas::new(700, 700);
    canvas.fill((128, 128, 128, 255));
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 511.0),
        Point::new(511.0, 0.0),
        Point::new(511.0, 511.0),
    ];
    let mut bz = Bezier::new(100, points, (255, 102, 0, 255), 1);
    bz.draw(&mut canvas);
    bz.set_color((180, 2, 50, 255));
    bz.transform(Transform::ShearY(Point::new(256.0, 256.0), 2.0));
    bz.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/bezier_aa_shear_y.png");
}
