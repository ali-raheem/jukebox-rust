# Changelog

## [0.2.0] - 2025-01-07

### Changed
- Updated to Rust 2021 edition
- Replaced deprecated `serial` crate with `serialport`
- Updated `rusqlite` to 0.31
- Improved add mode workflow: now reads card first, then prompts for command
- Increased serial timeout from 100ms to 1000ms for reliability

### Fixed
- Panic when card input shorter than expected (now validates length)
- Unnecessary string allocations (using `drain()` instead of `to_owned()`)

## [0.1.6] - 2017-11-25

- Built with rustc nightly (e97ba8328 2017-11-25)

## [0.1.5] - 2017-06-14

- Gracefully handle unknown arguments

## [0.1.4] - 2017-06-13

- Use Connection not SqliteConnection for modern rusqlite
- Builds with rustc 1.19.0-nightly (cfb5debbc 2017-06-12)
