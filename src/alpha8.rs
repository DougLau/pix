// alpha8.rs     8-bit alpha pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use pixel::{PixFmt, lerp_u8};

/// 8-bit alpha [pixel format](trait.PixFmt.html).
///
/// This pixel format is for 8-bit alpha channel only.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Alpha8 {
    value: u8,
}

impl Alpha8 {
    /// Build a alpha8 value.
    pub fn new(value: u8) -> Self {
        Alpha8 { value }
    }
    /// Get the component value.
    pub fn value(self) -> u8 {
        self.value
    }
    /// Composite the color with another, using "over".
    fn over_alpha(self, bot: Alpha8, alpha: u8) -> Self {
        let value = lerp_u8(self.value(), bot.value(), alpha);
        Alpha8::new(value)
    }
}

impl PixFmt for Alpha8 {
    /// Blend pixels with an alpha mask.
    ///
    /// * `pix` Slice of pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over(pix: &mut [Self], mask: &[u8], clr: Self) {
        debug_assert_eq!(pix.len(), mask.len());
        over_fallback(pix, mask, clr);
    }
}

/// Composite a color with a mask (slow fallback).
fn over_fallback(pix: &mut [Alpha8], mask: &[u8], clr: Alpha8) {
    for (bot, m) in pix.iter_mut().zip(mask) {
        *bot = clr.over_alpha(*bot, *m);
    }
}
