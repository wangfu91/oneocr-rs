# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.3.2] - 2025-09-05

### Changed
- Updated dependencies in `Cargo.toml`.
- Updated `README.md` with documentation improvements.

## [v0.3.1] - 2025-08-11

### Added
- Implemented `PartialEq` trait for `OcrLine` struct
- Implemented `Default` trait for `Resolution` struct
- Added `PartialEq` and related traits for core structs to improve usability

### Changed
- Updated Cargo.toml with additional metadata
- Updated README.md with latest information

### Internal
- Updated publish workflow configuration

## [v0.3.0] - 2025-07-22

### Added
- **New `ImageInput` enum** - Support for multiple image sources including file paths, buffers, and dynamic images
- **New `OcrOptions` struct** - Configurable OCR behavior with word-level details and resize resolution options
- Enhanced FFI bindings with model delay load improvements

### Changed
- **Breaking**: Refactored `OcrEngine` API to use new `ImageInput` and `OcrOptions` abstractions
- Updated all examples to use the new API structure
- Updated benchmarks to reflect new API changes
- Improved bounding box handling

### Fixed
- Fixed clippy warnings throughout the codebase
- Addressed PR feedback and code quality improvements

### Internal
- Restricted publish workflow to master branch only
- Updated README.md documentation
- Code cleanup and comment updates

## [v0.2.0] - 2025-06-23

### Changed
- Simplified library API calls using the windows-link macro
- Refined error messages for better user experience
- Updated OCR result handling implementation

### Internal
- Updated benchmark implementation
- Deleted outdated comments and code cleanup

## [v0.1.1] - 2025-05-23

### Added
- Re-exported `bounding_box::Point` struct to public API for external use

### Changed
- Updated README.md with improved documentation

## [v0.1.0] - 2025-05-23

### Added
- Initial release of oneocr-rs
- Core OCR functionality with Rust bindings
- Basic bounding box support
- FFI interface for OCR operations
- Support for image processing and text recognition

---

[v0.3.2]: https://github.com/wangfu91/oneocr-rs/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/wangfu91/oneocr-rs/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/wangfu91/oneocr-rs/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/wangfu91/oneocr-rs/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/wangfu91/oneocr-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/wangfu91/oneocr-rs/releases/tag/v0.1.0
