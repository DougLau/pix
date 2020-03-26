// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Module for alpha channel items
use crate::channel::Channel;
use crate::private::Sealed;
use std::any::Any;
use std::fmt::Debug;

/// Trait for handling straight versus premultiplied alpha.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait Mode:
    Any + Copy + Clone + Debug + Default + PartialEq + Sealed
{
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, a: C) -> C;
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, a: C) -> C;
}

/// Each `Channel` is "straight" (not premultiplied with alpha)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Straight;

/// Each `Channel` is premultiplied, or associated, with alpha
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Premultiplied;

impl Mode for Straight {
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, _a: C) -> C {
        c
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, _a: C) -> C {
        c
    }
}

impl Mode for Premultiplied {
    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel>(c: C, a: C) -> C {
        c * a
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel>(c: C, a: C) -> C {
        c / a
    }
}
