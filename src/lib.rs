// lib.rs      Pix crate.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Pixel and raster image library.
//!
//! A [raster] is a rectangular array of [pixel]s whose format is parameterized
//! by [color model], [channel], [alpha mode] and [gamma mode].
//!
//! [alpha mode]: alpha/trait.Mode.html
//! [channel]: trait.Channel.html
//! [color model]: trait.ColorModel.html
//! [gamma mode]: gamma/trait.Mode.html
//! [pixel]: trait.Pixel.html
//! [raster]: struct.Raster.html
//!
//! ### Example: Convert Raster Format
//! ```
//! # use pix::*;
//! let mut src = RasterBuilder::<SRgb8>::new().with_clear(120, 120);
//! // ... load pixels into raster
//! let dst: Raster<Rgba8p> = RasterBuilder::new().with_raster(&src);
//! ```
//!
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

pub mod alpha;
mod channel;
pub mod gamma;
mod gray;
mod hsl;
mod hsv;
mod hue;
mod hwb;
mod mask;
mod model;
mod palette;
pub mod pixel;
mod private;
mod raster;
mod rgb;
mod ycc;

pub use crate::channel::{Ch16, Ch32, Ch8, Channel};
pub use crate::gray::{
    Gray16, Gray32, Gray8, GrayModel, Graya16, Graya16p, Graya32, Graya32p,
    Graya8, Graya8p, SGray16, SGray32, SGray8, SGraya16, SGraya16p, SGraya32,
    SGraya32p, SGraya8, SGraya8p,
};
pub use crate::hsl::{
    Hsl16, Hsl32, Hsl8, HslModel, Hsla16, Hsla16p, Hsla32, Hsla32p, Hsla8,
    Hsla8p,
};
pub use crate::hsv::{
    Hsv16, Hsv32, Hsv8, HsvModel, Hsva16, Hsva16p, Hsva32, Hsva32p, Hsva8,
    Hsva8p,
};
pub use crate::hwb::{
    Hwb16, Hwb32, Hwb8, HwbModel, Hwba16, Hwba16p, Hwba32, Hwba32p, Hwba8,
    Hwba8p,
};
pub use crate::mask::{Mask16, Mask32, Mask8, MaskModel};
pub use crate::model::ColorModel;
pub use crate::palette::Palette;
#[doc(inline)]
pub use crate::pixel::Pixel;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{
    Rgb16, Rgb32, Rgb8, RgbModel, Rgba16, Rgba16p, Rgba32, Rgba32p, Rgba8,
    Rgba8p, SRgb16, SRgb32, SRgb8, SRgba16, SRgba16p, SRgba32, SRgba32p,
    SRgba8, SRgba8p,
};
pub use crate::ycc::{
    YCbCr16, YCbCr32, YCbCr8, YCbCrModel, YCbCra16, YCbCra16p, YCbCra32,
    YCbCra32p, YCbCra8, YCbCra8p,
};
