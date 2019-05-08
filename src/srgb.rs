// srgb.rs      sRGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{Blend, Format, Rgb};
use crate::alpha::{Alpha, Opaque, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};

/// sRGB pixel [Format](trait.Format.html).
///
/// The channels are *red*, *green* and *blue*.  They are gamma-encoded.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Srgb<C: Channel, A: Alpha<C>> {
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C, H, A, B> From<Rgb<H, B>> for Srgb<C, A>
    where C: Channel, H: Channel, C: From<H>, A: From<B>, A: Alpha<C>,
          B: Alpha<H>
{
    /// Get an Srgb from an Rgb
    fn from(c: Rgb<H, B>) -> Self {
        let r = Into::<C>::into(c.red());
        let g = Into::<C>::into(c.green());
        let b = Into::<C>::into(c.blue());
        let alpha = Into::<A>::into(c.alpha());
        let a = alpha.value();
        // NOTE: gamma must be encoded after removing premultiplied alpha !!!
        Srgb {
            red: (r / a).encode_gamma(),
            green: (g / a).encode_gamma(),
            blue: (b / a).encode_gamma(),
            alpha,
        }
    }
}

impl<C, H, A, B> From<Srgb<H, B>> for Rgb<C, A>
    where C: Channel, H: Channel, C: From<H>, A: From<B>, A: Alpha<C>,
          B: Alpha<H>
{
    /// Get an Rgb from an Srgb
    fn from(srgb: Srgb<H, B>) -> Self {
        let r = Into::<C>::into(srgb.red()).decode_gamma();
        let g = Into::<C>::into(srgb.green()).decode_gamma();
        let b = Into::<C>::into(srgb.blue()).decode_gamma();
        let a = Into::<A>::into(srgb.alpha());
        Rgb::new(r, g, b, a)
    }
}

impl<C: Channel, A: Alpha<C>> Srgb<C, A> {
    /// Build a color by specifying red, green and blue values.
    pub fn new<V>(red: V, green: V, blue: V) -> Self
        where C: From<V>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::default();
        Srgb { red, green, blue, alpha }
    }
    /// Get the red channel value.
    pub fn red(self) -> C {
        self.red
    }
    /// Get the green channel value.
    pub fn green(self) -> C {
        self.green
    }
    /// Get the blue channel value.
    pub fn blue(self) -> C {
        self.blue
    }
    /// Get the alpha channel value.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C: Channel, A: Alpha<C>> Format for Srgb<C, A> { }

impl<C: Channel, A: Alpha<C>> Blend for Srgb<C, A> {

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(_dst: &mut [Self], _mask: &[u8], _src: Self) {
        warn!("Gamma-encoded pixels cannot be composited!!!");
    }
}

pub type Srgb8 = Srgb<Ch8, Opaque<Ch8>>;
pub type Srgb16 = Srgb<Ch16, Opaque<Ch16>>;
pub type Srgb32 = Srgb<Ch32, Opaque<Ch32>>;
pub type Srgba8 = Srgb<Ch8, Translucent<Ch8>>;
pub type Srgba16 = Srgb<Ch16, Translucent<Ch16>>;
pub type Srgba32 = Srgb<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert!(std::mem::size_of::<Srgb8>() == 3);
        assert!(std::mem::size_of::<Srgb16>() == 6);
        assert!(std::mem::size_of::<Srgb32>() == 12);
        assert!(std::mem::size_of::<Srgba8>() == 4);
        assert!(std::mem::size_of::<Srgba16>() == 8);
        assert!(std::mem::size_of::<Srgba32>() == 16);
    }
}
