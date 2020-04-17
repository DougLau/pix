// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Color models
use crate::el::{PixRgba, Pixel};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Range;

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Bgr], [Gray], [Hsv], [Hsl], [Hwb],
/// [YCbCr] and [Matte].
///
/// [bgr]: bgr/struct.Bgr.html
/// [convert]: el/trait.Pixel.html#method.convert
/// [gray]: gray/struct.Gray.html
/// [hsl]: hsl/struct.Hsl.html
/// [hsv]: hsv/struct.Hsv.html
/// [hwb]: hwb/struct.Hwb.html
/// [matte]: matte/struct.Matte.html
/// [rgb]: rgb/struct.Rgb.html
/// [ycbcr]: ycc/struct.YCbCr.html
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
