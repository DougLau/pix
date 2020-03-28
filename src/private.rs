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
use crate::alpha;
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::gamma;
use crate::model::ColorModel;
use crate::el::{Pix1, Pix2, Pix3, Pix4};
use std::any::Any;

/// Sealed trait to prevent outside crates from implementing traits
pub trait Sealed: Any {}

impl Sealed for alpha::Straight {}

impl Sealed for alpha::Premultiplied {}

impl Sealed for gamma::Linear {}

impl Sealed for gamma::Srgb {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for u8 {}

impl Sealed for u16 {}

impl Sealed for f32 {}

impl Sealed for f64 {}

impl Sealed for crate::gray::Gray {}

impl Sealed for crate::hsl::Hsl {}

impl Sealed for crate::hsv::Hsv {}

impl Sealed for crate::hwb::Hwb {}

impl Sealed for crate::mask::Mask {}

impl Sealed for crate::rgb::Rgb {}

impl Sealed for crate::ycc::YCbCr {}

impl<C, M, A, G> Sealed for Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, M, A, G> Sealed for Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, M, A, G> Sealed for Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, M, A, G> Sealed for Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
}
