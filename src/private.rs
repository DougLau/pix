// private.rs     Private sealed trait
//
// Copyright (c) 2020  Douglas P Lau
//
//! Doc-tests that should fail...
//!
//! ```compile_fail
//! use pix::*;
//! Gray::value(Hsv8::new(0, 128, 255));
//! ```
//! ```compile_fail
//! use pix::*;
//! Hwb::hue(Rgb8::new(255, 255, 255));
//! ```
use crate::ColorModel;
use crate::chan::{
    Alpha, Ch8, Ch16, Ch32, Channel, Gamma, Linear, Premultiplied, Srgb,
    Straight,
};
use crate::el::Pix;
use std::any::Any;

/// Sealed trait to prevent outside crates from implementing traits
pub trait Sealed: Any {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for Straight {}

impl Sealed for Premultiplied {}

impl Sealed for Linear {}

impl Sealed for Srgb {}

impl<const N: usize, C, M, A, G> Sealed for Pix<N, C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
{
}
