![Tuassiff - An image file format](/assets/repo_cover.png)

**Totally Unoptimized and Super Slow Image File Format**

Because PNG, JPEG and WebP were just too cliché.

---

Maintainers: [@TaXi0k](https://github.com/TaXi0k)

## What is tuassiff?

**Tuassiff** is a bespoke, **almost** zero-dependency, **🔥blazing-fast🔥** (since written in Rust), binary image serialization file format and CLI converter tool to and from PNG.  
It takes perfectly good and optimized PNG file, gets rid of all the compression and efficiency and transforms it into a very sluggish proprietary format you have absolutely zero business using in prod. Or anywhere to be more precise.

## What about this crate?

This crate aims to provide you an option to interact with **tuassiff** files by converting them to and from [image crate's](https://crates.io/crates/image) `DynamicImage` type.

