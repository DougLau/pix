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
    SepLGray16, SepLGray32, SepLGray8, SepLGrayAlpha16, SepLGrayAlpha32,
    SepLGrayAlpha8, AssocLGrayAlpha16, AssocLGrayAlpha32, AssocLGrayAlpha8,
    AssocSGrayAlpha16, AssocSGrayAlpha32, AssocSGrayAlpha8, Gray, SepSGray16, SepSGray32, SepSGray8, SepSGrayAlpha16,
    SepSGrayAlpha32, SepSGrayAlpha8,

    Gray8, Gray16, Gray32,
    GrayAlpha8, GrayAlpha16, GrayAlpha32,

    SepSGray,
    SepLGray,
    AssocSGray,
    AssocLGray,

    SepGray,
    AssocGray,

    SGray,
    LGray,
};
pub use crate::mask::{Mask, Mask16, Mask32, Mask8};
pub use crate::palette::Palette;
pub use crate::raster::{Raster, RasterBuilder, RasterIter, Region};
pub use crate::rgb::{
    SepLRgb16, SepLRgb32, SepLRgb8, SepLRgba16, SepLRgba32,
    SepLRgba8, AssocLRgba16, AssocLRgba32, AssocLRgba8,
    AssocSRgba16, AssocSRgba32, AssocSRgba8, Rgb, SepSRgb16, SepSRgb32, SepSRgb8, SepSRgba16,
    SepSRgba32, SepSRgba8,

    Rgb8, Rgb16, Rgb32,
    Rgba8, Rgba16, Rgba32,

    SepSRgb,
    SepLRgb,
    AssocSRgb,
    AssocLRgb,

    SepRgb,
    AssocRgb,

    SRgb,
    LRgb,
};
