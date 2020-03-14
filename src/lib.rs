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
mod pixel;
pub mod gamma;
mod gray;
mod mask;
mod model;
mod palette;
mod private;
mod raster;
mod rgb;
mod hsv;
mod ycc;

pub use crate::channel::{Ch16, Ch32, Ch8, Channel};
pub use crate::pixel::Pixel;
pub use crate::gray::{
    Gray, Gray16, Gray32, Gray8, GrayAlpha16, GrayAlpha16p, GrayAlpha32,
    GrayAlpha32p, GrayAlpha8, GrayAlpha8p, SGray16, SGray32, SGray8,
    SGrayAlpha16, SGrayAlpha16p, SGrayAlpha32, SGrayAlpha32p, SGrayAlpha8,
    SGrayAlpha8p,
};
pub use crate::mask::{Mask, Mask16, Mask32, Mask8};
pub use crate::model::ColorModel;
pub use crate::palette::Palette;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{
    Rgb, Rgb16, Rgb32, Rgb8, Rgba16, Rgba16p, Rgba32, Rgba32p, Rgba8, Rgba8p,
    SRgb16, SRgb32, SRgb8, SRgba16, SRgba16p, SRgba32, SRgba32p, SRgba8,
    SRgba8p,
};
pub use crate::hsv::{
    Hsv, Hsv16, Hsv32, Hsv8, Hsva16, Hsva16p, Hsva32, Hsva32p, Hsva8, Hsva8p,
};
pub use crate::ycc::{
    YCbCr, YCbCr16, YCbCr32, YCbCr8, YCbCrAlpha16, YCbCrAlpha16p, YCbCrAlpha32,
    YCbCrAlpha32p, YCbCrAlpha8, YCbCrAlpha8p,
};
