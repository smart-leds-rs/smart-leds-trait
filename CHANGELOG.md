# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `RGBCCT` type for RGB + warmwhite + coldwhite LEDs such as the WS2805, FW1906 and UCS512H

## [0.3.1] - 2024-01-22
### Added
- Async variant of the trait. This should be seen as an addition to the synchronous implementation

## [0.3.0] - 2024-01-12

### Added
- `serde` feature for enabling serde support for the color types

### Modified
- Moved from `Iterator` to `IntoIterator`, so slices can be used directly

## [0.2.1] - 2020-09-07
### Added
- Some basic documentation
- Add some common impls to `White`, making it easier to use

## [0.2.0] - 2019-05-01
### Added
- Add an `RGBW` alias and a `White` type
### Changed
- Make the `SmartLedsWrite` color parameter generic, to support e.g. RGBW leds

## [0.1.1] - 2019-03-24
### Changed
- Use rgb type from the `rgb` crate

## [0.1.0] - 2019-03-05
### Added
- Initial release


[Unreleased]: https://github.com/smart-leds-rs/smart-leds-trait/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/smart-leds-rs/smart-leds-trait/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/smart-leds-rs/smart-leds-trait/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/smart-leds-rs/smart-leds-trait/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/smart-leds-rs/smart-leds-trait/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/smart-leds-rs/smart-leds-trait/releases/tag/v0.1.0
