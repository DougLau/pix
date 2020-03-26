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

impl SrgbValue for u8 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        ENCODE_SRGB_U8[usize::from(self)]
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        DECODE_SRGB_U8[usize::from(self)]
    }
}

impl SrgbValue for Ch8 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(u8::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(u8::from(self).decode_srgb())
    }
}

impl SrgbValue for u16 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        let s = f32::from(self) / 65535.0;
        (s.encode_srgb() * 65535.0).round() as u16
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        let s = f32::from(self) / 65535.0;
        (s.decode_srgb() * 65535.0).round() as u16
    }
}

impl SrgbValue for Ch16 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(u16::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(u16::from(self).decode_srgb())
    }
}

impl SrgbValue for f32 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        srgb_gamma_encode(self)
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        srgb_gamma_decode(self)
    }
}

impl SrgbValue for Ch32 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(f32::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(f32::from(self).decode_srgb())
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
            let v = (s.encode_srgb() * 255.0).round() as u8;
            assert_eq!(v, ENCODE_SRGB_U8[i]);
        }
    }
    #[test]
    fn lut_decode_u8() {
        for i in 0..=255 {
            let s = i as f32 / 255.0;
            let v = (s.decode_srgb() * 255.0).round() as u8;
            assert_eq!(v, DECODE_SRGB_U8[i]);
        }
    }
}
