use generative::canvas::Canvas;
use generative::shape::curve::Bezier;

fn main() {
    let mut canvas = Canvas::new(512, 512);
    canvas.fill((0, 0, 0, 255));
    let points = vec![(0.0, 0.0), (0.0, 511.0), (511.0, 0.0), (511.0, 511.0)];
    let bz = Bezier::new(100, points, (180, 2, 50, 255), 1);
    bz.draw(&mut canvas);
    canvas.save_as_image("examples/outputs/bezier_init.png");
}
