// format.rs     Pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::{
    AlphaMode, AlphaModeID, Channel, GammaMode, GammaModeID, Translucent,
};

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
pub trait Format:
    Clone + Copy + Default + PartialEq + AlphaMode + GammaMode
{
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

    /// Convert a pixel from one `Format` to another
    ///
    /// * `p` Source pixel to convert.
    fn convert<C, F>(self) -> F
    where
        F: Format<Chan = C>,
        C: Channel + From<Self::Chan>,
    {
        let rgba = self.rgba();
        // Decode gamma
        let rgba = if <Self as GammaMode>::ID != <F as GammaMode>::ID {
            [
                <Self as GammaMode>::decode(rgba[0]),
                <Self as GammaMode>::decode(rgba[1]),
                <Self as GammaMode>::decode(rgba[2]),
                rgba[3],
            ]
        } else {
            rgba
        };
        // Remove associated alpha
        let rgba = if <Self as AlphaMode>::ID != <F as AlphaMode>::ID {
            [
                <Self as AlphaMode>::decode(rgba[0], Translucent::new(rgba[3])),
                <Self as AlphaMode>::decode(rgba[1], Translucent::new(rgba[3])),
                <Self as AlphaMode>::decode(rgba[2], Translucent::new(rgba[3])),
                rgba[3],
            ]
        } else {
            rgba
        };
        // Convert bit depth
        let red = C::from(rgba[0]);
        let green = C::from(rgba[1]);
        let blue = C::from(rgba[2]);
        let alpha = C::from(rgba[3]);
        // Apply alpha (only if source alpha mode was set)
        let rgba = if <F as AlphaMode>::ID != <Self as AlphaMode>::ID
            && <F as AlphaMode>::ID != AlphaModeID::UnknownAlpha
        {
            [
                <F as AlphaMode>::encode(red, Translucent::new(alpha)),
                <F as AlphaMode>::encode(green, Translucent::new(alpha)),
                <F as AlphaMode>::encode(blue, Translucent::new(alpha)),
                alpha,
            ]
        } else {
            [red, green, blue, alpha]
        };
        // Encode gamma (only if source gamma mode was set)
        let rgba = if <F as GammaMode>::ID != <Self as GammaMode>::ID
            && <F as GammaMode>::ID != GammaModeID::UnknownGamma
        {
            [
                <F as GammaMode>::encode(rgba[0]),
                <F as GammaMode>::encode(rgba[1]),
                <F as GammaMode>::encode(rgba[2]),
                rgba[3],
            ]
        } else {
            rgba
        };
        F::with_rgba(rgba)
    }
}
