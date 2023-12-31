// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

//! Driver in Rust for the KC11B04 4-button analog keypad.
//!
//! A 3-pin module using a voltage divider circuit and analog pin to sense a key press.
//!
//! ![KC11B04 image][kc11b04-image]
//!
//! # Wiring notes
//!
//! The voltage on the `VCC` pin should correspond with the reference voltage (maximum value) your ADC is configured to read.
//!
//! For example the default on an Arduino Uno is that it matches the operating voltage of 5V.
//! But the ADC can be configured to use an external reference with the `AREF` pin. In this case `VCC` should match `AREF`.
//!
//! # Example
//!
//! ```rust
//! # use embedded_hal_mock::adc::*;
//! use kc11b04::{Key, KC11B04, MAP_10BIT};
//!
//! let mut adc = /* Configure your ADC using its HAL */
//! # Mock::new(&[Transaction::read(0, 1023)]);
//! let mut ad_pin = /* Set the pin connected to AD as analog input */
//! # MockChan0;
//!
//! // Create the keypad driver. Taking ownership of the pin,
//! // providing a map that matches the resolution of your ADC.
//! let mut keypad = KC11B04::new(ad_pin, MAP_10BIT);
//!
//! // Somewhere within loop { }
//! // Read current key state.
//! match keypad
//! 	.key_state(&mut adc)
//! 	.expect("Problem reading ADC channel")
//! {
//! 	Some(Key::K4) => { /* K4 key being pressed */ }
//! 	Some(Key::K3) => { /* K3 key being pressed */ }
//! 	Some(Key::K2) => { /* K2 key being pressed */ }
//! 	Some(Key::K1) => { /* K1 key being pressed */ }
//! 	None => { /* Either nothing, or multiple keys pressed */ }
//! }
//! ```
//!
//! # MSRV policy
//!
//! Upgrading the MSRV is considered SemVer breaking.
//! The MSRV will be set in `package.rust-version`.
//!
#![cfg_attr(
	feature = "doc-images",
	doc = ::embed_doc_image::embed_image!("kc11b04-image", "docs/KC11B04.webp")
)]
#![cfg_attr(
	not(feature = "doc-images"),
	doc = "[kc11b04-image]: docs/KC11B04.webp"
)]

mod driver;
pub mod mapping;

pub use driver::*;
pub use mapping::KeyMap;

/// A named key on the [KC11B04][crate] module.
#[cfg_attr(feature = "defmt-0-3", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt-0-2", derive(ufmt::derive::uDebug))]
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

/// [`KeyMap`] for 8bit ADCs with a maximum reading of `255`.
pub const MAP_8BIT: KeyMap<u8> = map_from_max!(u8, 255);

/// [`KeyMap`] for 10bit ADCs with a maximum reading of `1023`.
pub const MAP_10BIT: KeyMap<u16> = map_from_max!(u16, 1023);

/// [`KeyMap`] for 12bit ADCs with a maximum reading of `4095`.
pub const MAP_12BIT: KeyMap<u16> = map_from_max!(u16, 4095);

/// [`KeyMap`] for signed 12bit ADCs with a maximum *positive* reading of `2047`.
pub const MAP_12BIT_SIGNED: KeyMap<i16> = map_from_max!(i16, 2047);

/// [`KeyMap`] for 16bit ADCs with a maximum reading of `65535`.
pub const MAP_16BIT: KeyMap<u16> = map_from_max!(u16, 65535);

/// [`KeyMap`] for signed 16bit ADCs with a maximum *positive* reading of `32767`.
pub const MAP_16BIT_SIGNED: KeyMap<i16> = map_from_max!(i16, 32767);

/// [`KeyMap`] for 24bit ADCs with a maximum reading of `16777215`.
pub const MAP_24BIT: KeyMap<u32> = map_from_max!(u32, 0xFFFFFF);
