// clr/mod.rs   Color module.
//
// Copyright (c) 2020  Douglas P Lau
//
//! Color models
pub(crate) mod bgr;
pub(crate) mod gray;
pub(crate) mod hsl;
pub(crate) mod hsv;
mod hue;
pub(crate) mod hwb;
pub(crate) mod matte;
mod model;
pub(crate) mod rgb;
pub(crate) mod ycc;

pub use bgr::Bgr;
pub use gray::Gray;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use hwb::Hwb;
pub use matte::Matte;
pub use model::ColorModel;
pub use rgb::Rgb;
pub use ycc::YCbCr;
