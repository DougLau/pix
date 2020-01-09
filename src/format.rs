// format.rs     Pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::{
    AlphaMode, AlphaModeID, Channel, GammaMode, GammaModeID, Translucent,
};

/// Pixel format determines bit depth ([Channel](trait.Channel.html)),
/// color components, alpha mode, and gamma mode.
///
/// The naming scheme for type aliases goes:
///
/// * `Sep`/`Assoc` for [separated](struct.SeparatedAlpha.html) and
///   [associated](struct.AssociatedAlpha.html) alpha.
/// * `L`/`S` for [linear gamma colorspace](struct.LinearGamma.html) and
///   [sRGB gamma colorspace](struct.SrgbGamma.html).
/// * `Gray`/`Mask`/`Rgb` for [Gray](struct.Gray.html),
///   [Mask](struct.Mask.html), and [Rgb](struct.Rgb.html).
/// * `8`/`16`/`32` for 8-bit integer, 16-bit integer, and 32-bit floating-point
///   pixel formats.
///
/// The following types are defined:
///
/// * [Gray](struct.Gray.html): [Gray8](type.Gray8.html),
///   [Gray16](type.Gray16.html), [Gray32](type.Gray32.html),
///   [GrayAlpha8](type.GrayAlpha8.html), [GrayAlpha16](type.GrayAlpha16.html),
///   [GrayAlpha32](type.GrayAlpha32.html)
/// * [SGray](type.SGray.html)
/// * [SepSGray](type.SepSGray.html): [SepSGray8](type.SepSGray8.html),
///   [SepSGray16](type.SepSGray16.html), [SepSGray32](type.SepSGray32.html),
///   [SepSGrayAlpha8](type.SepSGrayAlpha8.html),
///   [SepSGrayAlpha16](type.SepSGrayAlpha16.html),
///   [SepSGrayAlpha32](type.SepSGrayAlpha32.html)
/// * [AssocSGray](type.AssocSGray.html):
///   [AssocSGrayAlpha8](type.AssocSGrayAlpha8.html),
///   [AssocSGrayAlpha16](type.AssocSGrayAlpha16.html),
///   [AssocSGrayAlpha32](type.AssocSGrayAlpha32.html)
/// * [LGray](type.LGray.html)
/// * [SepLGray](type.SepLGray.html): [SepLGray8](type.SepLGray8.html),
///   [SepLGray16](type.SepLGray16.html), [SepLGray32](type.SepLGray32.html),
///   [SepLGrayAlpha8](type.SepLGrayAlpha8.html),
///   [SepLGrayAlpha16](type.SepLGrayAlpha16.html),
///   [SepLGrayAlpha32](type.SepLGrayAlpha32.html)
/// * [AssocLGray](type.AssocLGray.html):
///   [AssocLGrayAlpha8](type.AssocLGrayAlpha8.html),
///   [AssocLGrayAlpha16](type.AssocLGrayAlpha16.html),
///   [AssocLGrayAlpha32](type.AssocLGrayAlpha32.html)
/// * [Mask](struct.Mask.html): [Mask8](type.Mask8.html),
///   [Mask16](type.Mask16.html), [Mask32](type.Mask32.html)
/// * [Rgb](struct.Rgb.html): [Rgb8](type.Rgb8.html),
///   [Rgb16](type.Rgb16.html), [Rgb32](type.Rgb32.html),
///   [Rgba8](type.Rgba8.html), [Rgba16](type.Rgba16.html),
///   [Rgba32](type.Rgba32.html)
/// * [SRgb](type.SRgb.html)
/// * [SepSRgb](type.SepSRgb.html): [SepSRgb8](type.SepSRgb8.html),
///   [SepSRgb16](type.SepSRgb16.html), [SepSRgb32](type.SepSRgb32.html),
///   [SepSRgba8](type.SepSRgba8.html),
///   [SepSRgba16](type.SepSRgba16.html),
///   [SepSRgba32](type.SepSRgba32.html)
/// * [AssocSRgb](type.AssocSRgb.html):
///   [AssocSRgba8](type.AssocSRgba8.html),
///   [AssocSRgba16](type.AssocSRgba16.html),
///   [AssocSRgba32](type.AssocSRgba32.html)
/// * [LRgb](type.LRgb.html)
/// * [SepLRgb](type.SepLRgb.html): [SepLRgb8](type.SepLRgb8.html),
///   [SepLRgb16](type.SepLRgb16.html), [SepLRgb32](type.SepLRgb32.html),
///   [SepLRgba8](type.SepLRgba8.html),
///   [SepLRgba16](type.SepLRgba16.html),
///   [SepLRgba32](type.SepLRgba32.html)
/// * [AssocLRgb](type.AssocLRgb.html):
///   [AssocLRgba8](type.AssocLRgba8.html),
///   [AssocLRgba16](type.AssocLRgba16.html),
///   [AssocLRgba32](type.AssocLRgba32.html)
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
