// xyz.rs        XYZ color model.
//
// Copyright (c) 2023-2024  Douglas P Lau
//
//! CIE 1931 [XYZ] color model and types.
//!
//! [XYZ]: https://en.wikipedia.org/wiki/CIE_1931_color_space#Definition_of_the_CIE_XYZ_color_space
#![allow(clippy::excessive_precision)]

use crate::ColorModel;
use crate::chan::{Ch8, Ch16, Ch32, Channel, Linear, Premultiplied, Straight};
use crate::el::{Pix, PixRgba, Pixel};
use std::ops::Range;

/// [Xyz] [color model] with D65 white point.
///
/// The components are *[X]*, *[Y]*, *[Z]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [color model]: ../trait.ColorModel.html
/// [x]: #method.x
/// [y]: #method.y
/// [z]: #method.z
/// [XYZ]: https://en.wikipedia.org/wiki/CIE_1931_color_space#Definition_of_the_CIE_XYZ_color_space
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Xyz {}

impl Xyz {
    /// Get the *X* component.
    ///
    /// # Example: Xyz *X*
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::xyz::{Xyz, Xyz32};
    ///
    /// let p = Xyz32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Xyz::x(p), Ch32::new(0.25));
    /// ```
    pub fn x<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<0>()
    }

    /// Get a mutable reference to the *X* component.
    ///
    /// # Example: Modify Xyz *X*
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::xyz::{Xyz, Xyz32};
    ///
    /// let mut p = Xyz32::new(0.25, 0.5, 1.0);
    /// *Xyz::x_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(Xyz::x(p), Ch32::new(0.75));
    /// ```
    pub fn x_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<0>()
    }

    /// Get the *Y* component.
    ///
    /// # Example: Xyz *Y*
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::xyz::{Xyz, Xyz16};
    ///
    /// let p = Xyz16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Xyz::y(p), Ch16::new(0x1234));
    /// ```
    pub fn y<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<1>()
    }

    /// Get a mutable reference to the *Y* component.
    ///
    /// # Example: Modify Xyz *Y*
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::xyz::{Xyz, Xyz16};
    ///
    /// let mut p = Xyz16::new(0x2000, 0x1234, 0x8000);
    /// *Xyz::y_mut(&mut p) = 0x4321.into();
    /// assert_eq!(Xyz::y(p), Ch16::new(0x4321));
    /// ```
    pub fn y_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<1>()
    }

    /// Get the *Z* component.
    ///
    /// # Example: Xyz *Z*
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::xyz::{Xyz, Xyz8};
    ///
    /// let p = Xyz8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Xyz::z(p), Ch8::new(0xA0));
    /// ```
    pub fn z<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<2>()
    }

    /// Get a mutable reference to the *Z* component.
    ///
    /// # Example: Modify Xyz *Z*
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::xyz::{Xyz, Xyz8};
    ///
    /// let mut p = Xyz8::new(0x88, 0x77, 0x66);
    /// *Xyz::z_mut(&mut p) = 0x55.into();
    /// assert_eq!(Xyz::z(p), Ch8::new(0x55));
    /// ```
    pub fn z_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<2>()
    }
}

impl ColorModel for Xyz {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let px = Self::x(p).to_f32();
        let py = Self::y(p).to_f32();
        let pz = Self::z(p).to_f32();

        let red = px * 3.2406 + py * -1.5372 + pz * -0.4986;
        let green = px * -0.9689 + py * 1.8758 + pz * 0.0415;
        let blue = px * 0.0557 + py * -0.2040 + pz * 1.0570;

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

        let x = red * 0.4124 + green * 0.3576 + blue * 0.1805;
        let y = red * 0.2126 + green * 0.7152 + blue * 0.0722;
        let z = red * 0.0193 + green * 0.1192 + blue * 0.9505;

        P::from_channels(&[x.into(), y.into(), z.into(), alpha])
    }
}

/// [Xyz](struct.Xyz.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyz8 = Pix<3, Ch8, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyz16 = Pix<3, Ch16, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyz32 = Pix<3, Ch32, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 8-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza8 = Pix<4, Ch8, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 16-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza16 = Pix<4, Ch16, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 32-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza32 = Pix<4, Ch32, Xyz, Straight, Linear>;

/// [Xyz](struct.Xyz.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza8p = Pix<4, Ch8, Xyz, Premultiplied, Linear>;

/// [Xyz](struct.Xyz.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza16p = Pix<4, Ch16, Xyz, Premultiplied, Linear>;

/// [Xyz](struct.Xyz.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Xyza32p = Pix<4, Ch32, Xyz, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    #[test]
    fn xyz_to_rgb() {
        // TODO
    }
}
