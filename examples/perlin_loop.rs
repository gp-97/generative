use generative::canvas::Canvas;
use generative::shape::shape2d::Polygon;
use generative::Transform;
use perlin2d::PerlinNoise2D;

fn setup() -> Canvas {
    let mut canvas = Canvas::new(3840 * 2, 2160 * 2);
    canvas.fill((0, 0, 0, 255));
    canvas
}

fn display(canvas: &mut Canvas) {
    let mut zoff = 0.1;
    let mut tx = 1_f32;
    let mut ty = 1_f32;
    while zoff < 10.0 {
        let noise = PerlinNoise2D::new(4, 3.0, 0.5, 0.75, 10.0, (100.0, 100.0), 0.5, 101);
        let mut points = vec![];
        let mut theta: f64 = 0.0;
        while theta < (std::f64::consts::PI * 2.0) {
            let r = noise.get_noise(f64::cos(theta) - tx as f64, f64::sin(theta) - ty as f64)
                * (canvas.get_height() as f64)
                / 2.0;
            points.push((
                f32::cos(theta as f32) * r as f32 + 100.0 * tx,
                f32::sin(theta as f32) * r as f32 + 100.0 * ty,
            ));
            theta += 0.01;
        }
        let color = (((zoff * 0.0) % 255.0) as u8, 192, 200, 255);
        tx += 0.001;
        ty += 0.001;
        zoff += 0.01;
        let mut poly = Polygon::new(points, color, 1);
        poly.transform(Transform::TRANSLATE(
            canvas.get_height() as f32 / 2.0,
            canvas.get_width() as f32 / 2.0,
        ));
        poly.draw(canvas);
    }
    canvas.save_as_image("examples/outputs/perlin_loop.png");
}

fn main() {
    let mut ctx = setup();
    display(&mut ctx);
}
