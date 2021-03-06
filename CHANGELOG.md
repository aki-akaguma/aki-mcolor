aki-mcolor TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.25 (2021-06-24)
=====

* add `memx_cdy::memx_init(); // fast mem operation.`
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-mcolor")`
* bug fix: `#[cfg(feature = "debian_build")]`

0.1.24 (2021-06-03)
=====

* add support features = \["debian_build"\]
* bug fix command option: -X rust-version-info
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

0.1.23 (2021-04-23)
=====

* fix build.rs

0.1.22 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* add command option: -X
* update depends: bug fix: regex(1.4.6)

0.1.21 (2021-04-19)
=====

* update depends: flood-tide-gen(0.1.10)

0.1.20 (2021-04-07)
=====

* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

0.1.19 (2021-03-22)
=====

* update depend: regex v1.4.5: fixes stack overflows
* add execute_env(), and change in handling of environments
* add some tests
* add some contents to --help

0.1.18 (2021-03-14)
=====

* update crate: regex: fix memory leak

0.1.17 (2021-03-08)
=====

* update crate: runnel
* update crate: rustc_version ("0.3")

0.1.16 (2021-03-08)
=====

* update crate: regex (1.4)
* update crate: runnel
* rename file: xtask/src/cmd.txt to xtask/src/aki-mcolor-cmd.txt

0.1.15 (2021-03-02)
=====
Feature:

* change env: RUST_MCOLOR_RED_ST to AKI_MCOLOR_RED_ST
* update crate: flood-tide-gen
* add some documents
* cleanup src/main.rs and build.rs

0.1.14 (2021-02-22)
=====
Feature:

* fix bug: add flush() on finish.
* update crate: runnel, flood-tide-gen

0.1.13 (2021-02-17)
=====
Feature:

* fix doc: output image

0.1.12 (2021-02-16)
=====
Feature:

* fix doc: color image

0.1.11 (2021-02-16)
=====
Feature:

* update crate runnel
* add doc
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

0.1.10 (2021-02-07)
=====
Feature:

* initial github

0.1.9 (2021-02-07)
=====
Feature:

* import crate exec-target from local, for test.
* add xtask
* add stream module
* change optpa_util_1 to flood-tide and flood-tied-gen
* change AppError to anyhow::Error

0.1.8 (2020-12-29)
=====
Feature:

* update crates
* remove optpaerr-1

0.1.7 (2020-11-17)
=====
Feature:

* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* change optpa_util to optpa_util_1

0.1.6 (2020-08-09)
=====
Feature:

* add support cargo deb
* update crates

0.1.5 (2020-05-10)
=====
Feature:

* change edition 2015 to 2018.
* update crates

0.1.4 (2020-03-30)
=====
Feature:

* add support broken pipe and test
* update crates

0.1.3 (2019-04-14)
=====
Feature:

* add support std::alloc
* update crates

0.1.2 (2018-05-04)
=====
Feature:

* add support cfg(has_global_allocator)
* update crates

0.1.1 (2018-03-22)
=====
Feature:

* add support broken pipe
* update crates
* a lot of things

0.1.0 (2017-12-04)
=====
first commit
