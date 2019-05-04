// pixel.rs     Pixel format basics.
//
// Copyright (c) 2018-2019  Douglas P Lau
//

/// Pixel format determines attributes of pixels:
/// * Color channels
/// * Bit depth
/// * Linear or gamma encoded
/// * Premultiplied alpha
///
/// These are existing formats:
/// * [Alpha](struct.Alpha.html)
/// * [Gray](struct.Gray.html)
/// * [Rgb](struct.Rgb.html)
/// * [Rgba](struct.Rgba.html)
/// * [Srgb](struct.Srgb.html)
///
pub trait PixFmt: Clone + Copy + Default {

    /// Blend pixels with an alpha mask.
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over(dst: &mut [Self], mask: &[u8], src: Self) {
        PixFmt::mask_over_fallback(dst, mask, src);
    }

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(dst: &mut [Self], mask: &[u8], src: Self);

    /// Convert a pixel slice into a u8 slice.
    ///
    /// * `pix` Slice of pixels.
    fn as_u8_slice(pix: &[Self]) -> &[u8] {
        unsafe { pix.align_to::<u8>().1 }
    }

    /// Convert a pixel slice into a mutable u8 slice.
    ///
    /// * `pix` Slice of pixels.
    fn as_u8_slice_mut(pix: &mut [Self]) -> &mut [u8] {
        unsafe { pix.align_to_mut::<u8>().1 }
    }

    /// Convert a u8 slice into a pixel slice.
    ///
    /// * `pix` Slice of u8 pixel data.
    fn as_slice(pix: &[u8]) -> &[Self] {
        unsafe { pix.align_to::<Self>().1 }
    }

    /// Convert a u8 slice into a mutable pixel slice.
    ///
    /// * `pix` Slice of u8 pixel data.
    fn as_slice_mut(pix: &mut [u8]) -> &mut [Self] {
        unsafe { pix.align_to_mut::<Self>().1 }
    }
}
