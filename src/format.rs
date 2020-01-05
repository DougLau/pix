// format.rs     Pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{AlphaMode, Channel, GammaModeID};

/// Pixel format determines [Channel](trait.Channel.html)s and bit depth.
///
/// * [Gray](struct.Gray.html): [Gray8](type.Gray8.html),
///   [Gray16](type.Gray16.html), [Gray32](type.Gray32.html),
///   [GrayAlpha8](type.GrayAlpha8.html), [GrayAlpha16](type.GrayAlpha16.html),
///   [GrayAlpha32](type.GrayAlpha32.html)
/// * [Mask](struct.Mask.html): [Mask8](type.Mask8.html),
///   [Mask16](type.Mask16.html), [Mask32](type.Mask32.html)
/// * [Rgb](struct.Rgb.html): [Rgb8](type.Rgb8.html), [Rgb16](type.Rgb16.html),
///   [Rgb32](type.Rgb32.html), [Rgba8](type.Rgba8.html),
///   [Rgba16](type.Rgba16.html), [Rgba32](type.Rgba32.html)
///
pub trait Format: Clone + Copy + Default + PartialEq {
    /// `Channel` type
    type Chan: Channel;

    /// Get *red*, *green*, *blue* and *alpha* `Channel`s
    fn rgba(self) -> [Self::Chan; 4];

    /// Make a pixel with given RGBA `Channel`s
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self;

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self;

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool;

    /// Encode into associated alpha from separate alpha.
    fn encode(self) -> Self;

    /// Decode into separate alpha from associated alpha.
    fn decode(self) -> Self;
}

/// Pixel modes are settings for [AlphaMode](enum.AlphaMode.html) and
/// [GammaMode](enum.GammaMode.html).
pub trait PixModes {
    /// Get the pixel format alpha mode
    fn alpha_mode() -> AlphaMode;

    /// Get the pixel format gamma mode
    fn gamma_mode() -> GammaModeID;
}
