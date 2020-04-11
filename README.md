# pix

Library for pixel and image compositing.

A raster image is a rectangular array of pixels.

## Color Models
* `RGB` / `BGR` (*red*, *green*, *blue*)
* `Gray` (*luma* / *relative luminance*)
* `HSV` (*hue*, *saturation*, *value*)
* `HSL` (*hue*, *saturation*, *lightness*)
* `HWB` (*hue*, *whiteness*, *blackness*)
* `YCbCr` (JPEG)
* `Matte` (*alpha* only)

### Example: Convert Raster Format
```
use pix::{Raster, Rgba8p, SRgb8};

let mut src = Raster::<SRgb8>::with_clear(120, 120);
// ... load pixels into raster
let dst: Raster<Rgba8p> = Raster::with_raster(&src);
```

## Documentation
[https://docs.rs/pix](https://docs.rs/pix)
