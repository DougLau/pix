// mask.rs      Alpha mask color model.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::chan::{Ch16, Ch32, Ch8, Channel, Linear, Straight};
use crate::clr::ColorModel;
use crate::el::{Pix1, PixRgba, Pixel};
use std::ops::Range;

/// Mask [color model].
///
/// The component is *alpha* only.
///
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mask {}

impl Mask {
    /// Get the *alpha* component.
    ///
    /// # Example: Mask Alpha
    /// ```
    /// # use pix::*;
    /// # use pix::chan::Ch8;
    /// # use pix::clr::Mask;
    /// let p = Mask8::new(0x94);
    /// assert_eq!(Mask::alpha(p), Ch8::new(0x94));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }
}

impl ColorModel for Mask {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..0;
    const ALPHA: usize = 0;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let max = P::Chan::MAX.into();
        PixRgba::<P>::new(max, max, max, Self::alpha(p).into())
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

/// [Mask](clr/struct.Mask.html) 8-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask8 = Pix1<Ch8, Mask, Straight, Linear>;

/// [Mask](clr/struct.Mask.html) 16-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask16 = Pix1<Ch16, Mask, Straight, Linear>;

/// [Mask](clr/struct.Mask.html) 32-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask32 = Pix1<Ch32, Mask, Straight, Linear>;
