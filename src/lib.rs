// lib.rs      Pix crate.
//
// Copyright (c) 2019  Douglas P Lau
//
//! Pixel and [Raster](struct.Raster.html) image manipulation crate.
//!
//! Rasters are made up of pixels in one of many possible
//! [Format](trait.Format.html)s.
//!
mod alpha;
mod channel;
mod format;
mod gamma;
mod gray;
mod mask;
mod raster;
mod rgb;
mod srgb;

pub use crate::alpha::{Alpha, Opaque, Translucent};
pub use crate::channel::{Channel, Ch8, Ch16, Ch32};
pub use crate::gray::{
    Gray, Gray8, Gray16, Gray32, GrayAlpha8, GrayAlpha16, GrayAlpha32
};
pub use crate::mask::{Mask, Mask8, Mask16, Mask32};
pub use crate::format::Format;
pub use crate::raster::Raster;
pub use crate::rgb::{Rgb, Rgb8, Rgb16, Rgb32, Rgba8, Rgba16, Rgba32};
pub use crate::srgb::{Srgb, Srgb8, Srgb16, Srgb32, Srgba8, Srgba16, Srgba32};
