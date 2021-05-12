use generative::prelude::*;

struct Brush {
    angle: Angle,
    components: Vec<isize>,
    x: f32,
    y: f32,
    color: (u8, u8, u8, u8),
}

impl Brush {
    pub fn new(width: usize, height: usize) -> Self {
        let angle = Angle::RADIAN(random(0.0, std::f32::consts::PI * 2.0));
        let x = random(0.0, height as f32);
        let y = random(0.0, width as f32);
        let color = (
            random(0.0, 256.0) as u8,
            random(0.0, 256.0) as u8,
            random(0.0, 256.0) as u8,
            255,
        );
        let components = vec![random(1.0, 5.0) as isize, random(1.0, 5.0) as isize];

        Self {
            angle,
            components,
            x,
            y,
            color,
        }
    }

    pub fn paint(&mut self, canvas: &mut Canvas) {
        let mut a = 0_f32;
        let mut r = 0_f32;
        let mut x1 = self.x;
        let mut y1 = self.y;
        let u = random(0.5, 1.0);
        let mut angle = Angle::get_angle_rad(self.angle);

        while a < std::f32::consts::PI * 2.0 {
            canvas.set_pixel_at(x1 as usize, y1 as usize, self.color);
            let v = random(1.0, 5.0);
            x1 = self.x + r * f32::cos(a * angle) * u * v;
            y1 = self.y + r * f32::cos(a * angle) * u * v;
            r += f32::sin(a * self.components[0] as f32) + f32::sin(a * self.components[1] as f32);
            let mut circle = Circle::new(Point::new(x1, y1), 500.0, self.color, 1);
            circle.draw(canvas);
            a += 10.0;
        }

        if self.x < 0.0 || self.x > canvas.get_height() as f32 || self.y < 0.0 || self.y > canvas.get_width() as f32 {
            angle += std::f32::consts::FRAC_PI_2;
            self.angle = Angle::RADIAN(angle);
        }

        self.x += 2.0 * f32::cos(angle);
        self.y += 2.0 * f32::sin(angle);
        angle += random(-0.15, 0.15);
        self.angle = Angle::RADIAN(angle);
    }
}

fn setup() -> Canvas {
    let canvas = Canvas::new(3840, 2160);
    canvas
}

fn display(canvas: &mut Canvas, brushes: &mut Vec<Brush>) {
    for brush in brushes {
        for _ in 0..500 {
            brush.paint(canvas);
        }
    }
}

fn main() {
    let mut ctx = setup();
    let mut brushes = vec![];
    for _ in 0..500 {
        brushes.push(Brush::new(ctx.get_width() as usize, ctx.get_height() as usize));
    }
    display(&mut ctx, &mut brushes);
    ctx.save_as_image("examples/outputs/watercolor_circles.png");
}
