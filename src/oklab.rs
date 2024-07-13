// oklab.rs        Oklab color model.
//
// Copyright (c) 2023-2024  Douglas P Lau
//
//! [Oklab] color model and types.
//!
//! [OkLab]: https://bottosson.github.io/posts/oklab/
#![allow(clippy::excessive_precision)]

use crate::chan::{Ch16, Ch32, Ch8, Channel, Linear, Premultiplied, Straight};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::ColorModel;
use std::ops::Range;

/// [Oklab] [color model]
///
/// The components are *[L]*, *[a]*, *[b]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [a]: #method.a
/// [b]: #method.b
/// [color model]: ../trait.ColorModel.html
/// [L]: #method.l
/// [OkLab]: https://bottosson.github.io/posts/oklab/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Oklab {}

impl Oklab {
    /// Get the *L* component (perceived lightness).
    ///
    /// # Example: Oklab *L*
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::oklab::{Oklab, Oklab32};
    ///
    /// let p = Oklab32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Oklab::l(p), Ch32::new(0.25));
    /// ```
    pub fn l<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *L* component.
    ///
    /// # Example: Modify Oklab *L*
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::oklab::{Oklab, Oklab32};
    ///
    /// let mut p = Oklab32::new(0.25, 0.5, 1.0);
    /// *Oklab::l_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(Oklab::l(p), Ch32::new(0.75));
    /// ```
    pub fn l_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *a* component (green/red).
    ///
    /// # Example: Oklab *a*
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::oklab::{Oklab, Oklab16};
    ///
    /// let p = Oklab16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Oklab::a(p), Ch16::new(0x1234));
    /// ```
    pub fn a<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get a mutable reference to the *a* component.
    ///
    /// # Example: Modify Oklab *a*
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::oklab::{Oklab, Oklab16};
    ///
    /// let mut p = Oklab16::new(0x2000, 0x1234, 0x8000);
    /// *Oklab::a_mut(&mut p) = 0x4321.into();
    /// assert_eq!(Oklab::a(p), Ch16::new(0x4321));
    /// ```
    pub fn a_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *b* component (blue/yellow).
    ///
    /// # Example: Oklab *b*
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::oklab::{Oklab, Oklab8};
    ///
    /// let p = Oklab8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Oklab::b(p), Ch8::new(0xA0));
    /// ```
    pub fn b<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *b* component.
    ///
    /// # Example: Modify Oklab *b*
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::oklab::{Oklab, Oklab8};
    ///
    /// let mut p = Oklab8::new(0x88, 0x77, 0x66);
    /// *Oklab::b_mut(&mut p) = 0x55.into();
    /// assert_eq!(Oklab::b(p), Ch8::new(0x55));
    /// ```
    pub fn b_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }
}

impl ColorModel for Oklab {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let pl = Self::l(p).to_f32();
        let pa = Self::a(p).to_f32();
        let pb = Self::b(p).to_f32();

        let l_ = pl + 0.3963377774 * pa + 0.2158037573 * pb;
        let m_ = pl - 0.1055613458 * pa - 0.0638541728 * pb;
        let s_ = pl - 0.0894841775 * pa - 1.2914855480 * pb;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let red = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let green = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let blue = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;
        PixRgba::<P>::new(red, green, blue, p.alpha().to_f32())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        let red = chan[0].to_f32();
        let green = chan[1].to_f32();
        let blue = chan[2].to_f32();
        let alpha = chan[3];

        let l = 0.4122214708 * red + 0.5363325363 * green + 0.0514459929 * blue;
        let m = 0.2119034982 * red + 0.6806995451 * green + 0.1073969566 * blue;
        let s = 0.0883024619 * red + 0.2817188376 * green + 0.6299787005 * blue;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        let pl = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let pa = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let pb = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

        P::from_channels(&[pl.into(), pa.into(), pb.into(), alpha])
    }
}

/// [Oklab](struct.Oklab.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklab8 = Pix3<Ch8, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklab16 = Pix3<Ch16, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklab32 = Pix3<Ch32, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 8-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba8 = Pix4<Ch8, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 16-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba16 = Pix4<Ch16, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 32-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba32 = Pix4<Ch32, Oklab, Straight, Linear>;

/// [Oklab](struct.Oklab.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba8p = Pix4<Ch8, Oklab, Premultiplied, Linear>;

/// [Oklab](struct.Oklab.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba16p = Pix4<Ch16, Oklab, Premultiplied, Linear>;

/// [Oklab](struct.Oklab.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Oklaba32p = Pix4<Ch32, Oklab, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    #[test]
    fn oklab_to_rgb() {
        // TODO
    }
}
