## [Unreleased]
### Added
* AlphaMode trait (and implementors: AssociatedAlpha, SeparatedAlpha)
* AlphaModeID can now be UnknownAlpha (for masks)
* GammaMode trait (and implementors: LinearGamma, SrgbGamma)
* GammaModeID can now be UnknownGamma (for masks)
* Implemented Add for Ch8, Ch16, and Ch32
* Type aliases for new generics
### Changed
* Gamma and Alpha are now Generics on Gray, Rgb instead of attributes on Raster
* Rename AlphaMode to AlphaModeID
* Format now depends on traits AlphaMode + GammaMode
* Rename GammaMode to GammaModeID

## [0.7.0] - 2020-01-01
### Added
* Implemented From u8, u16, f32 for Translucent
* Implemented From for Translucent <=> Opaque
### Changed
* Alpha, Channel now depend on traits Debug + Mul

## [0.6.1] - 2019-07-28
### Added
* Allow converting from Mask to Rgb

## [0.6.0] - 2019-05-24
### Added
* Implemented From<i32> for Rgb
* RasterBuilder::with_color
* Palette struct
* Format::difference / within_threshold methods
### Changed
* RasterBuilder methods reworked to be more consistent
### Removed
* Raster::to_raster (use RasterBuilder.with_raster instead)

## [0.5.0] - 2019-05-18
### Added
* RasterBuilder
* Raster::region, region_iter, set_region, to_raster methods
* RasterIter struct
* Gray, Mask, Rgb now implement Iterator (for use with set_region)
* AlphaMode, GammaMode, PixModes
### Changed
* Fixed bugs in channel conversions
* Region::intersection, right, bottom reworked
### Removed
* Raster::set_rect (use set_region instead)
* Srgb struct and types (use GammaMode instead)

## [0.4.0] - 2019-05-11
### Added
* Ch32 channel
### Changed
* Reworked alpha channel handling
* Cu8 / Cu16 renamed to Ch8 / Ch16
* Renamed PixFmt to Format
### Removed
* Blending/lerp moved to pixops crate

## [0.3.0] - 2019-05-05
### Added
* Channel trait with Cu8 and Cu16 implementations
* Gamma encoding/decoding module
* Alpha, Gray, Rgb, Rgba pixel formats
* New Srgb pixel format
* Raster::with_pixels and Raster::into to transfer pixel ownership
### Removed
* Alpha8, Gray8, Rgb8, Rgba8 pixel formats
* RasterB -- use Raster::with_pixels instead

## [0.2.0] - 2019-05-01
### Added
* Raster implements Clone and Debug
* Raster::set_rect
* Implemented PartialEq for alpha8, gray8, rgb8 and rgba8
### Changed
* Updated to rust 2018 edition

## [0.1.0] - 2019-04-07
### Added
* Initial version (from footile)
