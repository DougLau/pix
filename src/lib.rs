// lib.rs      Pix crate.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Pixel and [Raster](struct.Raster.html) image crate.
//!
//! `Raster`s are made up of pixels in one of many possible
//! [Format](trait.Format.html)s.
//!
pub mod alpha;
mod channel;
mod format;
pub mod gamma;
mod gray;
mod mask;
mod palette;
mod raster;
mod rgb;

pub use crate::channel::{Ch16, Ch32, Ch8, Channel};
pub use crate::format::Format;
pub use crate::gray::{
    Gray, Gray16, Gray32, Gray8,
    GrayAlpha16, GrayAlpha32, GrayAlpha8,
    GrayAlpha16p, GrayAlpha32p, GrayAlpha8p,
    SGray16, SGray32, SGray8, SGrayAlpha16, SGrayAlpha32, SGrayAlpha8,
    SGrayAlpha16p, SGrayAlpha32p, SGrayAlpha8p,
};
pub use crate::mask::{Mask, Mask16, Mask32, Mask8};
pub use crate::palette::Palette;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{
    Rgb, Rgb16, Rgb32, Rgb8, Rgba16, Rgba32, Rgba8, Rgba16p, Rgba32p, Rgba8p,
    SRgb16, SRgb32, SRgb8, SRgba16, SRgba32, SRgba8, SRgba16p, SRgba32p,
    SRgba8p,
};
