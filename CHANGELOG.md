# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- `-h` no longer shows `...` after each flag: flags are plain on/off switches now. Repeating a flag is still accepted (#13).

### Fixed
- Passing flags more than 255 times in total no longer panics in debug builds; in release builds it printed the greeting instead of the links when the total was a multiple of 256 (#13).

## [0.2.1] - 2026-10-11

### Changed
- Update `clap` from 4.4 to 4.6 (#25, #26).
- Raise the minimum supported Rust version (MSRV) to 1.85, now declared via `rust-version` in `Cargo.toml`; CI now builds and tests on 1.85 (#27).

### Security
- Bump `anstream` in `Cargo.lock` from 0.6.4 to 1.0.0 to fix GHSA-2rxc-gjrp-vjhx; affects `cargo install --locked` (#8, #25).

## [0.2.0] - 2023-09-30

### Changed
- Upgrade `clap` from v3 to v4 (#3).

## [0.1.3] - 2023-09-30

### Changed
- Update dependencies (`cargo update`).

## [0.1.2] - 2022-07-09

No user-facing changes.

## [0.1.1] - 2022-07-09

No user-facing changes.

## [0.1.0] - 2022-07-09

### Added
- Initial release: `sadikkuzu` CLI with `--github`, `--linkedin`, `--twitter` and `--all` flags.

[Unreleased]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.2.1...HEAD
[0.2.1]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/sadikkuzu/sadikkuzu.rs/releases/tag/0.1.0
