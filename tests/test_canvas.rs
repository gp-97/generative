#[cfg(test)]
use generative::canvas::Canvas;
use photon_rs::native::save_image;

#[test]
fn test_init() {
    let canv = Canvas::new(512, 512);
    save_image(Canvas::to_photon(&canv), "tests/outputs/canvas_init.jpg");
}
#[test]
fn test_fill() {
    let mut canv = Canvas::new(512, 512);
    canv.fill((180_u8, 2_u8, 50_u8, 255_u8));
    save_image(Canvas::to_photon(&canv), "tests/outputs/canvas_fill.jpg");
}

#[test]
fn test_set_pixel_at() {
    let mut canv = Canvas::new(500, 500);

    canv.fill((240_u8, 240_u8, 240_u8, 255_u8));

    // Left Eye
    for i in 70..170 {
        for j in 100..200 {
            canv.set_pixel_at(i, j, (200_u8, 226_u8, 152_u8, 255_u8));
        }
    }

    // Right Eye
    for i in 70..170 {
        for j in 300..400 {
            canv.set_pixel_at(i, j, (200_u8, 226_u8, 152_u8, 255_u8));
        }
    }

    // Nose
    for i in 150..300 {
        for j in 225..275 {
            canv.set_pixel_at(i, j, (200_u8, 226_u8, 152_u8, 255_u8));
        }
    }

    // Mouth
    for i in 350..420 {
        for j in 50..450 {
            canv.set_pixel_at(i, j, (200_u8, 226_u8, 152_u8, 255_u8));
        }
    }
    save_image(Canvas::to_photon(&canv), "tests/outputs/canvas_set_pixel.jpg");
}
