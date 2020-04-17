// matte.rs     Alpha matte color model.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Matte color model and types.
use crate::chan::{Ch16, Ch32, Ch8, Channel, Linear, Straight};
use crate::el::{Pix1, PixRgba, Pixel};
use crate::ColorModel;
use std::ops::Range;

/// Matte [color model].
///
/// The component is *[alpha]* only.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [color model]: ../trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matte {}

impl ColorModel for Matte {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..0;
    const ALPHA: usize = 0;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let max = P::Chan::MAX.into();
        PixRgba::<P>::new(max, max, max, Pixel::alpha(p).into())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        P::from_channels(&[chan[3]])
    }
}

/// [Matte](struct.Matte.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Matte8 = Pix1<Ch8, Matte, Straight, Linear>;

/// [Matte](struct.Matte.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Matte16 = Pix1<Ch16, Matte, Straight, Linear>;

/// [Matte](struct.Matte.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Matte32 = Pix1<Ch32, Matte, Straight, Linear>;
