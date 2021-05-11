use generative::prelude::*;

fn setup() -> Canvas {
    let mut canvas = Canvas::new(2840, 1600);
    canvas.fill((0, 0, 0, 255));
    canvas
}

fn display(canvas: &mut Canvas) {
    let width = canvas.get_width() as f32;
    let height = canvas.get_height() as f32;
    let w = 4.0;
    let h = (w * height) / width;
    let xmin = -h / 1.0;
    let ymin = -w / 3.5;
    let xmax = xmin + h;
    let ymax = ymin + w;

    let dx = (xmax - xmin) / height;
    let dy = (ymax - ymin) / width;
    let max_iter = 50;

    let mut y = ymin;
    for j in 0..(height as usize) {
        let mut x = xmin;
        for i in 0..(width as usize) {
            let mut a = x;
            let mut b = y;
            let mut n = 0;

            while n < max_iter {
                let aa = a * a;
                let bb = b * b;
                let twoab = 2.0 * a * b;
                a = aa - bb + x;
                b = twoab + y;
                if euclid_dist((aa, bb), (0.0, 0.0)) > 100.0 {
                    break;
                }
                n += 1;
            }
            let norm_val = map(n as f32, 0.0, max_iter as f32, 0.0, 1.0, false);
            let bright = map(norm_val.powf(2.0), 0.0, 1.0, 0.0, 255.0, false) as u8;
            if n < max_iter {
                canvas.set_pixel_at(j, i, (bright, bright, bright, 255));
            }
            x += dx;
        }
        y += dy;
    }
}

fn main() {
    let mut ctx = setup();
    display(&mut ctx);
    ctx.save_as_image("examples/outputs/mandelbrot.png");
}
