# EFM32XG Peripheral Access Crate (PAC)

[![crates.io](https://img.shields.io/crates/v/efm32xg-pac)](https://crates.io/crates/efm32xg-pac)
[![docs.rs](https://docs.rs/efm32xg-pac/badge.svg)](https://docs.rs/efm32xg-pac)

Low-level register mappings for Silicon Labs EFM32 ARM Cortex-M microcontrollers, written in Rust.
The code is generated automatically from vendor-supplied SVD files using [chiptool](https://github.com/embassy-rs/chiptool).

## Supported chips

| Feature       | Device family                                         | Core            |
| ------------- | ----------------------------------------------------- | --------------- |
| `efm32pg1b`   | [EFM32PG1B Pearl Gecko](https://www.silabs.com/mcu/32-bit/efm32-pearl-gecko) | Cortex-M4 |
| `efm32gg11`   | [EFM32GG11 Giant Gecko](https://www.silabs.com/mcu/32-bit/efm32-giant-gecko)   | Cortex-M4 |

Enable exactly one chip feature. The default feature set is `critical-section`, `rt`, and `efm32pg1b`.

## Usage

```toml
[dependencies.efm32xg-pac]
version = "0.1"
features = ["efm32gg11"]   # pick your chip
```

Optional features:

- `rt` — interrupt vector table via `cortex-m-rt` (enabled by default)
- `critical-section` — `critical-section` impl (enabled by default)
- `defmt` — `defmt::Format` impls for all registers and enums
- `atomics` — `portable-atomic` based atomic accessors

## Regenerating the PAC

The SVD files and chiptool transform pipelines live under `svd/`. To regenerate
the Rust source, install [`chiptool`](https://github.com/embassy-rs/chiptool)
and [`form`](https://crates.io/crates/form), then run:

```sh
./update.sh
```

This regenerates `src/common.rs` and the per-chip modules under `src/efm32*/`,
then checks both chip features compile.

## Documentation

Vendor supplied documents:

### EFM32PG1B

- [Datasheet](https://www.silabs.com/documents/public/data-sheets/efm32pg1-datasheet.pdf)
- [Reference Manual](https://www.silabs.com/documents/public/reference-manuals/EFM32PG1-ReferenceManual.pdf)
- [Errata](https://www.silabs.com/documents/public/errata/efm32pg1-errata.pdf)

### EFM32GG11

- [Datasheet](https://www.silabs.com/documents/public/data-sheets/efm32gg11-datasheet.pdf)
- [Reference Manual](https://www.silabs.com/documents/public/reference-manuals/efm32gg11-rm.pdf)
- [Errata](https://www.silabs.com/documents/public/errata/efm32gg11-errata.pdf)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
