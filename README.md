# KC11B04 Driver &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs]

[Latest Version]: https://img.shields.io/crates/v/kc11b04.svg
[crates.io]: https://crates.io/crates/kc11b04
[Docs Badge]: https://docs.rs/kc11b04/badge.svg
[docs]: https://docs.rs/kc11b04

Driver in Rust for the KC11B04 4-button analog keypad.

A 3-pin module using a voltage divider circuit and analog pin to sense a key press.

## Wiring notes

The voltage on the `VCC` pin should correspond with the reference voltage (maximum value) your ADC is configured to read.

For example the default on an Arduino Uno is that it matches the operating voltage of 5V.
But the ADC can be configured to use an external reference with the `AREF` pin. In this case `VCC` should match `AREF`.

## MSRV & Stability

This crate compiles on stable Rust `1.60` and up. It _might_ still work on older rust versions, but this isn't ensured. Upgrading the MSRV is considered SemVer breaking.
