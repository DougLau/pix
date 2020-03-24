// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Module for color model items
use crate::private::Sealed;
use crate::Channel;
use std::any::Any;

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Gray], [Hsv], [Hsl], [Hwb], [YCbCr] and
/// [Mask].
///
/// It is possible to convert from a color model to any other, using
/// [into_channels] and [from_channels].  For usage of this, see the `Pixel`
/// [convert] method.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
///
/// [convert]: trait.Pixel.html#method.convert
/// [from_channels]: trait.ColorModel.html#method.from_channels
/// [gray]: struct.Gray.html
/// [hsl]: struct.Hsl.html
/// [hsv]: struct.Hsv.html
/// [hwb]: struct.Hwb.html
/// [into_channels]: trait.ColorModel.html#method.into_channels
/// [mask]: struct.Mask.html
/// [rgb]: struct.Rgb.html
/// [ycbcr]: struct.YCbCr.html
pub trait ColorModel: Any + Sealed {
    /// Component `Channel` type
    type Chan: Channel;

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan;

    /// Convert into channels shared by types
    fn into_channels<R: ColorModel>(self) -> ([Self::Chan; 4], usize);

    /// Convert from channels shared by types
    fn from_channels<R: ColorModel>(chan: [Self::Chan; 4], alpha: usize) -> Self;
}
