#[cfg(test)]
use generative::canvas::Canvas;
use generative::shape::shape2d;
use generative::Angle;
use generative::Point;
use generative::Transform;
use photon_rs::native::save_image;
#[test]
fn test_line_xy_translation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![Point::new(128.0, 128.0), Point::new(384.0, 384.0)];
    let color = (242_u8, 145_u8, 10_u8, 255_u8);
    let mut line = shape2d::Line::new(points, color, 1);
    line.draw(&mut canvas);
    line.transform(Transform::TRANSLATE(10.0, 100.0));
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_line_translate.png");
}
#[test]
fn test_line_xy_rotation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![Point::new(128.0, 128.0), Point::new(128.0, 256.0)];
    let color = (242_u8, 145_u8, 10_u8, 255_u8);
    let mut line = shape2d::Line::new(points, color, 1);

    let mut deg = 0.0;
    while deg <= 360.0 {
        line.transform(Transform::ROTATE(Point::new(256.0, 256.0), Angle::DEGREE(1.0)));
        line.draw(&mut canvas);
        deg += 1.0;
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_line_rotate.png");
}
#[test]
fn test_line_shear_x() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![Point::new(128.0, 128.0), Point::new(128.0, 256.0)];
    let color = (242_u8, 145_u8, 10_u8, 255_u8);
    let mut line = shape2d::Line::new(points, color, 1);
    line.draw(&mut canvas);
    line.transform(Transform::ShearX(Point::new(0.0, 0.0), 1.2));
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_line_ShearX.png");
}
#[test]
fn test_line_shear_y() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![Point::new(32.0, 256.0), Point::new(128.0, 256.0)];
    let color = (242_u8, 145_u8, 10_u8, 255_u8);
    let mut line = shape2d::Line::new(points, color, 1);
    line.draw(&mut canvas);
    line.transform(Transform::ShearY(Point::new(0.0, 0.0), 1.2));
    line.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_line_ShearY.png");
}
#[test]
fn test_rectangle_translation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = [Point::new(128.0, 128.0), Point::new(384.0, 384.0)];
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut rect = shape2d::Rectangle::new(points, color, 1);
    rect.draw(&mut canvas);
    rect.transform(Transform::TRANSLATE(10.0, 100.0));
    rect.draw(&mut canvas);
    save_image(
        Canvas::to_photon(&canvas),
        "tests/outputs/canvas_rectangle_translate.png",
    );
}
#[test]
fn test_rectangle_rotation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = [Point::new(128.0, 64.0), Point::new(384.0, 448.0)];
    let color = (0_u8, 255_u8, 255_u8, 255_u8);
    let mut rect = shape2d::Rectangle::new(points, color, 1);
    // rect.draw(&mut canvas);

    for i in (0..64).step_by(2) {
        rect.transform(Transform::ROTATE(Point::new(256.0, 256.0), Angle::DEGREE(1.0)));
        rect.set_color((i * 3, 255 - i, 255, 255));
        rect.draw(&mut canvas);
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_rectangle_rotate.png");
}
#[test]
fn test_rectangle_shear_x() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = [Point::new(128.0, 128.0), Point::new(384.0, 384.0)];
    let color = (0_u8, 255_u8, 255_u8, 255_u8);
    let mut rect = shape2d::Rectangle::new(points, color, 1);
    rect.draw(&mut canvas);
    rect.transform(Transform::ShearX(Point::new(256.0, 0.0), 0.25));
    rect.set_color((182, 25, 210, 255));
    rect.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_rectangle_ShearX.png");
}
#[test]
fn test_rectangle_shear_y() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = [Point::new(128.0, 128.0), Point::new(384.0, 384.0)];
    let color = (0_u8, 255_u8, 255_u8, 255_u8);
    let mut rect = shape2d::Rectangle::new(points, color, 1);
    rect.draw(&mut canvas);
    rect.transform(Transform::ShearY(Point::new(256.0, 0.0), 0.25));
    rect.set_color((182, 25, 210, 255));
    rect.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_rectangle_ShearY.png");
}
#[test]
fn test_square_translation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = Point::new(128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut sqr = shape2d::Square::new(points, 256.0, color, 1);
    sqr.draw(&mut canvas);
    sqr.transform(Transform::TRANSLATE(10.0, 100.0));
    sqr.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_square_translate.png");
}
#[test]
fn test_square_rotation() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = Point::new(128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut sqr = shape2d::Square::new(points, 256.0, color, 1);

    for i in (0..64).step_by(2) {
        sqr.transform(Transform::ROTATE(Point::new(256.0, 256.0), Angle::DEGREE(1.0)));
        sqr.set_color((i * 3, 255 - i, 255, 255));
        sqr.draw(&mut canvas);
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_square_rotate.png");
}
#[test]
fn test_square_shear_x() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = Point::new(128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut sqr = shape2d::Square::new(points, 256.0, color, 1);
    sqr.draw(&mut canvas);
    sqr.transform(Transform::ShearX(Point::new(256.0, 0.0), 0.25));
    sqr.set_color((182, 25, 210, 255));
    sqr.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_square_ShearX.png");
}
#[test]
fn test_square_shear_y() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = Point::new(128.0, 128.0);
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut sqr = shape2d::Square::new(points, 256.0, color, 1);
    sqr.draw(&mut canvas);
    sqr.transform(Transform::ShearY(Point::new(256.0, 0.0), 0.25));
    sqr.set_color((18, 125, 210, 255));
    sqr.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_square_ShearY.png");
}
#[test]
fn test_polygon_translation() {
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
    let mut penta = shape2d::Polygon::new(points, color, 1);
    penta.draw(&mut canvas);
    penta.set_color((182, 225, 21, 255));
    penta.transform(Transform::TRANSLATE(128.0, 128.0));
    penta.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_polygon_translate.png");
}
#[test]
fn test_polygon_rotation() {
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
    let mut penta = shape2d::Polygon::new(points, color, 1);
    penta.set_color((182, 225, 21, 255));
    penta.transform(Transform::TRANSLATE(384.0, 128.0));
    for i in 0..64 {
        penta.transform(Transform::ROTATE(Point::new(256.0, 0.0), Angle::DEGREE(1.0)));
        penta.set_color((255, i * 3, 0, 255));
        penta.draw(&mut canvas);
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_polygon_rotate.png");
}
#[test]
fn test_polygon_shear_x() {
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
    let mut penta = shape2d::Polygon::new(points, color, 1);
    penta.set_color((182, 225, 21, 255));
    penta.transform(Transform::TRANSLATE(128.0, 128.0));
    penta.draw(&mut canvas);
    penta.transform(Transform::ShearX(Point::new(256.0, 256.0), 1.2));
    penta.set_color((182, 56, 210, 255));
    penta.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_polygon_shearX.png");
}
#[test]
fn test_polygon_shear_y() {
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
    let mut penta = shape2d::Polygon::new(points, color, 1);
    penta.set_color((182, 225, 21, 255));
    penta.transform(Transform::TRANSLATE(128.0, 128.0));
    penta.draw(&mut canvas);
    penta.transform(Transform::ShearY(Point::new(256.0, 256.0), 1.2));
    penta.set_color((182, 56, 210, 255));
    penta.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_polygon_shearY.png");
}
#[test]
fn test_circle_translate() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut circle = shape2d::Circle::new(Point::new(128.0, 128.0), 175.0, color, 1);
    circle.draw(&mut canvas);
    circle.transform(Transform::TRANSLATE(128.0, 128.0));
    circle.set_color((12, 255, 10, 255));
    circle.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_circle_translate.png");
}
#[test]
fn test_circle_rotate() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut circle = shape2d::Circle::new(Point::new(128.0, 128.0), 175.0, color, 1);
    circle.transform(Transform::TRANSLATE(128.0, 128.0));
    circle.draw(&mut canvas);
    for i in (0..64).step_by(4) {
        circle.transform(Transform::ROTATE(Point::new(0.0, 0.0), Angle::DEGREE(1.0)));
        circle.set_color((255, 0, i * 3, 255));
        circle.draw(&mut canvas);
    }
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_circle_rotate.png");
}

#[test]
fn test_circle_shear_x() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut circle = shape2d::Circle::new(Point::new(128.0, 128.0), 175.0, color, 1);
    circle.transform(Transform::TRANSLATE(128.0, 128.0));
    circle.draw(&mut canvas);
    circle.transform(Transform::ShearX(Point::new(0.0, 0.0), 0.5));
    circle.set_color((12, 185, 10, 255));
    circle.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_circle_shearX.png");
}

#[test]
fn test_circle_shear_y() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242_u8, 45_u8, 210_u8, 255_u8);
    let mut circle = shape2d::Circle::new(Point::new(128.0, 128.0), 175.0, color, 1);
    circle.transform(Transform::TRANSLATE(128.0, 128.0));
    circle.draw(&mut canvas);
    circle.transform(Transform::ShearY(Point::new(0.0, 0.0), 0.5));
    circle.set_color((12, 185, 10, 255));
    circle.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "tests/outputs/canvas_circle_shearY.png");
}

#[test]
fn test_ellipse_init() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242, 45, 210, 255);
    let mut ellipse = shape2d::Ellipse::new(Point::new(256.0, 256.0), 64.0, 128.0, color, 1);
    ellipse.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/ellipse_init.png");
}

#[test]
fn test_ellipse_translate() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242, 45, 210, 255);
    let mut ellipse = shape2d::Ellipse::new(Point::new(256.0, 256.0), 64.0, 128.0, color, 1);
    ellipse.draw(&mut canvas);
    ellipse.transform(Transform::TRANSLATE(10.0, 10.0));
    ellipse.set_color((10, 180, 200, 255));
    ellipse.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/ellipse_translate.png");
}

#[test]
fn test_ellipse_rotate() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242, 45, 210, 255);
    let mut ellipse = shape2d::Ellipse::new(Point::new(256.0, 256.0), 64.0, 128.0, color, 1);
    ellipse.draw(&mut canvas);
    ellipse.transform(Transform::ROTATE(Point::new(256.0, 256.0), Angle::DEGREE(90.0)));
    ellipse.set_color((10, 180, 200, 255));
    ellipse.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/ellipse_rotate.png");
}

#[test]
fn test_ellipse_shearx() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242, 45, 210, 255);
    let mut ellipse = shape2d::Ellipse::new(Point::new(256.0, 256.0), 64.0, 128.0, color, 1);
    ellipse.draw(&mut canvas);
    ellipse.transform(Transform::ShearX(Point::new(256.0, 256.0), 0.25));
    ellipse.set_color((10, 180, 200, 255));
    ellipse.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/ellipse_shearx.png");
}

#[test]
fn test_ellipse_sheary() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let color = (242, 45, 210, 255);
    let mut ellipse = shape2d::Ellipse::new(Point::new(256.0, 256.0), 64.0, 128.0, color, 1);
    ellipse.draw(&mut canvas);
    ellipse.transform(Transform::ShearY(Point::new(256.0, 256.0), 1.0));
    ellipse.set_color((10, 180, 200, 255));
    ellipse.draw(&mut canvas);
    canvas.save_as_image("tests/outputs/ellipse_sheary.png");
}
