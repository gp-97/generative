use generative::prelude::*;

fn setup() -> Canvas {
    let mut canvas = Canvas::image_as_canvas("examples/inputs/animal.jpg");
    canvas.fill((255, 255, 255, 255));
    canvas
}
fn display(canvas: &mut Canvas, amplitude: f32) {
    let img = Canvas::image_as_canvas("examples/inputs/animal.jpg");
    let width = canvas.get_width();
    let height = canvas.get_height();
    let mut y = 0_f32;
    while y < (height as f32) {
        let mut l = 0_f32;
        let mut x = 0_f32;
        let mut points = vec![];
        while x < (height as f32) {
            let xx = x;
            let row = xx as usize;
            let col = map(
                y * 0.02 * height as f32,
                0.0,
                height as f32,
                width as f32,
                (width as f32 - height as f32).abs(),
            ) as usize;
            let color = match img.get_pixel_at(row, col) {
                Some(pixel) => pixel.get_color(),
                None => continue,
            };
            l += (255.0 - color.0 as f32) / (255.0);
            let m = (255.0 - color.2 as f32) / 255.0;
            let col = (amplitude * dec_l(m) * f32::sin(l * 0.5 * std::f32::consts::PI)
                + map(
                    (y + 0.5) * 0.02 * height as f32,
                    0.0,
                    height as f32,
                    width as f32,
                    (width as f32 - height as f32).abs(),
                )) as usize;
            if row == (height - 2) as usize {
                break;
            }
            points.push((row as f32, col as f32));
            x += 1.0;
        }
        let points = points.iter().map(|point| Point::from(*point)).collect();
        let mut line = Line::new(points, (0, 0, 0, 255), 1);
        line.draw(canvas);
        y += 1.0;
    }
}
fn map(val: f32, start_1: f32, end_1: f32, start_2: f32, end_2: f32) -> f32 {
    if start_1 == start_2 && end_1 == end_2 {
        val
    } else if end_1 == end_2 {
        val * (1.0 + (start_2 / end_1))
    } else {
        (val - start_1) * ((end_2 - start_2) / (end_1 - start_1)) + start_2
    }
}
fn dec_l(x: f32) -> f32 {
    1.0 - (x - 1.0).powf(2.0)
}
fn main() {
    let mut ctx = setup();
    let amplitude = 5.0;
    display(&mut ctx, amplitude);
    ctx.save_as_image("examples/outputs/animal_wave.png");
}
