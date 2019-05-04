// gray.rs      Linear grayscale pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::channel::Channel;
use crate::pixel::PixFmt;
use crate::rgb::Rgb;

/// Linear grayscale [pixel format](trait.PixFmt.html).
///
/// There is a single gray channel with linear intensity.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Gray<C: Channel> {
    value: C,
}

impl<C: Channel, H: Channel> From<Rgb<H>> for Gray<C>
    where C: From<H>
{
    /// Get a gray from an rgb
    fn from(rgb: Rgb<H>) -> Self {
        let r = Into::<C>::into(rgb.red());
        let g = Into::<C>::into(rgb.green());
        let b = Into::<C>::into(rgb.blue());
        // FIXME: adjust luminance based on channels
        let v: C = r.max(g).max(b);
        Gray::new(v)
    }
}

impl<C: Channel> Gray<C> {
    /// Build a gray value.
    pub fn new(value: C) -> Self {
        Gray { value }
    }
    /// Get the component value.
    pub fn value(self) -> C {
        self.value
    }
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, dst: Gray<C>, alpha: u8) -> Self {
        let v = Into::<C>::into(dst.value());
        let a = Into::<C>::into(alpha);
        let value = self.value.lerp_alpha(v, a);
        Gray::new(value)
    }
}

impl<C: Channel> PixFmt for Gray<C> {

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
