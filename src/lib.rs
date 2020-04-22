// lib.rs      Pix crate.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Library for image conversion and compositing.
//!
//! A [raster] image can be cheaply converted to and from raw byte buffers,
//! enabling interoperability with other crates.
//!
//! Many image formats are supported:
//!
//! * Bit depth: 8- or 16-bit integer and 32-bit float
//! * Alpha: *premultiplied* or *straight*
//! * Gamma: *linear* or *sRGB*
//! * Color models:
//!   - [`RGB`] / [`BGR`] (*red*, *green*, *blue*)
//!   - [`CMY`] (*cyan*, *magenta*, *yellow*)
//!   - [`Gray`] (*luma* / *relative luminance*)
//!   - [`HSV`] (*hue*, *saturation*, *value*)
//!   - [`HSL`] (*hue*, *saturation*, *lightness*)
//!   - [`HWB`] (*hue*, *whiteness*, *blackness*)
//!   - [`YCbCr`] (used by JPEG)
//!   - [`Matte`] (*alpha* only)
//!
//! Compositing with blending [operations] is supported for *premultiplied*
//! images with *linear* gamma.
//!
//! [alpha]: chan/trait.Alpha.html
//! [`bgr`]: bgr/index.html
//! [channel]: chan/trait.Channel.html
//! [`cmy`]: cmy/index.html
//! [color model]: trait.ColorModel.html
//! [gamma]: chan/trait.Gamma.html
//! [`gray`]: gray/index.html
//! [`hsl`]: hsl/index.html
//! [`hsv`]: hsv/index.html
//! [`hwb`]: hwb/index.html
//! [`matte`]: matte/index.html
//! [operations]: ops/index.html
//! [raster]: struct.Raster.html
//! [`rgb`]: rgb/index.html
//! [`ycbcr`]: ycc/index.html
//!
//! ### HWB Color Example
//! ```
//! use pix::hwb::SHwb8;
//! use pix::rgb::SRgb8;
//! use pix::Raster;
//!
//! let mut r = Raster::with_clear(256, 256);
//! for (y, row) in r.rows_mut(()).enumerate() {
//!     for (x, p) in row.iter_mut().enumerate() {
//!         let h = ((x + y) >> 1) as u8;
//!         let w = y.saturating_sub(x) as u8;
//!         let b = x.saturating_sub(y) as u8;
//!         *p = SHwb8::new(h, w, b);
//!     }
//! }
//! // Convert to SRgb8 pixel format
//! let raster = Raster::<SRgb8>::with_raster(&r);
//! ```
//!
//! ![Colors](https://raw.githubusercontent.com/DougLau/pix/master/res/colors.png)
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
