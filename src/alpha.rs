// alpha.rs     Alpha pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::channel::Channel;
use crate::pixel::PixFmt;

/// Linear alpha [pixel format](trait.PixFmt.html).
///
/// This pixel format is for alpha channel only.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Alpha<C: Channel> {
    value: C,
}

impl<C: Channel> Alpha<C> {
    /// Build an alpha value.
    pub fn new<V>(value: V) -> Self
        where C: From<V>
    {
        let value = C::from(value);
        Alpha { value }
    }
    /// Get the component value.
    pub fn value(self) -> C {
        self.value
    }
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, dst: Alpha<C>, alpha: u8) -> Self {
        let v = Into::<C>::into(dst.value());
        let a = Into::<C>::into(alpha);
        let value = self.value.lerp_alpha(v, a);
        Alpha::new(value)
    }
}

impl<C: Channel> PixFmt for Alpha<C> {

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
