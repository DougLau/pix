// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Color models
pub use crate::clr::{Gray, Hsl, Hsv, Hwb, Mask, Rgb, YCbCr};
use crate::el::{PixRgba, Pixel};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Range;

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Bgr], [Gray], [Hsv], [Hsl], [Hwb],
/// [YCbCr] and [Mask].
///
/// [bgr]: struct.Bgr.html
/// [convert]: el/trait.Pixel.html#method.convert
/// [gray]: struct.Gray.html
/// [hsl]: struct.Hsl.html
/// [hsv]: struct.Hsv.html
/// [hwb]: struct.Hwb.html
/// [mask]: struct.Mask.html
/// [rgb]: struct.Rgb.html
/// [ycbcr]: struct.YCbCr.html
pub trait ColorModel: Clone + Copy + Debug + Default + PartialEq + Any {
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
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>;
}
