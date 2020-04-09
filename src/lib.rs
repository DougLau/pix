// lib.rs      Pix crate.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Pixel and raster image library.
//!
//! A [raster] is a rectangular array of [pixel]s whose format is parameterized
//! by [color model], [channel], [alpha] and [gamma] mode.
//!
//! [alpha]: chan/trait.Alpha.html
//! [channel]: chan/trait.Channel.html
//! [color model]: clr/trait.ColorModel.html
//! [gamma]: chan/trait.Gamma.html
//! [pixel]: el/trait.Pixel.html
//! [raster]: struct.Raster.html
//!
//! ### Example: Convert Raster Format
//! ```
//! use pix::{Raster, Rgba8p, SRgb8};
//!
//! let mut src = Raster::<SRgb8>::with_clear(120, 120);
//! // ... load pixels into raster
//! let dst: Raster<Rgba8p> = Raster::with_raster(&src);
//! ```
//!
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

pub mod chan;
pub mod clr;
pub mod el;
pub mod ops;
mod palette;
mod private;
mod raster;

pub use crate::clr::bgr::{
    Bgr16, Bgr32, Bgr8, Bgra16, Bgra16p, Bgra32, Bgra32p, Bgra8, Bgra8p,
    SBgr16, SBgr32, SBgr8, SBgra16, SBgra16p, SBgra32, SBgra32p, SBgra8,
    SBgra8p,
};
pub use crate::clr::gray::{
    Gray16, Gray32, Gray8, Graya16, Graya16p, Graya32, Graya32p, Graya8,
    Graya8p, SGray16, SGray32, SGray8, SGraya16, SGraya16p, SGraya32,
    SGraya32p, SGraya8, SGraya8p,
};
pub use crate::clr::hsl::{
    Hsl16, Hsl32, Hsl8, Hsla16, Hsla16p, Hsla32, Hsla32p, Hsla8, Hsla8p,
};
pub use crate::clr::hsv::{
    Hsv16, Hsv32, Hsv8, Hsva16, Hsva16p, Hsva32, Hsva32p, Hsva8, Hsva8p,
};
pub use crate::clr::hwb::{
    Hwb16, Hwb32, Hwb8, Hwba16, Hwba16p, Hwba32, Hwba32p, Hwba8, Hwba8p,
};
pub use crate::clr::matte::{Matte16, Matte32, Matte8};
pub use crate::clr::rgb::{
    Rgb16, Rgb32, Rgb8, Rgba16, Rgba16p, Rgba32, Rgba32p, Rgba8, Rgba8p,
    SRgb16, SRgb32, SRgb8, SRgba16, SRgba16p, SRgba32, SRgba32p, SRgba8,
    SRgba8p,
};
pub use crate::clr::ycc::{
    YCbCr16, YCbCr32, YCbCr8, YCbCra16, YCbCra16p, YCbCra32, YCbCra32p,
    YCbCra8, YCbCra8p,
};
pub use crate::palette::Palette;
pub use crate::raster::{Raster, Region, Rows, RowsMut};
