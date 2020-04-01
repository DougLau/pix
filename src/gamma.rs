// gamma.rs     Gamma encoding/decoding
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Module for gamma encoding items
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::private::Sealed;
use std::fmt::Debug;

// Include functions to convert gamma between linear and sRGB
include!("srgb_gamma.rs");

// Include build-time sRGB gamma look-up tables
include!(concat!(env!("OUT_DIR"), "/gamma_lut.rs"));

/// *Gamma* conversion mode.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait Mode: Copy + Clone + Debug + Default + PartialEq + Sealed {
    /// Convert a `Channel` value to linear.
    fn to_linear<C: Channel>(c: C) -> C;
    /// Convert a `Channel` value from linear.
    fn from_linear<C: Channel>(c: C) -> C;
}

/// Linear gamma (no gamma correction)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Linear;

/// Gamma correction using the sRGB formula
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Srgb;

// TODO: add PowerLawGamma when const generics feature is stable

/// Trait to encode/decode sRGB values.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait SrgbValue: Sealed {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self;
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self;
}

impl SrgbValue for Ch8 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        let s = ENCODE_SRGB_U8[usize::from(u8::from(self))];
        Self::new(s)
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        let s = DECODE_SRGB_U8[usize::from(u8::from(self))];
        Self::new(s)
    }
}

impl SrgbValue for Ch16 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        let s = f32::from(u16::from(self)) / 65535.0;
        let s = (srgb_gamma_encode(s) * 65535.0).round() as u16;
        Self::new(s)
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        let s = f32::from(u16::from(self)) / 65535.0;
        let s = (srgb_gamma_decode(s) * 65535.0).round() as u16;
        Self::new(s)
    }
}

impl SrgbValue for Ch32 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        let s = srgb_gamma_encode(f32::from(self));
        Self::new(s)
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        let s = srgb_gamma_decode(f32::from(self));
        Self::new(s)
    }
}

impl Mode for Linear {
    /// Convert a `Channel` value to linear.
    fn to_linear<C: Channel>(c: C) -> C {
        c
    }
    /// Convert a `Channel` value from linear.
    fn from_linear<C: Channel>(c: C) -> C {
        c
    }
}

impl Mode for Srgb {
    /// Convert a `Channel` value to linear.
    fn to_linear<C: Channel>(c: C) -> C {
        c.decode_srgb()
    }
    /// Convert a `Channel` value from linear.
    fn from_linear<C: Channel>(c: C) -> C {
        c.encode_srgb()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lut_encode_u8() {
        for i in 0..=255 {
            let s = i as f32 / 255.0;
            let v = (srgb_gamma_encode(s) * 255.0).round() as u8;
            assert_eq!(v, ENCODE_SRGB_U8[i]);
        }
    }
    #[test]
    fn lut_decode_u8() {
        for i in 0..=255 {
            let s = i as f32 / 255.0;
            let v = (srgb_gamma_decode(s) * 255.0).round() as u8;
            assert_eq!(v, DECODE_SRGB_U8[i]);
        }
    }
}
