# Changelog

## [Unreleased]


## v0.1.0 - 2026-09-06

First release of `efm32xg-pac`, the multi-chip peripheral access crate for
Silicon Labs EFM32 microcontrollers, generated with
[chiptool](https://github.com/embassy-rs/chiptool).

This crate continues and supersedes `efm32pg1b-pac` (which used svd2rust and
covered only EFM32PG1B).  Notable changes from the last `efm32pg1b-pac`
release:

* Switched code generator from svd2rust to chiptool.
* Added EFM32GG11 (Giant Gecko) as a second supported chip, selected via the
  `efm32gg11` feature.
* Per-chip SVD transforms (instance block merging, register clustering,
  fieldset/enum dedup, PRSSEL/LOCKKEY enum merging, GPIO port clustering,
  LDMA/TIMER/USB/LESENSE/VDAC register clusters, readable field renames).
* `defmt` and `atomics` features made explicit via `dep:` syntax.
* Dependency floors bumped: critical-section 1.2, cortex-m 0.7.9,
  cortex-m-rt 0.7.6, defmt 1.1, portable-atomic 1.15.
