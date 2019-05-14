## [Unreleased]

### Added
* RasterIter struct
* Raster::region, region_iter, set_region, to_raster methods
* Gray, Mask, Rgb now implement Iterator (for use with set_region)
### Changed
* Fixed bugs in channel conversions
* Region::intersection, right, bottom reworked
### Removed
* Raster::set_rect (use set_region instead)

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
