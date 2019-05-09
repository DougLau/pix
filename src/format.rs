// format.rs     Pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//

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
/// * [Srgb](struct.Srgb.html)
///
pub trait Format: Clone + Copy + Default { }
