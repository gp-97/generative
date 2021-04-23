#[cfg(test)]
use generative::canvas::Canvas;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;
#[test]
fn test_init() {
    let canv = Canvas::new(512, 512);
    save_image(Canvas::to_photon(&canv), "assets/canvas_init.jpg");
}
#[test]
fn test_fill() {
    let mut canv = Canvas::new(512, 512);
    canv.fill((180_u8, 2_u8, 50_u8, 255_u8));
    save_image(Canvas::to_photon(&canv), "assets/canvas_fill.jpg");
}
#[test]
fn test_get_pixel_at() {
    let canv = Canvas::new(512, 512);
    let p1 = canv.get_pixel_at(112, 134);
    let p2 = canv.get_pixel_at(1120, 1340);

    assert_eq!(p1, Some((0, 0, 0, 0)));
    assert_eq!(p2, None);
}

#[test]
fn test_set_pixel_at() {
    let mut canv = Canvas::new(512, 512);
    for i in 10..200 {
        for j in 100..200 {
            canv.set_pixel_at(i, j, (180_u8, 2_u8, 50_u8, 255_u8));
        }
    }
    save_image(Canvas::to_photon(&canv), "assets/canvas_set_pixel.jpg");
}
