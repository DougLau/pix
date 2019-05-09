// channel.rs       Color channels
//
// Copyright (c) 2019  Douglas P Lau
//
use std::cmp::Ordering;
use std::ops::{Div, Mul};
use crate::gamma::Gamma;

/// One *component* of a pixel [Format](trait.Format.html).
///
/// For example, in [Rgb](struct.Rgb.html) there are *red*, *green* and *blue*
/// channels.
///
/// Defined channels are [Ch8](struct.Ch8.html), [Ch16](struct.Ch16.html)
/// and [Ch32](struct.Ch32.html).
pub trait Channel: Copy + Default + Ord + Mul<Output=Self> + Div<Output=Self> +
    Gamma
{
    /// Minimum intensity (*zero*)
    const MIN: Self;

    /// Maximum intensity (*one*)
    const MAX: Self;

    /// Linear interpolation
    fn lerp(self, rhs: Self, t: Self) -> Self;
}

/// 8-bit color [Channel](trait.Channel.html).
///
/// The channel is represented by a u8, but multiplication and division treat
/// the values as though they range between 0 and 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ch8(u8);

/// 16-bit color [Channel](trait.Channel.html)
///
/// The channel is represented by a u16, but multiplication and division treat
/// the values as though they range between 0 and 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ch16(u16);

/// 32-bit color [Channel](trait.Channel.html)
///
/// The channel is represented by an f32, but the value is guaranteed to be
/// between 0 and 1, inclusive.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Ch32(f32);

impl Ch8 {
    /// Create a new 8-bit channel value.
    pub fn new(value: u8) -> Self {
        Ch8 { 0: value }
    }
}

impl From<u8> for Ch8 {
    fn from(value: u8) -> Self {
        Ch8 { 0: value }
    }
}

impl From<Ch8> for u8 {
    fn from(c: Ch8) -> u8 {
        c.0
    }
}

impl<R> Mul<R> for Ch8 where Self: From<R> {
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        let rhs: Self = rhs.into();
        let l = self.0 as u32;
        let l = (l << 4) | (l >> 4);
        let r = rhs.0 as u32;
        let r = (r << 4) | (r >> 4);
        let value = ((l * r) >> 16) as u8;
        Ch8 { 0: value }
    }
}

impl Mul<f32> for Ch8 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let rhs: Self = Ch32::new(rhs).into();
        self * rhs
    }
}

impl<R> Div<R> for Ch8 where Self: From<R> {
    type Output = Self;
    fn div(self, rhs: R) -> Self {
        let rhs: Self = rhs.into();
        if rhs.0 > 0 {
            let ss = (self.0 as u32) << 8;
            let rr = rhs.0 as u32;
            let value = (ss / rr).min(255) as u8;
            Ch8 { 0: value }
        } else {
            Ch8 { 0: 0 }
        }
    }
}

impl Div<f32> for Ch8 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let rhs: Self = Ch32::new(rhs).into();
        self / rhs
    }
}

impl Gamma for Ch8 {
    /// Encode a gamma value from linear intensity
    fn encode_gamma(self) -> Self {
        Ch8 { 0: self.0.encode_gamma() }
    }
    /// Decode a gamma value into linear intensity
    fn decode_gamma(self) -> Self {
        Ch8 { 0: self.0.decode_gamma() }
    }
}

impl Channel for Ch8 {

    /// Minimum intensity (*zero*)
    const MIN: Ch8 = Ch8 { 0: 0 };

    /// Maximum intensity (*one*)
    const MAX: Ch8 = Ch8 { 0: 0xFF };

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        // NOTE: Alpha blending euqation is: `(1 - t) * v0 + t * v1`
        //       This is equivalent to lerp: `v0 + t * (v1 - v0)`
        let v0: i32 = self.0.into();
        let v1: i32 = rhs.0.into();
        let r = v0 + scale_i32(t.0, v1 - v0);
        Ch8 { 0: r as u8 }
    }
}

/// Scale an i32 value by a u8 (for lerp)
#[inline]
fn scale_i32(t: u8, v: i32) -> i32 {
    let c = v * t as i32;
    // cheap alternative to divide by 255
    (((c + 1) + (c >> 8)) >> 8) as i32
}

impl Ch16 {
    /// Create a new 16-bit channel value.
    pub fn new(value: u16) -> Self {
        Ch16 { 0: value }
    }
}

impl From<Ch8> for Ch16 {
    fn from(c: Ch8) -> Self {
        let value = c.0 as u16;
        let value = value << 8 | value;
        Ch16 { 0: value }
    }
}

impl From<u16> for Ch16 {
    fn from(value: u16) -> Self {
        Ch16 { 0: value }
    }
}

impl From<Ch16> for Ch8 {
    fn from(c: Ch16) -> Self {
        Ch8::new((c.0 >> 8) as u8)
    }
}

impl<R> Mul<R> for Ch16 where Self: From<R> {
    type Output = Self;
    fn mul(self, rhs: R) -> Self {
        let rhs: Self = rhs.into();
        let l = self.0 as u64;
        let l = (l << 8) | (l >> 8);
        let r = rhs.0 as u64;
        let r = (r << 8) | (r >> 8);
        let value = ((l * r) >> 32) as u16;
        Ch16 { 0: value }
    }
}

impl Mul<f32> for Ch16 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let rhs: Self = Ch32::new(rhs).into();
        self * rhs
    }
}

impl<R> Div<R> for Ch16 where Self: From<R> {
    type Output = Self;
    fn div(self, rhs: R) -> Self {
        let rhs: Self = rhs.into();
        if rhs.0 > 0 {
            let ss = (self.0 as u64) << 16;
            let rr = rhs.0 as u64;
            let value = (ss / rr).min(65535) as u16;
            Ch16 { 0: value }
        } else {
            Ch16 { 0: 0 }
        }
    }
}

impl Div<f32> for Ch16 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let rhs: Self = Ch32::new(rhs).into();
        self / rhs
    }
}

impl Gamma for Ch16 {
    /// Encode a gamma value from linear intensity
    fn encode_gamma(self) -> Self {
        Ch16 { 0: self.0.encode_gamma() }
    }
    /// Decode a gamma value into linear intensity
    fn decode_gamma(self) -> Self {
        Ch16 { 0: self.0.decode_gamma() }
    }
}

impl Channel for Ch16 {

    /// Minimum intensity (*zero*)
    const MIN: Ch16 = Ch16 { 0: 0 };

    /// Maximum intensity (*one*)
    const MAX: Ch16 = Ch16 { 0: 0xFFFF };

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        // NOTE: Alpha blending euqation is: `(1 - t) * v0 + t * v1`
        //       This is equivalent to lerp: `v0 + t * (v1 - v0)`
        let v0: i64 = self.0.into();
        let v1: i64 = rhs.0.into();
        let r = v0 + scale_i64(t.0, v1 - v0);
        Ch16 { 0: r as u16 }
    }
}

/// Scale an i64 value by a u16 (for lerp)
#[inline]
fn scale_i64(t: u16, v: i64) -> i64 {
    let c = v * t as i64;
    // cheap alternative to divide by 65535
    (((c + 1) + (c >> 16)) >> 16) as i64
}

impl Ch32 {
    /// Create a new 32-bit channel value.
    ///
    /// Returns [MIN](trait.Channel.html#associatedconstant.MIN) if value is
    ///         less than 0.0, or NaN.
    /// Returns [MAX](trait.Channel.html#associatedconstant.MAX) if value is
    ///         greater than 1.0.
    pub fn new(value: f32) -> Self {
        let v = if value.is_nan() || value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        };
        Ch32 { 0: v }
    }
}

impl From<Ch8> for Ch32 {
    fn from(c: Ch8) -> Self {
        let value = c.0 as f32 * 255.0;
        Ch32 { 0: value }
    }
}

impl From<f32> for Ch32 {
    fn from(value: f32) -> Self {
        Ch32::new(value)
    }
}

impl From<Ch32> for Ch8 {
    fn from(c: Ch32) -> Self {
        let value = c.0;
        debug_assert!(value >= 0.0 && value <= 1.0);
        // cast is not UB since the value is guaranteed to
        // be between 0.0 and 1.0 (see bug #10184)
        Ch8::new((value * 255.0).round() as u8)
    }
}

impl From<Ch32> for Ch16 {
    fn from(c: Ch32) -> Self {
        let value = c.0;
        debug_assert!(value >= 0.0 && value <= 1.0);
        // cast is not UB since the value is guaranteed to
        // be between 0.0 and 1.0 (see bug #10184)
        Ch16::new((value * 65535.0).round() as u16)
    }
}

impl Eq for Ch32 { }

impl Ord for Ch32 {
    fn cmp(&self, other: &Ch32) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Mul for Ch32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Ch32 { 0: self.0 * rhs.0 }
    }
}

impl Div for Ch32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let v = rhs.0;
        if v > 0.0 {
            let value = (self.0 / v).min(1.0);
            Ch32 { 0: value }
        } else {
            Ch32 { 0: 0.0 }
        }
    }
}

impl Gamma for Ch32 {
    /// Encode a gamma value from linear intensity
    fn encode_gamma(self) -> Self {
        Ch32 { 0: self.0.encode_gamma() }
    }
    /// Decode a gamma value into linear intensity
    fn decode_gamma(self) -> Self {
        Ch32 { 0: self.0.decode_gamma() }
    }
}

impl Channel for Ch32 {

    /// Minimum intensity (*zero*)
    const MIN: Ch32 = Ch32 { 0: 0.0 };

    /// Maximum intensity (*one*)
    const MAX: Ch32 = Ch32 { 0: 1.0 };

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        // NOTE: Alpha blending euqation is: `(1 - t) * v0 + t * v1`
        //       This is equivalent to lerp: `v0 + t * (v1 - v0)`
        let v0 = self.0;
        let v1 = rhs.0;
        let r = v0 + t.0 * (v1 - v0);
        Ch32 { 0: r }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ch8_into() {
        assert_eq!(Ch16::new(255), 255.into());
        assert_eq!(Ch16::new(128), 128.into());
        assert_eq!(Ch16::new(64), 64.into());
        assert_eq!(Ch16::new(32), 32.into());
    }
    #[test]
    fn ch16_into() {
        assert_eq!(Ch16::new(65535), 65535.into());
        assert_eq!(Ch16::new(32768), 32768.into());
        assert_eq!(Ch16::new(16384), 16384.into());
        assert_eq!(Ch16::new(8192), 8192.into());
    }
    #[test]
    fn ch32_into() {
        assert_eq!(Ch32::new(1.0), 1.0.into());
        assert_eq!(Ch32::new(0.5), 0.5.into());
        assert_eq!(Ch32::new(0.25), 0.25.into());
        assert_eq!(Ch32::new(0.125), 0.125.into());
    }
    #[test]
    fn ch8_mul() {
        assert_eq!(Ch8::new(255), Ch8::new(255) * 1.0);
        assert_eq!(Ch8::new(128), Ch8::new(255) * 0.5);
        assert_eq!(Ch8::new(64), Ch8::new(255) * 0.25);
        assert_eq!(Ch8::new(32), Ch8::new(255) * 0.125);
        assert_eq!(Ch8::new(16), Ch8::new(255) * 0.0625);
        assert_eq!(Ch8::new(64), Ch8::new(128) * 0.5);
        assert_eq!(Ch8::new(32), Ch8::new(128) * 0.25);
        assert_eq!(Ch8::new(16), Ch8::new(128) * 0.125);
        assert_eq!(Ch8::new(8), Ch8::new(128) * 0.0625);
    }
    #[test]
    fn ch8_div() {
        assert_eq!(Ch8::new(255), Ch8::new(255) / 1.0);
        assert_eq!(Ch8::new(255), Ch8::new(128) / 0.5);
        assert_eq!(Ch8::new(255), Ch8::new(64) / 0.25);
        assert_eq!(Ch8::new(255), Ch8::new(32) / 0.125);
        assert_eq!(Ch8::new(255), Ch8::new(16) / 0.0625);
        assert_eq!(Ch8::new(128), Ch8::new(128) / 1.0);
        assert_eq!(Ch8::new(128), Ch8::new(64) / 0.5);
        assert_eq!(Ch8::new(128), Ch8::new(32) / 0.25);
        assert_eq!(Ch8::new(128), Ch8::new(16) / 0.125);
        assert_eq!(Ch8::new(64), Ch8::new(64) / 1.0);
        assert_eq!(Ch8::new(64), Ch8::new(32) / 0.5);
        assert_eq!(Ch8::new(64), Ch8::new(16) / 0.25);
    }
    #[test]
    fn ch16_mul() {
        assert_eq!(Ch16::new(65535), Ch16::new(65535) * 1.0);
        assert_eq!(Ch16::new(32768), Ch16::new(65535) * 0.5);
        assert_eq!(Ch16::new(16384), Ch16::new(65535) * 0.25);
        assert_eq!(Ch16::new(8192), Ch16::new(65535) * 0.125);
        assert_eq!(Ch16::new(4096), Ch16::new(65535) * 0.0625);
        assert_eq!(Ch16::new(16384), Ch16::new(32768) * 0.5);
        assert_eq!(Ch16::new(8192), Ch16::new(32768) * 0.25);
        assert_eq!(Ch16::new(4096), Ch16::new(32768) * 0.125);
        assert_eq!(Ch16::new(2048), Ch16::new(32768) * 0.0625);
    }
    #[test]
    fn ch16_div() {
        assert_eq!(Ch16::new(65535), Ch16::new(65535) / 1.0);
        assert_eq!(Ch16::new(65535), Ch16::new(32768) / 0.5);
        assert_eq!(Ch16::new(65535), Ch16::new(16384) / 0.25);
        assert_eq!(Ch16::new(65535), Ch16::new(8192) / 0.125);
        assert_eq!(Ch16::new(65535), Ch16::new(4096) / 0.0625);
        assert_eq!(Ch16::new(32768), Ch16::new(32768) / 1.0);
        assert_eq!(Ch16::new(32768), Ch16::new(16384) / 0.5);
        assert_eq!(Ch16::new(32768), Ch16::new(8192) / 0.25);
        assert_eq!(Ch16::new(32768), Ch16::new(4096) / 0.125);
        assert_eq!(Ch16::new(16384), Ch16::new(16384) / 1.0);
        assert_eq!(Ch16::new(16384), Ch16::new(8192) / 0.5);
        assert_eq!(Ch16::new(16384), Ch16::new(4096) / 0.25);
    }
}
