// rgba.rs      Linear RGBA pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::channel::{Channel, Cu8};
use crate::pixel::PixFmt;
use crate::rgb::Rgb;

#[cfg(all(target_arch = "x86", feature = "use-simd"))]
use std::arch::x86::*;
#[cfg(all(target_arch = "x86_64", feature = "use-simd"))]
use std::arch::x86_64::*;

/// Linear RGBA [pixel format](trait.PixFmt.html).
///
/// The channels are *red*, *green*, *blue* and *alpha*.  They are encoded in
/// linear intensity, with premultiplied alpha.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgba<C: Channel> {
    red: C,
    green: C,
    blue: C,
    alpha: C,
}

impl<C: Channel> From<Rgba<C>> for i32 {
    /// Get an i32 from a Rgba
    fn from(rgba: Rgba<C>) -> i32 {
        let red = Into::<u8>::into(rgba.red());
        let red = Into::<i32>::into(red) << 0;
        let green = Into::<u8>::into(rgba.green());
        let green = Into::<i32>::into(green) << 8;
        let blue = Into::<u8>::into(rgba.blue());
        let blue = Into::<i32>::into(blue) << 16;
        let alpha = Into::<u8>::into(rgba.alpha());
        let alpha = Into::<i32>::into(alpha) << 24;
        red | green | blue | alpha
    }
}

impl<C: Channel, H: Channel> From<Rgb<H>> for Rgba<C>
    where C: From<H>
{
    /// Get an Rgba from an Rgb
    fn from(rgb: Rgb<H>) -> Self {
        let r = Into::<C>::into(rgb.red());
        let g = Into::<C>::into(rgb.green());
        let b = Into::<C>::into(rgb.blue());
        let a = C::full();
        Rgba::new(r, g, b, a)
    }
}

impl<C: Channel> Rgba<C> {
    /// Build a color by specifying red, green, blue and alpha values.
    pub fn new<V>(red: V, green: V, blue: V, alpha: V) -> Self
        where C: From<V>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = C::from(alpha);
        Rgba { red, green, blue, alpha }
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
    pub fn alpha(self) -> C {
        self.alpha
    }
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, dst: Rgba<C>, alpha: u8) -> Self {
        let r = Into::<C>::into(dst.red());
        let g = Into::<C>::into(dst.green());
        let b = Into::<C>::into(dst.blue());
        let da = Into::<C>::into(dst.alpha());
        let a = Into::<C>::into(alpha);
        let red = self.red().lerp_alpha(r, a);
        let green = self.green().lerp_alpha(g, a);
        let blue = self.blue().lerp_alpha(b, a);
        let alpha = self.alpha().lerp_alpha(da, a);
        Rgba::new(red, green, blue, alpha)
    }
}

impl PixFmt for Rgba<Cu8> {
    /// Blend pixels with an alpha mask.
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over(dst: &mut [Self], mask: &[u8], clr: Self) {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
              feature = "use-simd"))]
        {
            if is_x86_feature_detected!("ssse3") {
                let len = dst.len().min(mask.len());
                if len >= 4 {
                    unsafe { over_x86(dst, mask, clr) }
                }
                let ln = (len >> 2) << 2;
                if len > ln {
                    Self::mask_over_fallback(&mut dst[ln..], &mask[ln..], clr);
                }
                return;
            }
        }
        PixFmt::mask_over_fallback(dst, mask, clr);
    }

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

/// Composite a color with a mask.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "use-simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn over_x86(pix: &mut [Rgba<Cu8>], mask: &[u8], clr: Rgba<Cu8>) {
    let len = pix.len().min(mask.len());
    // Truncate len to multiple of 4
    let len = (len >> 2) << 2;
    let clr = _mm_set1_epi32(clr.into());
    let src = mask.as_ptr();
    let dst = pix.as_mut_ptr();
    // 4 pixels at a time
    for i in (0..len).step_by(4) {
        let off = i as isize;
        let dst = dst.offset(off) as *mut __m128i;
        let src = src.offset(off) as *const i32;
        // get 4 alpha values from src,
        // then shuffle: xxxxxxxxxxxx3210 => 3333222211110000
        let alpha = swizzle_mask_x86(_mm_set1_epi32(*src));
        // get RGBA values from dst
        let bot = _mm_loadu_si128(dst);
        // compose top over bot
        let out = over_alpha_u8x16_x86(clr, bot, alpha);
        // store blended pixels
        _mm_storeu_si128(dst, out);
    }
}

/// Swizzle alpha mask (xxxxxxxxxxxx3210 => 3333222211110000)
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "use-simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn swizzle_mask_x86(v: __m128i) -> __m128i {
    _mm_shuffle_epi8(v, _mm_set_epi8(3, 3, 3, 3,
                                     2, 2, 2, 2,
                                     1, 1, 1, 1,
                                     0, 0, 0, 0))
}

/// Composite packed u8 values using `over`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "use-simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn over_alpha_u8x16_x86(t: __m128i, b: __m128i, a: __m128i) -> __m128i {
    // Since alpha can range from 0 to 255 and (t - b) can range from -255 to
    // +255, we would need 17 bits to store the result of a multiplication.
    // Instead, shift alpha right by 1 bit (divide by 2).  Afterwards, we can
    // shift back by one less bit (in scale_i16_to_u8_x86).
    // For even lanes: b + alpha * (t - b)
    let t_even = _mm_unpacklo_epi8(t, _mm_setzero_si128());
    let b_even = _mm_unpacklo_epi8(b, _mm_setzero_si128());
    let a_even = _mm_unpacklo_epi8(a, _mm_setzero_si128());
    let a_even = _mm_srli_epi16(a_even, 1);
    let even = _mm_mullo_epi16(a_even, _mm_sub_epi16(t_even, b_even));
    let even = scale_i16_to_u8_x86(even);
    let even = _mm_add_epi16(b_even, even);
    // For odd lanes: b + alpha * (t - b)
    let t_odd = _mm_unpackhi_epi8(t, _mm_setzero_si128());
    let b_odd = _mm_unpackhi_epi8(b, _mm_setzero_si128());
    let a_odd = _mm_unpackhi_epi8(a, _mm_setzero_si128());
    let a_odd = _mm_srli_epi16(a_odd, 1);
    let odd = _mm_mullo_epi16(a_odd, _mm_sub_epi16(t_odd, b_odd));
    let odd = scale_i16_to_u8_x86(odd);
    let odd = _mm_add_epi16(b_odd, odd);
    _mm_packus_epi16(even, odd)
}

/// Scale i16 values (result of "u7" * "i9") into u8.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "use-simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn scale_i16_to_u8_x86(v: __m128i) -> __m128i {
    // To scale into a u8, we would normally divide by 255.  This is equivalent
    // to: ((v + 1) + (v >> 8)) >> 8
    // For the last right shift, we use 7 instead to simulate multiplying by
    // 2.  This is necessary because alpha was shifted right by 1 bit to allow
    // fitting 17 bits of data into epi16 lanes.
    _mm_srai_epi16(_mm_add_epi16(_mm_add_epi16(v,
                                               _mm_set1_epi16(1)),
                                 _mm_srai_epi16(v, 8)),
                   7)
}
