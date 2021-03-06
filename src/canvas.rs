use crate::{Pixel, Point};
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;
use std::num::Wrapping;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer: Vec<u8> = vec![255; (width * height * 4) as usize];
        Self { width, height, buffer }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn fill(&mut self, color: (u8, u8, u8, u8)) {
        for i in (0..self.buffer.len()).step_by(4) {
            self.buffer[i] = color.0;
            self.buffer[i + 1] = color.1;
            self.buffer[i + 2] = color.2;
            self.buffer[i + 3] = color.3;
        }
    }

    pub fn get_pixel_at(&self, row: usize, column: usize) -> Option<Pixel> {
        let idx = self.index_at(row, column);
        if idx < self.buffer.len() - 4 {
            let point = Point::new(row as f32, column as f32);
            let color = (
                self.buffer[idx],
                self.buffer[idx + 1],
                self.buffer[idx + 2],
                self.buffer[idx + 3],
            );
            let pixel = Pixel::new(point, color);
            Some(pixel)
        } else {
            None
        }
    }

    pub fn set_pixel_at(&mut self, row: usize, column: usize, pixel: (u8, u8, u8, u8)) {
        if row < self.height as usize && column < self.width as usize {
            let idx = self.index_at(row, column);
            if idx < self.buffer.len() - 4 {
                self.buffer[idx] = pixel.0;
                self.buffer[idx + 1] = pixel.1;
                self.buffer[idx + 2] = pixel.2;
                self.buffer[idx + 3] = pixel.3;
            }
        }
    }

    pub fn to_photon(canvas: &Canvas) -> PhotonImage {
        PhotonImage::new(canvas.get_buffer(), canvas.get_width(), canvas.get_height())
    }

    fn index_at(&self, row: usize, column: usize) -> usize {
        Wrapping(
            Wrapping(row) * Wrapping(self.width as usize * 4) + Wrapping(Wrapping(column as usize) * Wrapping(4)).0,
        )
        .0
         .0
    }

    pub fn save_as_image(&self, image_path: &str) {
        save_image(Canvas::to_photon(self), image_path);
    }
    pub fn image_as_canvas(image_path: &str) -> Self {
        match open_image(image_path) {
            Ok(photon_image) => Self {
                width: photon_image.get_width(),
                height: photon_image.get_height(),
                buffer: photon_image.get_raw_pixels(),
            },
            Err(_) => Self {
                width: 512,
                height: 512,
                buffer: vec![255; (512 * 512 * 4) as usize],
            },
        }
    }
}

impl From<PhotonImage> for Canvas {
    fn from(photon_image: PhotonImage) -> Self {
        Self {
            width: photon_image.get_width(),
            height: photon_image.get_height(),
            buffer: photon_image.get_raw_pixels(),
        }
    }
}
