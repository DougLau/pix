// rgb.rs       Linear RGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::alpha::Alpha;
use crate::channel::Channel;
use crate::gray::Gray;
use crate::pixel::PixFmt;
use crate::rgba::Rgba;

/// Linear RGB [pixel format](trait.PixFmt.html).
///
/// The channels are *red*, *green* and *blue*.  They are encoded in linear
/// intensity.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel> {
    red: C,
    green: C,
    blue: C,
}

impl<C: Channel> From<Rgb<C>> for i32 {
    /// Get an i32 from a Rgb
    fn from(rgb: Rgb<C>) -> i32 {
        let red = Into::<u8>::into(rgb.red());
        let red = Into::<i32>::into(red) << 0;
        let green = Into::<u8>::into(rgb.green());
        let green = Into::<i32>::into(green) << 8;
        let blue = Into::<u8>::into(rgb.blue());
        let blue = Into::<i32>::into(blue) << 16;
        red | green | blue
    }
}

impl<C: Channel, H: Channel> From<Alpha<H>> for Rgb<C>
    where C: From<H>
{
    /// Get an Rgb from an Alpha
    fn from(_: Alpha<H>) -> Self {
        let v = C::full();
        Rgb::new(v, v, v)
    }
}

impl<C: Channel, H: Channel> From<Gray<H>> for Rgb<C>
    where C: From<H>
{
    /// Get an Rgb from a Gray
    fn from(c: Gray<H>) -> Self {
        let v = Into::<C>::into(c.value());
        Rgb::new(v, v, v)
    }
}

impl<C: Channel, H: Channel> From<Rgba<H>> for Rgb<C>
    where C: From<H>
{
    /// Get an Rgb from an Rgba
    fn from(rgb: Rgba<H>) -> Self {
        let r = Into::<C>::into(rgb.red());
        let g = Into::<C>::into(rgb.green());
        let b = Into::<C>::into(rgb.blue());
        let _a = Into::<C>::into(rgb.alpha());
        // FIXME: remove premultiplied alpha
        Rgb::new(r, g, b)
    }
}

impl<C: Channel> Rgb<C> {
    /// Build a color by specifying red, green and blue values.
    pub fn new(red: C, green: C, blue: C) -> Self {
        Rgb { red, green, blue }
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
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, dst: Rgb<C>, alpha: u8) -> Self {
        let r = Into::<C>::into(dst.red());
        let g = Into::<C>::into(dst.green());
        let b = Into::<C>::into(dst.blue());
        let a = Into::<C>::into(alpha);
        let red   = self.red().lerp_alpha(r, a);
        let green = self.green().lerp_alpha(g, a);
        let blue  = self.blue().lerp_alpha(b,  a);
        Rgb::new(red, green, blue)
    }
}

impl<C: Channel> PixFmt for Rgb<C> {

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
