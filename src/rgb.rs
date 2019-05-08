// rgb.rs       Linear RGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{Blend, Format};
use crate::alpha::{Alpha, Opaque, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};

/// RGB pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// The channels are *red*, *green* and *blue*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel, A: Alpha<C>> {
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C: Channel, A: Alpha<C>> From<Rgb<C, A>> for i32 {
    /// Get an i32 from an Rgb
    fn from(c: Rgb<C, A>) -> i32 {
        let red = Into::<u8>::into(c.red());
        let red = Into::<i32>::into(red) << 0;
        let green = Into::<u8>::into(c.green());
        let green = Into::<i32>::into(green) << 8;
        let blue = Into::<u8>::into(c.blue());
        let blue = Into::<i32>::into(blue) << 16;
        let alpha = Into::<u8>::into(c.alpha().value());
        let alpha = Into::<i32>::into(alpha) << 24;
        red | green | blue | alpha
    }
}

impl<C, H> From<Rgb<H, Translucent<H>>> for Rgb<C, Opaque<C>>
    where C: Channel, H: Channel, C: From<H>
{
    /// Get an Opaque Rgb from a Translucent Rgb
    fn from(c: Rgb<H, Translucent<H>>) -> Self {
        let r = Into::<C>::into(c.red());
        let g = Into::<C>::into(c.green());
        let b = Into::<C>::into(c.blue());
        let a = Into::<C>::into(c.alpha().value());
        Rgb::new(r / a, g / a, b / a, Opaque::default())
    }
}

impl<C: Channel, A: Alpha<C>> Rgb<C, A> {
    /// Build a color by specifying red, green and blue values.
    pub fn new<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
        where C: From<H>, A: From<B>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(alpha);
        // FIXME: do alpha properly and premultiply if necessary
        Rgb { red, green, blue, alpha }
    }
    /// Get the red channel.
    pub fn red(self) -> C {
        self.red
    }
    /// Get the green channel.
    pub fn green(self) -> C {
        self.green
    }
    /// Get the blue channel.
    pub fn blue(self) -> C {
        self.blue
    }
    /// Get the alpha channel.
    pub fn alpha(self) -> A {
        self.alpha
    }
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, dst: Rgb<C, A>, alpha: u8) -> Self {
        let r = Into::<C>::into(dst.red());
        let g = Into::<C>::into(dst.green());
        let b = Into::<C>::into(dst.blue());
        let da = Into::<A>::into(dst.alpha());
        let a = Into::<C>::into(alpha);
        let red = self.red().lerp_alpha(r, a);
        let green = self.green().lerp_alpha(g, a);
        let blue = self.blue().lerp_alpha(b, a);
        let alpha = self.alpha().lerp_alpha(da.value(), a);
        Rgb::new(red, green, blue, alpha)
    }
}

impl<C: Channel, A: Alpha<C>> Format for Rgb<C, A> { }

impl<C: Channel, A: Alpha<C>> Blend for Rgb<C, A> {

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(dst: &mut [Self], mask: &[u8], src: Self) {
        for (bot, m) in dst.iter_mut().zip(mask) {
            *bot = src.with_alpha_over(*bot, *m);
        }
    }
}

/// [Opaque](struct.Opaque.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>>;

/// [Opaque](struct.Opaque.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>>;

/// [Opaque](struct.Opaque.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert!(std::mem::size_of::<Rgb8>() == 3);
        assert!(std::mem::size_of::<Rgb16>() == 6);
        assert!(std::mem::size_of::<Rgb32>() == 12);
        assert!(std::mem::size_of::<Rgba8>() == 4);
        assert!(std::mem::size_of::<Rgba16>() == 8);
        assert!(std::mem::size_of::<Rgba32>() == 16);
    }
}
