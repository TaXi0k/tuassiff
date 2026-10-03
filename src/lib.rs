use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use crate::error::Error;

// ==================================================

pub mod error;

// ==================================================

pub struct Data {
    width: u32,
    height: u32,
    pixels: Vec<Rgba<u8>>,
}
impl Data {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> &[Rgba<u8>] {
        &self.pixels
    }

    // Open tuassiff:Data from a file
    pub fn open(source: &Path) -> Result<Self, Error> {
        todo!()
    }

    // Write tuassiff::Data to a file
    pub fn save(&self, target: &Path) -> Result<(), Error> {
        todo!()
    }

    // Decode tuassiff::Data to image::DynamicImage
    pub fn to_img(self) -> DynamicImage {
        let mut img: RgbaImage = ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = ((y * self.width) + x) as usize;
            *pixel = self.pixels[index];
        }

        DynamicImage::from(img)
        }

    // Encode image::DynamicImage
    pub fn from_img(img: DynamicImage) -> Self {
        let width = img.width();
        let height = img.height();
        let pixels: Vec<Rgba<u8>> = img.to_rgba32f().pixels().map(
            |p|
            {
                let channels = p.channels();
                Rgba([
                    (channels[0].clamp(0.0, 1.0) * 255.0) as u8,
                    (channels[1].clamp(0.0, 1.0) * 255.0) as u8,
                    (channels[2].clamp(0.0, 1.0) * 255.0) as u8,
                    (channels[3].clamp(0.0, 1.0) * 255.0) as u8,
                ])
            }
        ).collect();

        Self { width, height, pixels }
    }
}

