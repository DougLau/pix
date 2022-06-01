## [Unreleased]

## [0.13.2] - 2022-06-01
### Added
* impl `From<(i32, i32)>` for `Region`

## [0.13.1] - 2020-06-04
### Added
* `Region::left()` and `::top()`
### Changed
* Made `Region::width()`, `::height()`, `::right()` and `::bottom()` pub

## [0.13.0] - 2020-05-05
### Changed
* `Raster::rows()` / `rows_mut()` sliced to span of region
* Renamed `Pixel::composite_channels_matte` to `composite_channels_alpha`
* Matte type aliases premultiplied instead of straight

## [0.12.0] - 2020-04-23
### Added
* Raster::copy_color and copy_raster
* CMY color model

### Changed
* Moved color models to their own sub-modules
* composite methods only available for premultiplied Rasters

## [0.11.0] - 2020-04-11
### Added
* Raster::pixel_mut(x, y); replaces set_pixel
* Raster::rows and ::rows_mut
* Raster::with_ constructors
* ops module with Porter-Duff compositing
* Raster::composite_color
* Raster::composite_raster

### Changed
* Renamed channel module to chan
* Moved alpha/gamma stuff to chan module
* Renamed model module to clr
* Moved color model impls to clr module
* GrayModel to Gray
* HslModel to Hsl
* HsvModel to Hsv
* HwbModel to Hwb
* MaskModel to Matte
* RgbModel to Rgb
* YCbCrModel to YCbCr
* Moved `alpha` associated functions out of ColorModel trait

### Removed
* Raster::set_pixel(x, y, p)
* `Pix1::From<u8>` and `Pix1::From<i32>`
* Raster::set_region
* RasterBuilder (use Raster::with_ methods instead)

## [0.10.0] - 2020-03-26
### Added
* ColorModel trait
* HsvModel, HslModel, HwbModel and YCbCrModel
* Pix1, Pix2, Pix3, Pix4 structs

### Changed
* Use Any/TypeId instead of GammaModeID
* Renamed GammaMode to gamma::Mode + sealed trait
* Renamed LinearGamma to gamma::Linear
* Renamed SrgbGamma to gamma::Srgb
* Use Any/TypeId instead of AlphaModeID
* Renamed AlphaMode to alpha::Mode + sealed trait
* Renamed StraightAlpha to alpha::Straight
* Renamed PremultipliedAlpha to alpha::Premultiplied
* Renamed Format trait to Pixel
* Renamed all GrayAlpha types to Graya
* Types with alpha can just use `new` to construct
* Gray conversion now uses perceptual luminance

### Removed
* alpha::Alpha and Translucent/Opaque
* Gray/Rgb with_alpha constructors (use `new` instead)

## [0.9.0] - 2020-03-08
### Changed
* Simplified type aliases to shorter names
* AssociatedAlpha renamed to PremultipliedAlpha
* SeparatedAlpha renamed to StraightAlpha
* Alpha items no longer re-exported in crate root
* Gamma items no longer re-exported in crate root

## [0.8.0] - 2020-02-20
### Added
* AlphaMode trait (and implementors: AssociatedAlpha, SeparatedAlpha)
* AlphaModeID can now be UnknownAlpha (for masks)
* GammaMode trait (and implementors: LinearGamma, SrgbGamma)
* GammaModeID can now be UnknownGamma (for masks)
* Implemented Add for Ch8, Ch16, and Ch32
* Type aliases for new generics
* Format::convert method
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
