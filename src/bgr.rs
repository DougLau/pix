// bgr.rs       BGR color model.
//
// Copyright (c) 2020-2024  Douglas P Lau
//
//! BGR color model and types.
use crate::ColorModel;
use crate::chan::{Ch8, Ch16, Ch32, Linear, Premultiplied, Srgb, Straight};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use std::ops::Range;

/// BGR arrangement of [RGB] [color model].
///
/// The components are *[blue]*, *[green]*, *[red]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [blue]: #method.blue
/// [color model]: ../trait.ColorModel.html
/// [green]: #method.green
/// [red]: #method.red
/// [rgb]: ../rgb/struct.Rgb.html
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bgr {}

impl Bgr {
    /// Get the *blue* component.
    ///
    /// # Example: Get BGR Blue
    /// ```
    /// use pix::bgr::{Bgr, Bgr8};
    /// use pix::chan::Ch8;
    ///
    /// let p = Bgr8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Bgr::blue(p), Ch8::new(0x93));
    /// ```
    pub fn blue<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *blue* component.
    ///
    /// # Example: Modify BGR Blue
    /// ```
    /// use pix::bgr::{Bgr, Bgr8};
    /// use pix::chan::Ch8;
    ///
    /// let mut p = Bgr8::new(0x88, 0x77, 0x66);
    /// *Bgr::blue_mut(&mut p) = 0x55.into();
    /// assert_eq!(Bgr::blue(p), Ch8::new(0x55));
    /// ```
    pub fn blue_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *green* component.
    ///
    /// # Example: BGR Green
    /// ```
    /// use pix::bgr::{Bgr, Bgr16};
    /// use pix::chan::Ch16;
    ///
    /// let p = Bgr16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Bgr::green(p), Ch16::new(0x1234));
    /// ```
    pub fn green<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get a mutable reference to the *green* component.
    ///
    /// # Example: Modify BGR Green
    /// ```
    /// use pix::bgr::{Bgr, Bgr16};
    /// use pix::chan::Ch16;
    ///
    /// let mut p = Bgr16::new(0x2000, 0x1234, 0x8000);
    /// *Bgr::green_mut(&mut p) = 0x4321.into();
    /// assert_eq!(Bgr::green(p), Ch16::new(0x4321));
    /// ```
    pub fn green_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *red* component.
    ///
    /// # Example: BGR Red
    /// ```
    /// use pix::bgr::{Bgr, Bgr32};
    /// use pix::chan::Ch32;
    ///
    /// let p = Bgr32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Bgr::red(p), Ch32::new(1.0));
    /// ```
    pub fn red<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *red* component.
    ///
    /// # Example: Modify BGR Red
    /// ```
    /// use pix::bgr::{Bgr, Bgr32};
    /// use pix::chan::Ch32;
    ///
    /// let mut p = Bgr32::new(0.25, 0.5, 1.0);
    /// *Bgr::red_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(Bgr::red(p), Ch32::new(0.75));
    /// ```
    pub fn red_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }
}

impl ColorModel for Bgr {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let red = Bgr::red(p);
        let green = Bgr::green(p);
        let blue = Bgr::blue(p);
        PixRgba::<P>::new::<P::Chan>(red, green, blue, p.alpha())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        let red = chan[0];
        let green = chan[1];
        let blue = chan[2];
        let alpha = chan[3];
        P::from_channels(&[blue, green, red, alpha])
    }
}

/// [Bgr](struct.Bgr.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgr8 = Pix3<Ch8, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgr16 = Pix3<Ch16, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgr32 = Pix3<Ch32, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html)
/// gamma [pixel](../el/trait.Pixel.html) format.
pub type Bgra8 = Pix4<Ch8, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Bgra16 = Pix4<Ch16, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Bgra32 = Pix4<Ch32, Bgr, Straight, Linear>;

/// [Bgr](struct.Bgr.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgra8p = Pix4<Ch8, Bgr, Premultiplied, Linear>;

/// [Bgr](struct.Bgr.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgra16p = Pix4<Ch16, Bgr, Premultiplied, Linear>;

/// [Bgr](struct.Bgr.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Bgra32p = Pix4<Ch32, Bgr, Premultiplied, Linear>;

/// [Bgr](struct.Bgr.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgr8 = Pix3<Ch8, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgr16 = Pix3<Ch16, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgr32 = Pix3<Ch32, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra8 = Pix4<Ch8, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra16 = Pix4<Ch16, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra32 = Pix4<Ch32, Bgr, Straight, Srgb>;

/// [Bgr](struct.Bgr.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra8p = Pix4<Ch8, Bgr, Premultiplied, Srgb>;

/// [Bgr](struct.Bgr.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra16p = Pix4<Ch16, Bgr, Premultiplied, Srgb>;

/// [Bgr](struct.Bgr.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SBgra32p = Pix4<Ch32, Bgr, Premultiplied, Srgb>;
