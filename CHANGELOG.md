# Changelog: aki-mcolor

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.1.32] (2024-06-19)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* miri supports on tests
* `tarpaulin` supports into `Makefile`

### Changed
* rename: `config` to `config.toml`
* remove: `cfg(has_not_matches)`
* refactored `Makefile`
* update depends: flood-tide(0.2.11), flood-tide-gen(0.1.22)
* update depends: memx-cdy(0.1.13), runnel(0.3.19)
* update depends: exec-taget(0.2.8), indoc(2.0.0), rust-version-info-file(0.1.10)

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* license files
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* clippy: `uninlined_format_args`, `unused_imports`, `derivable_impls`
* rust-version: "1.56.0" to "1.65.0"

## [0.1.31] (2023-01-11)
### Added
* badges into `README.tpl`

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: regex(1.7.1)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: bool_to_int_with_if
* clippy: uninlined_format_args

## [0.1.30] (2022-06-27)
### Added
* support multiple match exp
* multi color match to test

## [0.1.29] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)

## [0.1.28] (2022-05-22)
### Changed
* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.27] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.26] (2021-09-11)
### Added
* add depends: indoc(1.0.3)

### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.25] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-mcolor")`

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.24] (2021-06-03)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug fix command option: -X rust-version-info

## [0.1.23] (2021-04-23)
### Fixed
* bug: build.rs

## [0.1.22] (2021-04-23)
### Added
* add command option: `-X`

### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## [0.1.21] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.20] (2021-04-07)
### Changed
* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

## [0.1.19] (2021-03-22)
### Added
* `execute_env()`, and change in handling of environments
* some tests
* some contents to {}--help`

### Changed
* update depend: regex v1.4.5: fixes stack overflows

## [0.1.18] (2021-03-14)
### Changed
* update crate: regex: fix memory leak

## [0.1.17] (2021-03-08)
### Changed
* update crate: runnel
* update crate: rustc_version ("0.3")

## [0.1.16] (2021-03-08)
### Changed
* update crate: regex (1.4)
* update crate: runnel
* rename file: `xtask/src/cmd.txt` to `xtask/src/aki-mcolor-cmd.txt`

## [0.1.15] (2021-03-02)
### Added
* some documents

### Changed
* change env: RUST_MCOLOR_RED_ST to AKI_MCOLOR_RED_ST
* update crate: flood-tide-gen
* cleanup src/main.rs and build.rs

## [0.1.14] (2021-02-22)
### Changed
* update crate: runnel, flood-tide-gen

### Fixed
* bug: add flush() on finish.

## [0.1.13] (2021-02-17)
### Fixed
* bug doc: output image

## [0.1.12] (2021-02-16)
### Fixed
* bug doc: color image

## [0.1.11] (2021-02-16)
### Added
* add doc

### Changed
* update crate runnel
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

## [0.1.10] (2021-02-07)
### Changed
* initial github

## 0.1.9 (2021-02-07)
### Added
* add xtask
* add stream module

### Changed
* import crate exec-target from local, for test.
* change optpa_util_1 to flood-tide and flood-tied-gen
* change AppError to anyhow::Error

## 0.1.8 (2020-12-29)
### Changed
* update crates

### Removed
* remove optpaerr-1

## 0.1.7 (2020-11-17)
### Added
* `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`

### Changed
* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster
* change optpa_util to optpa_util_1

## 0.1.6 (2020-08-09)
### Added
* support `cargo deb`

### Changed
* update crates

## 0.1.5 (2020-05-10)
### Changed
* change edition 2015 to 2018.
* update crates

## 0.1.4 (2020-03-30)
### Added
* support broken pipe and test

### Changed
* update crates

## 0.1.3 (2019-04-14)
### Added
* support `std::alloc`

### Changed
* update crates

## 0.1.2 (2018-05-04)
### Added
* support `cfg(has_global_allocator)`

### Changed
* update crates

## 0.1.1 (2018-03-22)
### Added
* add support broken pipe

### Changed
* update crates
* a lot of things

## 0.1.0 (2017-12-04)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-mcolor/compare/v0.1.32..HEAD
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
