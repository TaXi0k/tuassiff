use std::path::Path;

use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
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
    pub fn from_img(img: DynamicImage) -> Result<Self, Error> {
        todo!()
    }
}