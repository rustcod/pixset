# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking
- `pix!` macro takes separate width and height directives (in that order), rather than size
- `PixLike` `tile_size` now returns `(u32, u32)` rather than `i32` to denote width and height

### Added
- support for non-square tilesets
