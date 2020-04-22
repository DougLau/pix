// lib.rs      Pix crate.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Library for pixel and image compositing.
//!
//! A [raster] image is a rectangular array of [pixel]s.
//!
//! ## Color Models
//! * [`RGB`] / `BGR` (*red*, *green*, *blue*)
//! * [`CMY`] (*cyan*, *magenta*, *yellow*)
//! * [`Gray`] (*luma* / *relative luminance*)
//! * [`HSV`] (*hue*, *saturation*, *value*)
//! * [`HSL`] (*hue*, *saturation*, *lightness*)
//! * [`HWB`] (*hue*, *whiteness*, *blackness*)
//! * [`YCbCr`] (used by JPEG)
//! * `Matte` (*alpha* only)
//!
//! [alpha]: chan/trait.Alpha.html
//! [channel]: chan/trait.Channel.html
//! [cmy]: https://en.wikipedia.org/wiki/CMY_color_model
//! [color model]: trait.ColorModel.html
//! [gamma]: chan/trait.Gamma.html
//! [`gray`]: https://en.wikipedia.org/wiki/Grayscale
//! [`hsl`]: https://en.wikipedia.org/wiki/HSL_and_HSV
//! [`hsv`]: https://en.wikipedia.org/wiki/HSL_and_HSV
//! [`hwb`]: https://en.wikipedia.org/wiki/HWB_color_model
//! [pixel]: el/trait.Pixel.html
//! [raster]: struct.Raster.html
//! [`rgb`]: https://en.wikipedia.org/wiki/RGB_color_model
//! [`ycbcr`]: https://en.wikipedia.org/wiki/YCbCr
//!
//! ### Example: Color Demo
//! ```
//! use pix::hwb::SHwb8;
//! use pix::Raster;
//!
//! let mut r = Raster::with_clear(256, 256);
//! for (y, row) in r.rows_mut(r.region()).enumerate() {
//!     for (x, p) in row.iter_mut().enumerate() {
//!         let h = ((x + y) >> 1) as u8;
//!         let w = y.saturating_sub(x) as u8;
//!         let b = x.saturating_sub(y) as u8;
//!         *p = SHwb8::new(h, w, b);
//!     }
//! }
//! ```
//!
//! ![Colors](https://raw.githubusercontent.com/DougLau/pix/master/res/colors.png)
//!
//! ### Example: Convert Raster Format
//! ```
//! use pix::rgb::{Rgba8p, SRgb8};
//! use pix::Raster;
//!
//! let mut src = Raster::<SRgb8>::with_clear(120, 120);
//! // ... load pixels into raster
//! let dst: Raster<Rgba8p> = Raster::with_raster(&src);
//! ```
//!
//! ## Documentation
//! [https://docs.rs/pix](https://docs.rs/pix)
//!
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

pub mod bgr;
pub mod chan;
pub mod cmy;
pub mod el;
pub mod gray;
pub mod hsl;
pub mod hsv;
mod hue;
pub mod hwb;
pub mod matte;
mod model;
pub mod ops;
mod palette;
mod private;
mod raster;
pub mod rgb;
pub mod ycc;

pub use crate::model::ColorModel;
pub use crate::palette::Palette;
pub use crate::raster::{Raster, Region, Rows, RowsMut};
