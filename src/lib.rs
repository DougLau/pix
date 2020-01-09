// lib.rs      Pix crate.
//
// Copyright (c) 2019  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
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

pub use crate::alpha::{
    Alpha, AlphaMode, AlphaModeID, AssociatedAlpha, Opaque, SeparatedAlpha,
    Translucent,
};
pub use crate::channel::{Ch16, Ch32, Ch8, Channel};
pub use crate::format::Format;
pub use crate::gamma::{
    GammaMode, GammaModeID, LinearGamma, PowerLawGamma, SrgbGamma,
};
pub use crate::gray::{
    AssocGray, AssocLGray, AssocLGrayAlpha16, AssocLGrayAlpha32,
    AssocLGrayAlpha8, AssocSGray, AssocSGrayAlpha16, AssocSGrayAlpha32,
    AssocSGrayAlpha8, Gray, Gray16, Gray32, Gray8, GrayAlpha16, GrayAlpha32,
    GrayAlpha8, LGray, SGray, SepGray, SepLGray, SepLGray16, SepLGray32,
    SepLGray8, SepLGrayAlpha16, SepLGrayAlpha32, SepLGrayAlpha8, SepSGray,
    SepSGray16, SepSGray32, SepSGray8, SepSGrayAlpha16, SepSGrayAlpha32,
    SepSGrayAlpha8,
};
pub use crate::mask::{Mask, Mask16, Mask32, Mask8};
pub use crate::palette::Palette;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{
    AssocLRgb, AssocLRgba16, AssocLRgba32, AssocLRgba8, AssocRgb, AssocSRgb,
    AssocSRgba16, AssocSRgba32, AssocSRgba8, LRgb, Rgb, Rgb16, Rgb32, Rgb8,
    Rgba16, Rgba32, Rgba8, SRgb, SepLRgb, SepLRgb16, SepLRgb32, SepLRgb8,
    SepLRgba16, SepLRgba32, SepLRgba8, SepRgb, SepSRgb, SepSRgb16, SepSRgb32,
    SepSRgb8, SepSRgba16, SepSRgba32, SepSRgba8,
};
