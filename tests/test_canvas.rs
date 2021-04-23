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
