# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.5] - 2017-07-16

### Breaking
- placeholder `Pix::Emptya`

### Added
- `Pix::At` is `@`

## [0.0.4] - 2017-07-16

### Added
- no longer require last enum in `Pix1` to be `Empty`

## [0.0.3] - 2017-07-16

### Breaking
- `pix!` macro now takes ints rather than strings for width and height
- `TILESET` is now a `Tileset` struct with `tileset` and `tile_size` attributes
- removed `tile_size` getter from `PixLike`

### Added
- Added `TilesetLike` trait and `impl`ed it for `TILESET`

## [0.0.2] - 2017-07-09

### Breaking
- `pix!` macro takes separate width and height directives (in that order), rather than size
- `PixLike` `tile_size` now returns `(u32, u32)` rather than `i32` to denote width and height

### Added
- support for non-square tilesets

## [0.0.1] - 2017-07-02

### Added
- initial impl
