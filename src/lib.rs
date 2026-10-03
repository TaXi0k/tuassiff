use std::{fs, io::Write, path::Path};

use image::{DynamicImage, ImageBuffer, Pixel, Rgba, RgbaImage};
use crate::error::Error;

// ==================================================

pub mod error;

// ==================================================

pub const MAGIC_BYTES: [u8; 12] = [
    0xdf, 0xa4, 0x43, 0xfb, 0xc0, 0x7b, 0xd6, 0xf0, 0xf5, 0x71, 0x92, 0xa9
];
pub const HEADER_LEN: usize = 20;

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
        match fs::read(source) {
            Ok(bytes) => {
                // Throw an error if file is shorter than header length
                if bytes.len() < HEADER_LEN { return Err(Error::UnexpectedEof); }

                // Split file into sections
                let magic_bytes = &bytes[0..12];
                let width_bytes = &bytes[12..16];
                let height_bytes = &bytes[16..20];
                let pixels_bytes = &bytes[20..];

                // Throw an error if magic bytes don't match
                if magic_bytes != MAGIC_BYTES { return Err(Error::InvalidMagic); }

                // Parsing width and height from raw bytes
                let width = u32::from_le_bytes(
                    width_bytes
                        .try_into()
                        .map_err(|_| Error::InvalidHeader )?
                );
                let height = u32::from_le_bytes(
                    height_bytes
                        .try_into()
                        .map_err(|_| Error::InvalidHeader )?
                );

                // Checking length of data part of file
                let expected_pixels = (width as usize) * (height as usize);
                if pixels_bytes.len() / 4 < expected_pixels { return Err(Error::UnexpectedEof); }
                else if pixels_bytes.len() / 4 > expected_pixels { return Err(Error::UnexpectedTrailingData); }

                let mut pixels: Vec<Rgba<u8>> = Vec::new();
                for chunk in pixels_bytes.chunks_exact(4) {
                    let channel_bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                    pixels.push(Rgba(channel_bytes));
                }

                Ok(Self{
                    width,
                    height,
                    pixels,
                })

            },
            Err(e) => Err( Error::Read(e) ),
        }

    }

    // Write tuassiff::Data to a file
    pub fn save(&self, target: &Path) -> Result<(), Error> {
        
        match fs::File::create(target) {
            Ok(mut file) => {
                file.write_all(&MAGIC_BYTES).map_err(Error::Write)?;
                file.write_all(&self.width.to_le_bytes()).map_err(Error::Write)?;
                file.write_all(&self.height.to_le_bytes()).map_err(Error::Write)?;

                for pixel in &self.pixels {
                    file.write_all(pixel.channels()).map_err(Error::Write)?;
                }
                
                Ok(())
            },
            Err(e) => Err(Error::Create(e))
        }
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

