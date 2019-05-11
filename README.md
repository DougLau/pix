# pix
Rust crate for pixel and raster image manipulation.
Currently an early work-in-progress.

## Documentation
[https://docs.rs/pix](https://docs.rs/pix)

## Future Plans
* Conversions between pixel formats.

### Cover Trait
* Provides iterator of pixels in a region
* Impl by Raster
* Impl by Rgb, Gray, Mask, etc.
* Impl by Gradients (linear, radial, etc)
* Raster region copying (or iterating).

### Color models / spaces
* Tristimulus, additive, subtractive, cylindrical
* RGB: additive
* CMYK: subtractive
* HSV / HSL: cylindrical
* LAB, LCH
* XYZ: tristimulus
* White point
