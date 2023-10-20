// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! Driver in Rust for the KC11B04 4-button analog keypad.
//!
//! A 3-pin module using a voltage divider circuit and analog pin to sense a key press.
//!
//! # Wiring notes
//!
//! The voltage on the `VCC` pin should correspond with the reference voltage (maximum value) your ADC is configured to read.
//!
//! For example the default on an Arduino Uno is that it matches the operating voltage of 5V.
//! But the ADC can be configured to use an external reference with the `AREF` pin. In this case `VCC` should match `AREF`.
//!
//! # MSRV & Stability
//!
//! This crate compiles on stable Rust `1.60` and up. It _might_ still work on older rust versions, but this isn't ensured. Upgrading the MSRV is considered SemVer breaking.

mod driver;
mod map;

pub use driver::*;
pub use map::*;

/// A named key on the [KC11B04][crate] module.
#[cfg_attr(feature = "defmt-0-3", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
	/// `K1` key on the [KC11B04][crate] module.
	K1 = 1,
	/// `K2` key on the [KC11B04][crate] module.
	K2,
	/// `K3` key on the [KC11B04][crate] module.
	K3,
	/// `K4` key on the [KC11B04][crate] module.
	K4,
}
