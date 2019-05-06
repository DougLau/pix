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
    /// Get min of two channel values
    fn min(self, rhs: Self) -> Self;

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
    /// Get min of two channel values
    fn min(self, rhs: Self) -> Self {
        Cu8 { value: self.value.min(rhs.value) }
    }
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

/// Unsigned 16-bit color channel
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cu16 {
    pub value: u16,
}

impl Cu16 {
    pub fn new(value: u16) -> Self {
        Cu16 { value }
    }
}

impl From<u8> for Cu16 {
    fn from(value: u8) -> Self {
        let value = value as u16;
        let value = value << 8 | value;
        Cu16 { value }
    }
}

impl From<f32> for Cu16 {
    fn from(value: f32) -> Self {
        // assert needed here to avoid UB on float-to-int cast
        // once bug #10184 is fixed, this can be removed
        assert!(value >= 0.0 && value <= 1.0);
        let value = (value * 65535.0).round() as u16;
        Cu16 { value }
    }
}

impl From<Cu16> for u8 {
    fn from(c: Cu16) -> u8 {
        (c.value >> 8) as u8
    }
}

impl Mul for Cu16 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let l = self.value as u64;
        let l = (l << 8) | (l >> 8);
        let r = rhs.value as u64;
        let r = (r << 8) | (r >> 8);
        let value = ((l * r) >> 32) as u16;
        Cu16 { value }
    }
}

impl Div for Cu16 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.value > 0 {
            let ss = (self.value as u64) << 16;
            let rr = rhs.value as u64;
            let value = (ss / rr).min(65535) as u16;
            Cu16 { value }
        } else {
            Cu16 { value: 0 }
        }
    }
}

impl Gamma for Cu16 {
    /// Encode a gamma value from linear intensity
    fn encode_gamma(self) -> Self {
        Cu16 { value: self.value.encode_gamma() }
    }
    /// Decode a gamma value into linear intensity
    fn decode_gamma(self) -> Self {
        Cu16 { value: self.value.decode_gamma() }
    }
}

impl Channel for Cu16 {
    /// Get min of two channel values
    fn min(self, rhs: Self) -> Self {
        Cu16 { value: self.value.min(rhs.value) }
    }
    /// Get max of two channel values
    fn max(self, rhs: Self) -> Self {
        Cu16 { value: self.value.max(rhs.value) }
    }
    /// Get channel value with full intensity
    fn full() -> Self {
        Cu16 { value: 0xFFFF }
    }
    /// Linear interpolation
    #[inline]
    fn lerp_alpha(self, dest: Self, alpha: Self) -> Self {
        // NOTE: Alpha blending euqation is: `alpha * top + (1 - alpha) * bot`
        //       This is equivalent to lerp: `bot + alpha * (top - bot)`
        let top: i32 = self.value.into();
        let bot: i32 = dest.value.into();
        let r = bot + scale_i32(alpha.into(), top - bot);
        Cu16 { value: r as u16 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cu8_into() {
        assert_eq!(Cu8::new(255), 1.0.into());
        assert_eq!(Cu8::new(128), 0.5.into());
        assert_eq!(Cu8::new(64), 0.25.into());
        assert_eq!(Cu8::new(32), 0.125.into());
    }
    #[test]
    fn cu8_mul() {
        assert_eq!(Cu8::new(255), Cu8::new(255) * 1.0.into());
        assert_eq!(Cu8::new(128), Cu8::new(255) * 0.5.into());
        assert_eq!(Cu8::new(64), Cu8::new(255) * 0.25.into());
        assert_eq!(Cu8::new(32), Cu8::new(255) * 0.125.into());
        assert_eq!(Cu8::new(16), Cu8::new(255) * 0.0625.into());
        assert_eq!(Cu8::new(64), Cu8::new(128) * 0.5.into());
        assert_eq!(Cu8::new(32), Cu8::new(128) * 0.25.into());
        assert_eq!(Cu8::new(16), Cu8::new(128) * 0.125.into());
        assert_eq!(Cu8::new(8), Cu8::new(128) * 0.0625.into());
    }
    #[test]
    fn cu8_div() {
        assert_eq!(Cu8::new(255), Cu8::new(255) / 1.0.into());
        assert_eq!(Cu8::new(255), Cu8::new(128) / 0.5.into());
        assert_eq!(Cu8::new(255), Cu8::new(64) / 0.25.into());
        assert_eq!(Cu8::new(255), Cu8::new(32) / 0.125.into());
        assert_eq!(Cu8::new(255), Cu8::new(16) / 0.0625.into());
        assert_eq!(Cu8::new(128), Cu8::new(128) / 1.0.into());
        assert_eq!(Cu8::new(128), Cu8::new(64) / 0.5.into());
        assert_eq!(Cu8::new(128), Cu8::new(32) / 0.25.into());
        assert_eq!(Cu8::new(128), Cu8::new(16) / 0.125.into());
        assert_eq!(Cu8::new(64), Cu8::new(64) / 1.0.into());
        assert_eq!(Cu8::new(64), Cu8::new(32) / 0.5.into());
        assert_eq!(Cu8::new(64), Cu8::new(16) / 0.25.into());
    }
    #[test]
    fn cu16_into() {
        assert_eq!(Cu16::new(65535), 1.0.into());
        assert_eq!(Cu16::new(32768), 0.5.into());
        assert_eq!(Cu16::new(16384), 0.25.into());
        assert_eq!(Cu16::new(8192), 0.125.into());
    }
    #[test]
    fn cu16_mul() {
        assert_eq!(Cu16::new(65535), Cu16::new(65535) * 1.0.into());
        assert_eq!(Cu16::new(32768), Cu16::new(65535) * 0.5.into());
        assert_eq!(Cu16::new(16384), Cu16::new(65535) * 0.25.into());
        assert_eq!(Cu16::new(8192), Cu16::new(65535) * 0.125.into());
        assert_eq!(Cu16::new(4096), Cu16::new(65535) * 0.0625.into());
        assert_eq!(Cu16::new(16384), Cu16::new(32768) * 0.5.into());
        assert_eq!(Cu16::new(8192), Cu16::new(32768) * 0.25.into());
        assert_eq!(Cu16::new(4096), Cu16::new(32768) * 0.125.into());
        assert_eq!(Cu16::new(2048), Cu16::new(32768) * 0.0625.into());
    }
    #[test]
    fn cu16_div() {
        assert_eq!(Cu16::new(65535), Cu16::new(65535) / 1.0.into());
        assert_eq!(Cu16::new(65535), Cu16::new(32768) / 0.5.into());
        assert_eq!(Cu16::new(65535), Cu16::new(16384) / 0.25.into());
        assert_eq!(Cu16::new(65535), Cu16::new(8192) / 0.125.into());
        assert_eq!(Cu16::new(65535), Cu16::new(4096) / 0.0625.into());
        assert_eq!(Cu16::new(32768), Cu16::new(32768) / 1.0.into());
        assert_eq!(Cu16::new(32768), Cu16::new(16384) / 0.5.into());
        assert_eq!(Cu16::new(32768), Cu16::new(8192) / 0.25.into());
        assert_eq!(Cu16::new(32768), Cu16::new(4096) / 0.125.into());
        assert_eq!(Cu16::new(16384), Cu16::new(16384) / 1.0.into());
        assert_eq!(Cu16::new(16384), Cu16::new(8192) / 0.5.into());
        assert_eq!(Cu16::new(16384), Cu16::new(4096) / 0.25.into());
    }
}
