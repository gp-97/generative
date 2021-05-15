use generative::prelude::*;

fn setup() -> Canvas {
    let canvas = Canvas::new(1920, 1080);
    canvas
}

fn display(canvas: &mut Canvas) {
    let amp = 10.0;
    let perlin = PerlinNoise2D::new(1, amp, 0.75, 0.75, 10.0, (100.0, 100.0), 0.5, 101);
    let width = canvas.get_width() as usize;
    let height = canvas.get_height() as usize;
    let step = 24;

    for row in (0..height).step_by(step) {
        for col in (0..width).step_by(step) {
            let noise = perlin.get_noise(row as f64, col as f64) as f32;
            let angle = 2.0 * std::f32::consts::PI * norm(noise, 0.0, amp as f32);
            let x = step as f32 * angle.cos() + row as f32;
            let y = step as f32 * angle.sin() + col as f32;
            // let points = vec![Point::new(row as f32, col as f32), Point::new(x, y)];
            // let mut line = Line_aa::new(points, (0, 0, 0, 255), 1, canvas);
            // line.draw(canvas);
            for i in 0..5 {
                let mut circle = Circle::new(Point::new(x, y), i as f32, (0, 0, 0, 255), 1);
                circle.draw(canvas);
            }
            // canvas.set_pixel_at(x as usize, y as usize, (0, 0, 0, 255));
        }
    }
}

fn main() {
    let mut canvas = setup();
    display(&mut canvas);
    canvas.save_as_image("examples/outputs/perlin_force_field.png");
}
