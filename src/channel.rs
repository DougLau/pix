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

impl Cu8 {
    pub fn new(value: u8) -> Self {
        Cu8 { value }
    }
}

impl From<u8> for Cu8 {
    fn from(value: u8) -> Self {
        Cu8 { value }
    }
}

impl From<f32> for Cu8 {
    fn from(value: f32) -> Self {
        // assert needed here to avoid UB on float-to-int cast
        // once bug #10184 is fixed, this can be removed
        assert!(value >= 0.0 && value <= 1.0);
        let value = (value * 255.0).round() as u8;
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
        let l = self.value as u32;
        let l = (l << 4) | (l >> 4);
        let r = rhs.value as u32;
        let r = (r << 4) | (r >> 4);
        let value = ((l * r) >> 16) as u8;
        Cu8 { value }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cu8_into() {
        assert_eq!(Cu8::new(255), Into::<Cu8>::into(1.0));
        assert_eq!(Cu8::new(128), Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(64), Into::<Cu8>::into(0.25));
        assert_eq!(Cu8::new(32), Into::<Cu8>::into(0.125));
    }
    #[test]
    fn cu8_mul() {
        assert_eq!(Cu8::new(255), Cu8::new(255) * Into::<Cu8>::into(1.0));
        assert_eq!(Cu8::new(128), Cu8::new(255) * Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(64), Cu8::new(255) * Into::<Cu8>::into(0.25));
        assert_eq!(Cu8::new(32), Cu8::new(255) * Into::<Cu8>::into(0.125));
        assert_eq!(Cu8::new(16), Cu8::new(255) * Into::<Cu8>::into(0.0625));
        assert_eq!(Cu8::new(64), Cu8::new(128) * Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(32), Cu8::new(128) * Into::<Cu8>::into(0.25));
        assert_eq!(Cu8::new(16), Cu8::new(128) * Into::<Cu8>::into(0.125));
        assert_eq!(Cu8::new(8), Cu8::new(128) * Into::<Cu8>::into(0.0625));
    }
    #[test]
    fn cu8_div() {
        assert_eq!(Cu8::new(255), Cu8::new(255) / Into::<Cu8>::into(1.0));
        assert_eq!(Cu8::new(255), Cu8::new(128) / Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(255), Cu8::new(64) / Into::<Cu8>::into(0.25));
        assert_eq!(Cu8::new(255), Cu8::new(32) / Into::<Cu8>::into(0.125));
        assert_eq!(Cu8::new(255), Cu8::new(16) / Into::<Cu8>::into(0.0625));
        assert_eq!(Cu8::new(128), Cu8::new(128) / Into::<Cu8>::into(1.0));
        assert_eq!(Cu8::new(128), Cu8::new(64) / Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(128), Cu8::new(32) / Into::<Cu8>::into(0.25));
        assert_eq!(Cu8::new(128), Cu8::new(16) / Into::<Cu8>::into(0.125));
        assert_eq!(Cu8::new(64), Cu8::new(64) / Into::<Cu8>::into(1.0));
        assert_eq!(Cu8::new(64), Cu8::new(32) / Into::<Cu8>::into(0.5));
        assert_eq!(Cu8::new(64), Cu8::new(16) / Into::<Cu8>::into(0.25));
    }
}
