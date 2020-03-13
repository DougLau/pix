// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Module for color model items
use crate::private::Sealed;

/// Model for describing colors.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait ColorModel: Sealed {

    /// Number of [channel](trait.Channel.html)s in the model
    const NUM_CHANNELS: usize;
}
