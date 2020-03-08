// format.rs     Pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{AlphaMode, AlphaModeID, Translucent};
use crate::gamma::{GammaMode, GammaModeID};
use crate::Channel;

/// Pixel format determines bit depth ([Channel](trait.Channel.html)),
/// color components, [alpha mode](alpha/trait.AlphaMode.html),
/// and [gamma mode](gamma/trait.GammaMode.html).
///
/// The naming scheme for type aliases goes:
///
/// * `S` for [sRGB gamma](gamma/struct.SrgbGamma.html) colorspace;
///   [linear gamma](gamma/struct.LinearGamma.html) if omitted.
/// * `Gray`/`GrayAlpha`/`Mask`/`Rgb`/`Rgba` for [Gray](struct.Gray.html),
///   [Mask](struct.Mask.html), and [Rgb](struct.Rgb.html).
/// * `8`/`16`/`32` for 8-bit integer, 16-bit integer, and 32-bit floating-point
///   [channels](trait.Channel.html).
/// * `p` for [premultiplied](alpha/struct.PremultipliedAlpha.html) alpha.
///
/// The following types are defined:
///
/// * Opaque, linear gamma:
///   [Gray8](type.Gray8.html),
///   [Gray16](type.Gray16.html),
///   [Gray32](type.Gray32.html),
///   [Rgb8](type.Rgb8.html),
///   [Rgb16](type.Rgb16.html),
///   [Rgb32](type.Rgb32.html)
/// * Opaque, sRGB gamma:
///   [SGray8](type.SGray8.html),
///   [SGray16](type.SGray16.html),
///   [SGray32](type.SGray32.html),
///   [SRgb8](type.SRgb8.html),
///   [SRgb16](type.SRgb16.html),
///   [SRgb32](type.SRgb32.html)
/// * Translucent (straight alpha), linear gamma:
///   [GrayAlpha8](type.GrayAlpha8.html),
///   [GrayAlpha16](type.GrayAlpha16.html),
///   [GrayAlpha32](type.GrayAlpha32.html)
///   [Rgba8](type.Rgba8.html),
///   [Rgba16](type.Rgba16.html),
///   [Rgba32](type.Rgba32.html)
/// * Translucent (premultiplied alpha), linear gamma:
///   [GrayAlpha8p](type.GrayAlpha8p.html),
///   [GrayAlpha16p](type.GrayAlpha16p.html),
///   [GrayAlpha32p](type.GrayAlpha32p.html)
///   [Rgba8p](type.Rgba8p.html),
///   [Rgba16p](type.Rgba16p.html),
///   [Rgba32p](type.Rgba32p.html)
/// * Translucent (straight alpha), sRGB gamma:
///   [SGrayAlpha8](type.SGrayAlpha8.html),
///   [SGrayAlpha16](type.SGrayAlpha16.html),
///   [SGrayAlpha32](type.SGrayAlpha32.html)
///   [SRgba8](type.SRgba8.html),
///   [SRgba16](type.SRgba16.html),
///   [SRgba32](type.SRgba32.html)
/// * Translucent (premultiplied alpha), sRGB gamma:
///   [SGrayAlpha8p](type.SGrayAlpha8p.html),
///   [SGrayAlpha16p](type.SGrayAlpha16p.html),
///   [SGrayAlpha32p](type.SGrayAlpha32p.html)
///   [SRgba8p](type.SRgba8p.html),
///   [SRgba16p](type.SRgba16p.html),
///   [SRgba32p](type.SRgba32p.html)
/// * Alpha mask:
///   [Mask8](type.Mask8.html),
///   [Mask16](type.Mask16.html),
///   [Mask32](type.Mask32.html)
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
        // Remove premultiplied alpha
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
