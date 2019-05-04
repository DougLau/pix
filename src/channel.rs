// channel.rs       Color channels
//
// Copyright (c) 2019  Douglas P Lau
//
use std::ops::{Div, Mul};
use crate::gamma::Gamma;

/// Color channel trait
pub trait Channel: Copy + Default + Mul<Output=Self> + Div<Output=Self> +
    From<u8> + Into<u8> + Gamma
{
    /// Get max of two channel values
    fn max(self, rhs: Self) -> Self;

    /// Get channel value with full intensity
    fn full() -> Self;

    /// Linear interpolation with alpha
    fn lerp_alpha(self, dest: Self, alpha: Self) -> Self;
}

/// Unsigned 8-bit color channel
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cu8 {
    pub value: u8,
}

impl From<u8> for Cu8 {
    fn from(value: u8) -> Self {
        Cu8 { value }
    }
}

impl From<Cu8> for u8 {
    fn from(c: Cu8) -> u8 {
        c.value
    }
}

impl Mul for Cu8 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // FIXME
        Cu8 { value: self.value * rhs.value }
    }
}

impl Div for Cu8 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value > 0 {
            let ss = (self.value as u32) << 8;
            let rr = rhs.value as u32;
            let value = (ss / rr).min(255) as u8;
            Cu8 { value }
        } else {
            Cu8 { value: 0 }
        }
    }
}

impl Gamma for Cu8 {
    /// Encode a gamma value from linear intensity
    fn encode_gamma(self) -> Self {
        Cu8 { value: self.value.encode_gamma() }
    }
    /// Decode a gamma value into linear intensity
    fn decode_gamma(self) -> Self {
        Cu8 { value: self.value.decode_gamma() }
    }
}

impl Channel for Cu8 {
    /// Get max of two channel values
    fn max(self, rhs: Self) -> Self {
        Cu8 { value: self.value.max(rhs.value) }
    }
    /// Get channel value with full intensity
    fn full() -> Self {
        Cu8 { value: 0xFF }
    }
    /// Linear interpolation
    #[inline]
    fn lerp_alpha(self, dest: Self, alpha: Self) -> Self {
        // NOTE: Alpha blending euqation is: `alpha * top + (1 - alpha) * bot`
        //       This is equivalent to lerp: `bot + alpha * (top - bot)`
        let top: i32 = self.value.into();
        let bot: i32 = dest.value.into();
        let r = bot + scale_i32(alpha.value, top - bot);
        Cu8 { value: r as u8 }
    }
}

/// Scale an i32 value by a u8 (for alpha blending)
#[inline]
fn scale_i32(a: u8, v: i32) -> i32 {
    let c = v * a as i32;
    // cheap alternative to divide by 255
    (((c + 1) + (c >> 8)) >> 8) as i32
}
