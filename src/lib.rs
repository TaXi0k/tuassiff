//! # tuassiff
//! 
//! **Tuassiff** is a bespoke, minimalist binary image serialization format 
//! designed with strict OPSEC and zero metadata overhead.
//! 
//! ## Quick Start
//! 
//! Interacting with tuassiff files revolves around the code [`Data`] structure,
//! which bridges your files and the broader [`image`] crate ecosystem.
//! 
//! ### Converting a **PNG** to **tuassiff**
//! 
//! ```no_run
//! use tuassiff::Data;
//! use image;
//! 
//! 
//! let source = PathBuf::from("/home/user/image.png");
//! let target = PathBuf::from("/home/user/image.tuassiff");
//! 
//! // Open standard image via the image crate ecosystem
//! let img = image::open(&source).unwrap();
//! 
//! // Convert DynamicImage to tuassiff::Data
//! let data = Data::from_img(img);
//! 
//! // Save to drive
//! data.save(&target).unwrap();
//! ```
//! 
//! ### Converting **tuassiff** back to **PNG**
//! ```no_run
//! use tuassiff::Data;
//! use image;
//! 
//! let source = PathBuf::from("/home/user/image.tuassiff");
//! let target = PathBuf::from("/home/user/image.png");
//! 
//! // Open and validate tuassiff file data
//! let data = tuassiff::Data::open(&source).unwrap();
//! 
//! // Convert back to DynamicImage
//! let img = data.to_img();
//! 
//! // Save standard image to drive
//! img.save(&target).unwrap();
//! ```

use std::{fs, io::Write, path::Path};
use image::{DynamicImage, ImageBuffer, Pixel, Rgba, RgbaImage};

// ==================================================

pub mod error;
pub use crate::error::Error;

// ==================================================

/// The unique magic bytes signature that identifies a valid `.tuassiff` file.
/// 
/// Placed at very beginning of the file, this sequence allows
/// [`Data::open`] to strictly verify the file format instantly,
/// completely ignoring misleading file extensions.
/// 
/// **For curious:**  
/// 
/// In base10 these read:
/// ```
/// 69213742067133780085420692137
/// ```
/// which is quite funny indeed (laugh a bit at least, I beg you!)
pub const MAGIC_BYTES: [u8; 12] = [
    0xdf, 0xa4, 0x43, 0xfb, 0xc0, 0x7b, 0xd6, 0xf0, 0xf5, 0x71, 0x92, 0xa9
];

/// The total fixed size (in bytes) of the `.tuassiff` file header.
/// 
/// This accounts for the space occupied by the [`MAGIC_BYTES`]
/// plus the serialized width and height metadata fields
/// before the pixel buffer begins.
pub const HEADER_LEN: usize = 20;

// ==================================================

/// **tuassiff::Data**
/// 
/// This struct holds the raw dimensions and pixel buffer required 
/// to serialize or deserialize. 
pub struct Data {
    /// The width of the image in pixels.
    width: u32,
    /// The height of the image in pixels.
    height: u32,
    /// The raw pixel buffer stored as a vector of [`image::Rgba`] values.
    pixels: Vec<Rgba<u8>>,
}
impl Data {
    /// Returns the width of the image in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Returns the height of the image in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Returns the raw pixel buffer as slice of [`image::Rgba`] values
    pub fn pixels(&self) -> &[Rgba<u8>] {
        &self.pixels
    }
    /// Creates a new [`Data`] instance from the provided dimensions and pixel buffer.
    /// 
    /// # Errors
    /// 
    /// * **[`Error::InvalidPixels`]** - if length of provided pixels vector isn't equal to product of width and height
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tuassiff::Data;
    /// use image::Rgba;
    /// 
    /// // Create a tiny 2x3 red image buffer
    /// let pixels = vec![Rgba([255, 0, 0, 255]); 6];
    /// let data = Data::from(2, 3, pixels).unwrap();
    /// 
    /// assert_eq!(data.width(), 2);
    /// assert_eq!(data.height(), 3);
    /// assert_eq!(data.pixels(), &[
    ///     Rgba([255, 0, 0, 255]),
    ///     Rgba([255, 0, 0, 255]),
    ///     Rgba([255, 0, 0, 255]),
    ///     Rgba([255, 0, 0, 255]),
    ///     Rgba([255, 0, 0, 255]),
    ///     Rgba([255, 0, 0, 255]),
    /// ]); 
    /// ```
    pub fn from(width: u32, height: u32, pixels: Vec<Rgba<u8>>) -> Result<Self, Error> {
        if pixels.len() != (width as usize) * (height as usize) { return Err(Error::InvalidPixels); }
        Ok( Self { width, height, pixels } )
    }

    /// Opens a `.tuassiff` image from the specified path and returns its [`Data`].
    /// 
    /// File extensions are not enforced; instead, the file's magic bytes are validated. 
    /// Returns an [`Error::InvalidMagic`] if the file is not a valid `.tuassiff` format.
    /// 
    /// # Errors
    /// 
    /// * **[`Error::Read`]** - if reading file at specified path fails, wraps `std::io::Error`, that caused it.
    /// * **[`Error::InvalidMagic`]** - if opened file doesn't start with correct [`MAGIC_BYTES`]
    /// * **[`Error::UnexpectedEof`]** - if file is shorten than minimal required.
    /// * **[`Error::InvalidHeader`]** - if parsing width or height buffers as u32 fails, likely caused by some kind of corruption of file
    /// * **[`Error::InvalidPixels`]** - if count of pixels provided in data part doesn't match expected (`let expected_pixels: usize = (width as usize) * (height as usize);`).
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use tuassiff::Data;
    /// use std::path::Path;
    /// use image::Rgba;
    /// 
    /// // Let's assume the `image.tuassiff` is a 2x3 green rectangle
    /// let path = Path::new("image.tuassiff");
    /// let data = Data::open(path).unwrap();
    /// 
    /// assert_eq!(data.width(), 2);
    /// assert_eq!(data.height(), 3);
    /// assert_eq!(data.pixels(), &[
    ///     Rgba([0, 255, 0, 255]),
    ///     Rgba([0, 255, 0, 255]),
    ///     Rgba([0, 255, 0, 255]),
    ///     Rgba([0, 255, 0, 255]),
    ///     Rgba([0, 255, 0, 255]),
    ///     Rgba([0, 255, 0, 255]),
    /// ]);
    /// ```
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
                let expected_pixels: usize = (width as usize) * (height as usize);
                if pixels_bytes.len() / 4 != expected_pixels { return Err(Error::InvalidPixels); }

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

    /// Serializes the image data into the `.tuassiff` binary format and writes it to a specified path.
    /// 
    /// # Errors
    /// 
    /// * **[`Error::Create`]** - if creation of a new file at specified path fails
    /// * **[`Error::Write`]** - if writing to newly created file fails
    /// 
    /// Both errors wrap `std::io::Error`, that caused them.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use tuassiff::Data;
    /// use image::Rgba;
    /// use std::path::Path;
    /// 
    /// // Let's create a tiny 3x2 blue rectangle
    /// let pixels = vec![Rgba([0, 0, 255, 255]); 6];
    /// let data = Data::from(3, 2, pixels).unwrap();
    /// 
    /// let path = Path::new("output.tuassiff");
    /// data.save(path).unwrap()
    /// ```
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

    /// Converts this [`Data`] instance to a standard [`image::DynamicImage`].
    /// 
    /// This integrates tuassiff with [`image`] crate ecosystem, letting you eg. edit .tuassiff files.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tuassiff::Data;
    /// use image::{Rgba, DynamicImage};
    /// 
    /// // Let's create a little yellow 2x3 rectangle.
    /// let pixels = vec![Rgba([255, 255, 0, 255]); 6];
    /// let data = Data::from(2, 3, pixels).unwrap();
    /// 
    /// // Convert to a DynamicImage
    /// let dynamic_image = data.to_img();
    /// 
    /// assert_eq!(dynamic_image.width(), 2);
    /// assert_eq!(dynamic_image.height(), 3);
    /// ```
    pub fn to_img(self) -> DynamicImage {
        let mut img: RgbaImage = ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = ((y * self.width) + x) as usize;
            *pixel = self.pixels[index];
        }

        DynamicImage::from(img)
        }

    /// Convets a standard [`image::DynamicImage`] into a [`Data`] instance.
    /// 
    /// This allows you to take any ordinary image (like PNG or JPEG)
    /// and package it into this ever superior **tuassiff** format.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tuassiff::Data;
    /// use image::{DynamicImage, RgbaImage};
    /// 
    /// // Let's create a blank 2x2 RGBA image
    /// let img_buffer = RgbaImage::new(2, 2);
    /// let dynamic_img = DynamicImage::ImageRgba8(img_buffer);
    /// 
    /// let data = Data::from_img(dynamic_img);
    /// 
    /// assert_eq!(data.width(), 2);
    /// assert_eq!(data.height(), 2);
    /// ```
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

