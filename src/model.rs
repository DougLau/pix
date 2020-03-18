// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Module for color model items
use crate::private::Sealed;
use crate::Channel;

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Gray], [Hsv], [Hsl] and [Mask].
///
/// It is possible to convert from a color model to any other, using [to_rgba]
/// and [with_rgba].  For usage of this, see the `Pixel` [convert] method.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
///
/// [convert]: trait.Pixel.html#method.convert
/// [gray]: struct.Gray.html
/// [hsl]: struct.Hsl.html
/// [hsv]: struct.Hsv.html
/// [mask]: struct.Mask.html
/// [rgb]: struct.Rgb.html
/// [to_rgba]: trait.ColorModel.html#method.to_rgba
/// [with_rgba]: trait.ColorModel.html#method.with_rgba
pub trait ColorModel: Sealed {

    /// Component `Channel` type
    type Chan: Channel;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan];

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan;

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4];

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self;
}
