// srgb.rs      sRGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::channel::Channel;
use crate::pixel::PixFmt;
use crate::rgb::Rgb;
use crate::rgba::Rgba;

/// sRGB [pixel format](trait.PixFmt.html).
///
/// The channels are *red*, *green* and *blue*.  They are gamma-encoded.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Srgb<C: Channel> {
    red: C,
    green: C,
    blue: C,
}

impl<C: Channel, H: Channel> From<Rgb<H>> for Srgb<C>
    where C: From<H>
{
    /// Get an Srgb from an Rgb
    fn from(rgb: Rgb<H>) -> Self {
        let r = Into::<C>::into(rgb.red());
        let g = Into::<C>::into(rgb.green());
        let b = Into::<C>::into(rgb.blue());
        Srgb {
            red: r.encode_gamma(),
            green: g.encode_gamma(),
            blue: b.encode_gamma(),
        }
    }
}

impl<C: Channel, H: Channel> From<Rgba<H>> for Srgb<C>
    where C: From<H>
{
    /// Get an Srgb from an Rgba
    fn from(rgba: Rgba<H>) -> Self {
        let r = Into::<C>::into(rgba.red());
        let g = Into::<C>::into(rgba.green());
        let b = Into::<C>::into(rgba.blue());
        let a = Into::<C>::into(rgba.alpha());
        // FIXME: remove premultiplied alpha
        // NOTE: gamma must be encoded last !!!
        Srgb {
            red: (r / a).encode_gamma(),
            green: (g / a).encode_gamma(),
            blue: (b / a).encode_gamma(),
        }
    }
}

impl<C: Channel> Srgb<C> {
    /// Build a color by specifying red, green and blue values.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        let red = red.into();
        let green = green.into();
        let blue = blue.into();
        Srgb { red, green, blue }
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
}

impl<C: Channel> PixFmt for Srgb<C> {

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(_dst: &mut [Self], _mask: &[u8], _src: Self) {
        warn!("Gamma-encoded pixels cannot be composited!!!");
    }
}
