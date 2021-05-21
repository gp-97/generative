use generative::prelude::*;

fn setup() -> Canvas {
    let mut canvas = Canvas::new(1920 * 4, 1080 * 4);
    canvas.fill((12, 12, 12, 255));
    canvas
}

fn field_point_generator(
    field_points_holder: &mut Vec<Vec<Point>>,
    field_points_xy_holder: &mut Vec<Vec<Point>>,
    canvas: &mut Canvas,
) {
    let amp = 15.0;
    let perlin = PerlinNoise2D::new(1, amp, 0.2, 0.25, 10.0, (100.0, 100.0), 0.5, 101);
    let width = canvas.get_width() as usize;
    let height = canvas.get_height() as usize;
    let step = 12;

    for row in (0..height).step_by(step) {
        let mut points = vec![];
        let mut points_xy = vec![];
        for col in (0..width).step_by(step) {
            let noise = perlin.get_noise(row as f64, col as f64) as f32;
            let angle = 2.0 * std::f32::consts::PI * norm(noise, 0.0, amp as f32);
            let x = step as f32 * angle.cos() + row as f32;
            let y = step as f32 * angle.sin() + col as f32;

            points_xy.push(Point::new(row as f32, col as f32));
            points.push(Point::new(x, y));
        }
        field_points_holder.push(points);
        field_points_xy_holder.push(points_xy);
    }
}

fn display(canvas: &mut Canvas) {
    let width = canvas.get_width();
    let height = canvas.get_height();
    let mut field_points = Vec::<Vec<Point>>::new();
    let mut field_points_xy = Vec::<Vec<Point>>::new();
    field_point_generator(&mut field_points, &mut field_points_xy, canvas);

    let radius = 8_f32;
    for _ in 0..1500 {
        let x_rand = random(0.0, height as f32);
        let y_rand = random(0.0, width as f32);
        let mut p_rand = Point::new(x_rand, y_rand);
        let mut points = vec![p_rand];
        let mut final_point = Point::new(x_rand, y_rand);

        let color = (
            random(0.0, 255.0) as u8,
            random(0.0, 255.0) as u8 % 10,
            random(0.0, 255.0) as u8,
            255,
        );

        for _ in 0..1000 {
            if final_point.get_x() >= 0.0
                && final_point.get_x() < height as f32
                && final_point.get_y() >= 0.0
                && final_point.get_y() < width as f32
            {
                for (point_vec, point_xy_vec) in field_points.iter().zip(field_points_xy.iter()) {
                    for (point, point_xy) in point_vec.iter().zip(point_xy_vec.iter()) {
                        if point.euclid_dist_square(&p_rand) <= radius.powf(2.0) {
                            let i = point.get_x() - point_xy.get_x();
                            let j = point.get_y() - point_xy.get_y();
                            final_point.set_x(final_point.get_x() + i);
                            final_point.set_y(final_point.get_y() + j);
                        }
                    }
                }
                points.push(final_point);
                p_rand = final_point;
            }
        }
        let mut line = Line_aa::new(points.clone(), color, 1, canvas);
        line.draw(canvas);
    }
}

fn main() {
    let mut canvas = setup();
    display(&mut canvas);
    canvas.save_as_image("examples/outputs/perlin_field_lines.png");
}
