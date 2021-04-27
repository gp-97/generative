use generative::canvas::Canvas;
use generative::shape::shape2d::Circle;
use generative::Transform;
use photon_rs::native::save_image;

fn main() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let mut circle = Circle::new((128.0, 128.0), 200.0, (240, 45, 98, 255), 1);
    circle.draw(&mut canvas);
    circle.transform(Transform::TRANSLATE(128.0, 128.0));
    circle.draw(&mut canvas);
    save_image(Canvas::to_photon(&canvas), "examples/outputs/circle_generation.png");
}
