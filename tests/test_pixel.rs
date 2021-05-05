#[cfg(test)]
use generative::canvas::Canvas;
use generative::Pixel;
use generative::Point;

#[test]
fn test_pixel_init() {
    let p = Point::new(10.0, 12.0);
    let c = (255, 128, 134, 255);
    let px1 = Pixel::new(p, c);
    assert_eq!(px1.get_color(), c);
    assert_eq!(px1.get_point(), p);
}

#[test]
fn test_within_canvas() {
    let ctx = Canvas::new(512, 512);
    let p = Point::new(10.0, 12.0);
    let c = (255, 128, 134, 255);
    let px1 = Pixel::new(p, c);
    assert_eq!(px1.is_within_canvas(&ctx), true);
}
