// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Color models
use crate::el::{Pixel, PixRgba};
pub use crate::gray::Gray;
pub use crate::hsl::Hsl;
pub use crate::hsv::Hsv;
pub use crate::hwb::Hwb;
pub use crate::mask::Mask;
use crate::private::Sealed;
pub use crate::rgb::Rgb;
pub use crate::ycc::YCbCr;
use std::fmt::Debug;
use std::ops::Range;

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Gray], [Hsv], [Hsl], [Hwb], [YCbCr] and
/// [Mask].
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
///
/// [convert]: el/trait.Pixel.html#method.convert
/// [gray]: struct.Gray.html
/// [hsl]: struct.Hsl.html
/// [hsv]: struct.Hsv.html
/// [hwb]: struct.Hwb.html
/// [mask]: struct.Mask.html
/// [rgb]: struct.Rgb.html
/// [ycbcr]: struct.YCbCr.html
pub trait ColorModel:
    Clone + Copy + Debug + Default + PartialEq + Sealed
{
    /// Range of circular channel numbers
    const CIRCULAR: Range<usize>;

    /// Range of linear channel numbers
    const LINEAR: Range<usize>;

    /// Alpha channel number
    const ALPHA: usize;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>;

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: &[P::Chan]) -> P
    where
        P: Pixel<Model = Self>;
}
