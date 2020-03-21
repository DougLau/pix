// srgb_gamma.rs    sRGB gamma functions
//
// Copyright (c) 2019-2020  Douglas P Lau
//
// Functions for encoding and decoding gamma between linear and sRGB.
//
// This is a separate file so that it can be includeed by build.rs

/// Encode an sRGB gamma value from linear intensity
fn srgb_gamma_encode(v: f32) -> f32 {
    if v <= 0.0 {
        0.0
    } else if v < 0.003_130_8 {
        v * 12.92
    } else if v < 1.0 {
        v.powf(1.0 / 2.4) * 1.055 - 0.055
    } else {
        1.0
    }
}

/// Decode an sRGB gamma value into linear intensity
fn srgb_gamma_decode(v: f32) -> f32 {
    if v <= 0.0 {
        0.0
    } else if v < 0.04045 {
        v / 12.92
    } else if v < 1.0 {
        ((v + 0.055) / 1.055).powf(2.4)
    } else {
        1.0
    }
}
