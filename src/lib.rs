// lib.rs      Pix crate.
//
// Copyright (c) 2019  Douglas P Lau
//
//! Pixel and [Raster](struct.Raster.html) image crate.
//!
//! `Raster`s are made up of pixels in one of many possible
//! [Format](trait.Format.html)s.
//!
mod alpha;
mod channel;
mod format;
mod gamma;
mod gray;
mod mask;
mod palette;
mod raster;
mod rgb;

pub use crate::alpha::{Alpha, AlphaMode, AlphaMode2, Opaque, Translucent, Associated, Separated};
pub use crate::channel::{Ch16, Ch32, Ch8, Channel};
pub use crate::format::{Format, PixModes};
pub use crate::gamma::{GammaModeID, GammaMode, Srgb, Linear, PowerLaw, UnknownGamma};
pub use crate::gray::{
    Gray, Gray16, Gray32, Gray8, GrayAlpha16, GrayAlpha32, GrayAlpha8,
    LinearGray16, LinearGray32, LinearGray8, LinearGrayAlpha16, LinearGrayAlpha32, LinearGrayAlpha8,
    PremulGrayAlpha16, PremulGrayAlpha32, PremulGrayAlpha8, PremulLinearGrayAlpha16, PremulLinearGrayAlpha32, PremulLinearGrayAlpha8,
};
pub use crate::mask::{Mask, Mask16, Mask32, Mask8};
pub use crate::palette::Palette;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{Rgb, Rgb16, Rgb32, Rgb8, LinearRgb16, LinearRgb32, LinearRgb8, Rgba16, Rgba32, Rgba8, LinearRgba16, LinearRgba32, LinearRgba8, PremulRgba16, PremulRgba32, PremulRgba8, PremulLinearRgba16, PremulLinearRgba32, PremulLinearRgba8};
