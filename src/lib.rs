// lib.rs      Pix crate.
//
// Copyright (c) 2019  Douglas P Lau
//
//! Pix is a library for 2D image manipulation.
//!
#[macro_use]
extern crate log;

mod alpha;
mod channel;
mod gamma;
mod gray;
mod pixel;
mod raster;
mod rgb;
mod rgba;
mod srgb;

pub use crate::alpha::Alpha;
pub use crate::channel::{Channel, Ch8, Ch16};
pub use crate::gray::Gray;
pub use crate::pixel::PixFmt;
pub use crate::raster::Raster;
pub use crate::rgb::Rgb;
pub use crate::rgba::Rgba;
pub use crate::srgb::Srgb;
