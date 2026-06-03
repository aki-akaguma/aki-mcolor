# Changelog: aki-mcolor
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Document `docs/reviews/2026-06-01_code_review.3.md`.

### Changed

- Reorganize existing code review files into `docs/reviews/` directory and adopt a new `YYYY-MM-DD_code_review.N.md` naming convention.
- Remove stale, commented-out test code in `src/util/opt_uc_x_param.rs`.

## [0.2.1] - 2026-05-18

### Changed

- Refactor core processing logic to reuse memory buffers for line-by-line processing, reducing heap allocations and improving performance.
- Document capture group coloring feature, where parentheses can be used to color only a specific part of a match.
- Update crates: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2).
- Update crates: `runnel` (0.4.2), `regex` (1.12).
- Update MSRV to 1.68.0 (2c8cc3432 2023-03-06).

### Fixed

- Correct misspelling of "Magenta" throughout the codebase (CLI options, environment variables, enums, etc.).
- Resolve Clippy warnings: `uninlined-format-args`, `needless_borrow`.

### Removed

- `memx-cdy` dependency.

## [0.2.0] - 2025-09-15

### Added

- `specs` directory.
- Additional tests.
- `execute_with_env()` function.

### Changed

- Implement `IntoIterator` compatibility for arguments in `execute()`.
- Update crates: `runnel` (0.4.0), `rust-version-info-file` (0.2), `regex` (1.11).
- Refactor `run.rs` and `lib.rs`.

### Fixed

- Update minimum supported version in documentation.

### Removed

- `execute_env()`.
- `base_dir=` from `-X` options.

## [0.1.32] - 2024-06-19

### Added

- GitHub Actions workflows for Ubuntu, macOS, and Windows.
- Test status badges to `README.tpl`.
- Miri support for tests.
- Tarpaulin support to `Makefile`.

### Changed

- Rename `config` to `config.toml`.
- Remove `cfg(has_not_matches)`.
- Refactor `Makefile`.
- Update dependencies: `flood-tide` (0.2.11), `flood-tide-gen` (0.1.22), `memx-cdy` (0.1.13), `runnel` (0.3.19), `exec-target` (0.2.8), `indoc` (2.0.0), `rust-version-info-file` (0.1.10).

### Fixed

- Update `LICENSE-APACHE` and `LICENSE-MIT` files.
- Resolve Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`, `derivable_impls`.
- Update MSRV from 1.56.0 to 1.65.0.

### Removed

- `COPYING` file.

## [0.1.31] - 2023-01-11

### Added

- Badges to `README.tpl`.

### Changed

- Reformat `CHANGELOG.md`.
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15), `regex` (1.7.1).

### Fixed

- Resolve Clippy warnings: `PartialEq` without `Eq`, `bool_to_int_with_if`, `uninlined_format_args`.

## [0.1.30] - 2022-06-27

### Added

- Support for multiple match expressions.
- Multi-color match test case.

## [0.1.29] - 2022-06-18

### Changed

- Update to Rust 2021 edition.
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), `semver` (1.0.10).

## [0.1.28] - 2022-05-22

### Changed

- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `exec-target` (0.2.5), `rust-version-info-file` (0.1.5).

## [0.1.27] - 2021-11-15

### Added

- Additional documentation.

### Changed

- Update MSRV to 1.47.0 (18bf6b4f0 2020-10-07).
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `rust-version-info-file` (0.1.3).

## [0.1.26] - 2021-09-11

### Added

- Dependency: `indoc` (1.0.3).

### Changed

- Resolve Cargo Clippy warnings.
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), `exec-target` (0.2.3).
- Rewrite `TARGET_EXE_PATH` using `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`.

## [0.1.25] - 2021-06-24

### Added

- `memx_cdy::memx_init()` for faster memory operations.

### Changed

- Rewrite `TARGET_EXE_PATH` using `env!("CARGO_BIN_EXE_aki-mcolor")`.

### Fixed

- Correct bug in `#[cfg(feature = "debian_build")]`.

## [0.1.24] - 2021-06-03

### Added

- `debian_build` feature.

### Changed

- Update dependencies: `flood-tide` (0.2.2), `regex` (1.5.4).

### Fixed

- Correct bug in `-X rust-version-info` command option.

## [0.1.23] - 2021-04-23

### Fixed

- Correct bug in `build.rs`.

## [0.1.22] - 2021-04-23

### Added

- `-X` command option.

### Changed

- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1), `regex` (1.4.6).

## [0.1.21] - 2021-04-19

### Changed

- Update dependency: `flood-tide-gen` (0.1.10).

## [0.1.20] - 2021-04-07

### Changed

- Update dependencies: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), `runnel` (0.3.6).

## [0.1.19] - 2021-03-22

### Added

- `execute_env()` and changes in handling of environments.
- Additional tests.
- Additional content to `--help`.

### Changed

- Update dependency: `regex` (1.4.5) to fix stack overflows.

## [0.1.18] - 2021-03-14

### Changed

- Update crate: `regex` to fix memory leak.

## [0.1.17] - 2021-03-08

### Changed

- Update crate: `runnel`.
- Update crate: `rustc_version` (0.3).

## [0.1.16] - 2021-03-08

### Changed

- Update crate: `regex` (1.4).
- Update crate: `runnel`.
- Rename `xtask/src/cmd.txt` to `xtask/src/aki-mcolor-cmd.txt`.

## [0.1.15] - 2021-03-02

### Added

- Additional documentation.

### Changed

- Rename environment variable `RUST_MCOLOR_RED_ST` to `AKI_MCOLOR_RED_ST`.
- Update crate: `flood-tide-gen`.
- Clean up `src/main.rs` and `build.rs`.

## [0.1.14] - 2021-02-22

### Changed

- Update crates: `runnel`, `flood-tide-gen`.

### Fixed

- Correct bug: Add `flush()` on finish.

## [0.1.13] - 2021-02-17

### Fixed

- Correct documentation bug: Output image.

## [0.1.12] - 2021-02-16

### Fixed

- Correct documentation bug: Color image.

## [0.1.11] - 2021-02-16

### Added

- Additional documentation.

### Changed

- Update crate: `runnel`.
- Rename section `AAA-admin` to `AAA-text` of `package.metadata.deb`.

## [0.1.10] - 2021-02-07

### Changed

- Initial GitHub release.

## 0.1.9 - 2021-02-07

### Added

- `xtask`.
- `stream` module.

### Changed

- Import crate `exec-target` from local for testing.
- Change `optpa_util_1` to `flood-tide` and `flood-tide-gen`.
- Change `AppError` to `anyhow::Error`.

## 0.1.8 - 2020-12-29

### Changed

- Update crates.

### Removed

- `optpaerr-1`.

## 0.1.7 - 2020-11-17

### Added

- `README.md`, `COPYING`, `LICENSE-APACHE`, and `LICENSE-MIT`.

### Changed

- Fix version pinning: `rustc_version (=0.2.3)` as `0.3.0` does not compile with new `semver` on Debian 10 Buster.
- Change `optpa_util` to `optpa_util_1`.

## 0.1.6 - 2020-08-09

### Added

- `cargo deb` support.

### Changed

- Update crates.

## 0.1.5 - 2020-05-10

### Changed

- Update edition from 2015 to 2018.
- Update crates.

## 0.1.4 - 2020-03-30

### Added

- Broken pipe support and test case.

### Changed

- Update crates.

## 0.1.3 - 2019-04-14

### Added

- `std::alloc` support.

### Changed

- Update crates.

## 0.1.2 - 2018-05-04

### Added

- `cfg(has_global_allocator)` support.

### Changed

- Update crates.

## 0.1.1 - 2018-03-22

### Added

- Broken pipe support.

### Changed

- Update crates.
- Miscellaneous improvements and refactoring.

## 0.1.0 - 2017-12-04

### Added

- First commit.

[Unreleased]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.32..v0.2.0
[0.1.32]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.31..v0.1.32
[0.1.31]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.30..v0.1.31
[0.1.30]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.29..v0.1.30
[0.1.29]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.28..v0.1.29
[0.1.28]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.27..v0.1.28
[0.1.27]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.26..v0.1.27
[0.1.26]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.25..v0.1.26
[0.1.25]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-mcolor/releases/tag/v0.1.10
