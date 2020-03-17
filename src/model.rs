// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Module for color model items
use crate::private::Sealed;
use crate::Channel;

/// Model for pixel colors.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait ColorModel: Sealed {

    /// Component `Channel` type
    type Chan: Channel;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan];

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan;

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4];

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self;

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self;

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool;
}
