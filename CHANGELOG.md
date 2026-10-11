# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Declare the minimum supported Rust version (MSRV) as 1.85 via `rust-version` in `Cargo.toml`; CI now builds and tests on 1.85 (#27).

### Security
- Bump `anstream` 0.6.4 → 0.6.18 in `Cargo.lock` to fix GHSA-2rxc-gjrp-vjhx; affects `cargo install --locked` (#8).

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

[Unreleased]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.2.0...HEAD
[0.2.0]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/sadikkuzu/sadikkuzu.rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/sadikkuzu/sadikkuzu.rs/releases/tag/0.1.0
