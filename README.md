# pix

Library for pixel and image compositing.

A raster image is a rectangular array of pixels.

## Color Models
* `RGB` / `BGR` (*red*, *green*, *blue*)
* `CMY` (*cyan*, *magenta*, *yellow*)
* `Gray` (*luma* / *relative luminance*)
* `HSV` (*hue*, *saturation*, *value*)
* `HSL` (*hue*, *saturation*, *lightness*)
* `HWB` (*hue*, *whiteness*, *blackness*)
* `YCbCr` (JPEG)
* `Matte` (*alpha* only)

### Example: Color Demo
```
use pix::hwb::SHwb8;
use pix::Raster;

let mut r = Raster::with_clear(256, 256);
for (y, row) in r.rows_mut(r.region()).enumerate() {
    for (x, p) in row.iter_mut().enumerate() {
        let h = ((x + y) >> 1) as u8;
        let w = y.saturating_sub(x) as u8;
        let b = x.saturating_sub(y) as u8;
        *p = SHwb8::new(h, w, b);
    }
}
```

![Colors](https://raw.githubusercontent.com/DougLau/pix/master/res/colors.png)

### Example: Convert Raster Format
```
use pix::rgb::{Rgba8p, SRgb8};
use pix::Raster;

let mut src = Raster::<SRgb8>::with_clear(120, 120);
// ... load pixels into raster
let dst: Raster<Rgba8p> = Raster::with_raster(&src);
```

## Documentation
[https://docs.rs/pix](https://docs.rs/pix)
