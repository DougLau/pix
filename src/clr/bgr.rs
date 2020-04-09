// bgr.rs       BGR color model.
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::chan::{Ch16, Ch32, Ch8, Linear, Premultiplied, Srgb, Straight};
use crate::clr::ColorModel;
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use std::ops::Range;

/// BGR arrangement of [RGB] [color model].
///
/// The components are *[blue]*, *[green]*, *[red]* and optional *[alpha]*.
///
/// [alpha]: #method.alpha
/// [blue]: #method.blue
/// [color model]: trait.ColorModel.html
/// [green]: #method.green
/// [red]: #method.red
/// [rgb]: struct.Rgb.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bgr {}

impl Bgr {
    /// Get the *blue* component.
    ///
    /// # Example: BGR Blue
    /// ```
    /// use pix::Bgr8;
    /// use pix::chan::Ch8;
    /// use pix::clr::Bgr;
    ///
    /// let p = Bgr8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Bgr::blue(p), Ch8::new(0x93));
    /// ```
    pub fn blue<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *green* component.
    ///
    /// # Example: BGR Green
    /// ```
    /// use pix::Bgr16;
    /// use pix::chan::Ch16;
    /// use pix::clr::Bgr;
    ///
    /// let p = Bgr16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Bgr::green(p), Ch16::new(0x1234));
    /// ```
    pub fn green<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *red* component.
    ///
    /// # Example: BGR Red
    /// ```
    /// use pix::Bgr32;
    /// use pix::chan::Ch32;
    /// use pix::clr::Bgr;
    ///
    /// let p = Bgr32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Bgr::red(p), Ch32::new(1.0));
    /// ```
    pub fn red<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: BGR Alpha
    /// ```
    /// use pix::Bgra8;
    /// use pix::chan::Ch8;
    /// use pix::clr::Bgr;
    ///
    /// let p = Bgra8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(Bgr::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
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
        let red = Bgr::red(p).into();
        let green = Bgr::green(p).into();
        let blue = Bgr::blue(p).into();
        let alpha = Bgr::alpha(p).into();
        PixRgba::<P>::new(red, green, blue, alpha)
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

/// [Bgr](clr/struct.Bgr.html) 8-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgr8 = Pix3<Ch8, Bgr, Straight, Linear>;
/// [Bgr](clr/struct.Bgr.html) 16-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgr16 = Pix3<Ch16, Bgr, Straight, Linear>;
/// [Bgr](clr/struct.Bgr.html) 32-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgr32 = Pix3<Ch32, Bgr, Straight, Linear>;

/// [Bgr](clr/struct.Bgr.html) 8-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra8 = Pix4<Ch8, Bgr, Straight, Linear>;
/// [Bgr](clr/struct.Bgr.html) 16-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra16 = Pix4<Ch16, Bgr, Straight, Linear>;
/// [Bgr](clr/struct.Bgr.html) 32-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra32 = Pix4<Ch32, Bgr, Straight, Linear>;

/// [Bgr](clr/struct.Bgr.html) 8-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra8p = Pix4<Ch8, Bgr, Premultiplied, Linear>;
/// [Bgr](clr/struct.Bgr.html) 16-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra16p = Pix4<Ch16, Bgr, Premultiplied, Linear>;
/// [Bgr](clr/struct.Bgr.html) 32-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Bgra32p = Pix4<Ch32, Bgr, Premultiplied, Linear>;

/// [Bgr](clr/struct.Bgr.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgr8 = Pix3<Ch8, Bgr, Straight, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgr16 = Pix3<Ch16, Bgr, Straight, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgr32 = Pix3<Ch32, Bgr, Straight, Srgb>;

/// [Bgr](clr/struct.Bgr.html) 8-bit [straight](chan/struct.Straight.html)
/// alpha [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SBgra8 = Pix4<Ch8, Bgr, Straight, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 16-bit [straight](chan/struct.Straight.html)
/// alpha [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SBgra16 = Pix4<Ch16, Bgr, Straight, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 32-bit [straight](chan/struct.Straight.html)
/// alpha [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SBgra32 = Pix4<Ch32, Bgr, Straight, Srgb>;

/// [Bgr](clr/struct.Bgr.html) 8-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgra8p = Pix4<Ch8, Bgr, Premultiplied, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 16-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgra16p = Pix4<Ch16, Bgr, Premultiplied, Srgb>;
/// [Bgr](clr/struct.Bgr.html) 32-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [sRGB](chan/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SBgra32p = Pix4<Ch32, Bgr, Premultiplied, Srgb>;
