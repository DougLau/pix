// blend.rs     Pixel blend ops.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::Format;

/// Pixel [Format](trait.Format.html)s which can be composited.
pub trait Blend: Format {

    /// Blend pixels with an alpha mask.
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over(dst: &mut [Self], mask: &[u8], src: Self) {
        Blend::mask_over_fallback(dst, mask, src);
    }

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(dst: &mut [Self], mask: &[u8], src: Self);
}
