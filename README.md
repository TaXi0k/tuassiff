![Tuassiff - An image file format](https://raw.githubusercontent.com/TaXi0k/tuassiff/refs/heads/master/assets/repo_cover.png)

**Totally Unoptimized and Super Slow Image File Format**

Because PNG, JPEG and WebP were just too cliché.

---

Maintainers: [@TaXi0k](https://github.com/TaXi0k)

## What is tuassiff?

**Tuassiff** is a bespoke, **almost** zero-dependency, **🔥blazing-fast🔥** (since written in Rust), binary image serialization file format and CLI converter tool to and from PNG.  
It takes a perfectly good and optimized PNG file, gets rid of all the compression and efficiency and transforms it into a very sluggish proprietary format you have absolutely zero business using in prod. Or anywhere to be more precise.

### Core features:

* **Full OPSEC friendliness:** tuassiff **can not** store **any** EXIF data. It only stores image dimmensions and data about each pixel. This means as long as you use tuassiff, you don't have to ever think about removing exif data or anything like that!
* **Strict Magic Bytes Verification:** Uses unmistakeable 12-byte signature to instantly validate file integrity, completly ignoring file extensions.
* **Zero-Bloat Minimalist Design:** Strips away all extraneous formatting overhead, storing only exact dimmensions (`width` x `height`) and the raw pixel array.

## What about this crate?

This crate aims to provide you an option to interact with **tuassiff** files by converting them to and from [image crate's](https://crates.io/crates/image) `DynamicImage` type.  
Additionally it includes a little CLI wrapper for this library, allowing you to quickly convert between **PNG** and **tuassiff**.

# Library

## High level API

Load tuassiff images using `open()`:
```rs
let tuassiff_data = tuassiff::Data::open(&path)?;
```

Save tuassiff images using `save()`:
```rs
tuassiff_data.save(&path)?;
```

Convert `tuassiff::Data` to `image::DynamicImage`:
```rs
let dynamic_image = tuassiff_data.to_img();
```

Convert `image::DynamicImage` to `tuassiff::Data`:
```rs
let tuassiff_data = tuassiff::Data::from_img(dynamic_image);
```

## Examples

### Converting PNG image to tuassiff and saving it to the drive

Using this crate together with [`image`](https://crates.io/crates/image) you can easily write a simple converter from PNG to tuassiff.
```rs
use tuassiff;
use image;


let source = PathBuf::from("/home/user/image.png");
let target = PathBuf::from("/home/user/image.tuassiff");

// Use image's open() function to open a PNG file
let img = image::open(&source)?;

// Convert DynamicImage to tuassiff::Data consuming original DynamicImage
let data: tuassiff::Data = tuassiff::Data::from_img(img);

// Save tuassiff::Data to drive
data.save(&target)?;
```
Again, together with [`image`](https://crates.io/crates/image) you can easily write a simple converter from tuassiff to PNG.
```rs
use tuassiff;
use image;


let source = PathBuf::from("/home/user/image.tuassiff");
let target = PathBuf::from("/home/user/image.png");

// Use open() function to open a tuassiff file and get it's data
let data: tuassiff::Data = tuassiff::Data::open(&source)?;

// Convert tuassiff::Data to DynamicImage consuming original tuasiff::data
let img = data.to_img();

// Save DynamicImage to drive
img.save(&target)?;
```

# CLI wrapper

As already mentioned, this crate provides you with a tiny CLI wrapper allowing you to perform quick conversions between **PNG** and **tuassiff**.  
### Usage:
```
Usage: tuassiff <encode|decode> <source> <target>
 * encode - convert PNG file to TUASSIFF file
 * decode - convert TUASSIFF file to PNG file
```
Additionally if you ever forget what's the syntax, you'll see exact same thing if you provide incorrect or no arguments, so to see the help menu you can type: `tuassiff` or `tuassiff help` or `tuassiff --help` or `tuassiff -h` or `tuassif Lorem Ipsum` etc.

# Installation

* **Library:**  
    Run the following command in your project directory: `cargo add tuassiff`

* **CLI wrapper:**  
    Run the following command in your terminal: `cargo install tuassiff`  
    Install binary from [latest GitHub release](https://github.com/TaXi0k/tuassiff/releases).

* **Building from source:**  
  1. Clone the repository: `git clone https://github.com/TaXi0k/tuassiff.git`
  2. Enter its directory: `cd tuassiff`
  3. Compile with release flag: `cargo build --release`
  4. Now your compiled binary is sitting in `./target/release`

# Special thanks to

* All contributors of crate [**image**](https://crates.io/crates/image) wihout which this project might never have been finished
* [**FaceDev**](https://www.youtube.com/@FaceDevStuff) for inspiring me to do this project (especially his [*I built my own Image Format*](https://youtu.be/48B8FPmMT0g?si=REzM9hdomVZ6Pu__) video)

# License

This project is licensed under [**Creative Commons Attribution-NonCommercial 4.0 International**](https://creativecommons.org/licenses/by-nc/4.0/) (CC BY-NC 4.0).