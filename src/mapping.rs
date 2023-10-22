// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! This module maps keys to their expected ADC readings.
//!
//! A specific mapping is represented as a [`KeyMap`].
//! Their values depend on the resolution of your ADC (8bit, 10bit, 12bit, etc.).
//! Several predefined mappings are exported in the root of the crate, such as [`MAP_10BIT`][crate::MAP_10BIT].
//! See the crate's [Constants][crate#constants] for this.
//!
//! ## Custom maps
//!
//! You can define custom maps. However floating point arithmetic is slow and inconsistent between different targets.
//! It's also *totally unnecessary to do at runtime* because we'll only ever compare integers against integers.
//!
//! You can of course do your own math ahead of time and write down literal integers.
//!
//! ```rust
//! use kc11b04::KeyMap;
//!
//! // The 10-bit KeyMap from the manufacturer's example table.
//! let map = KeyMap {
//! 	k1: 404,
//! 	k2: 607,
//! 	k3: 812,
//! 	k4: 1023,
//! 	margin: 30,
//! };
//! ```
//!
//! Or you could perform all floating point math in a constant expression.
//! Letting the compiler do this math *at compile time*, keeping the binary small and
//! avoiding the performance hit on your embedded device.
//!
//! ```rust
//! use kc11b04::KeyMap;
//!
//! // Floating point math is supported in constant definitions.
//! // And should "compile away" into just the final integers.
//! const CUSTOM_MAP: KeyMap = {
//! 	use kc11b04::mapping::{K1_F, K2_F, K3_F};
//! 	let max = 1023;
//! 	let margin = 0.03;
//! 	KeyMap {
//! 		k1: (max as f32 * K1_F) as u16,
//! 		k2: (max as f32 * K2_F) as u16,
//! 		k3: (max as f32 * K3_F) as u16,
//! 		k4: max,
//! 		margin: (max as f32 * margin) as u16,
//! 	}
//! };
//! ```
//!
//! This is effectively what the [`map_from_max!`][crate::map_from_max] macro allows you to do as well.
//!
//! ```rust
//! use kc11b04::{KeyMap, map_from_max};
//!
//! /// 10bit map, but with 15% margin.
//! const CUSTOM_MAP: KeyMap = map_from_max!(1023, 0.15);
//! ```
//!
//! ## Schematic and factors
//!
//! While the manufacturer provides a table of voltages, it's actually off slightly.
//! For 5V on `VCC` they list:
//! - K4 = 5V (100%)
//! - K3 = 4V (80%)
//! - K2 = 3V (60%)
//! - K1 = 2V (40%)
//!
//! This *roughly* lines up but not exactly.
//! Both my ADC and volt meter find slightly different values.
//! Taking a closer look at the schematic for the module:
//!
//! ![KC11B04 schematic][kc11b04-schema]
//!
//! We can see a voltage divider on the left based on the key being pressed.
//! However, there is also a pull-down resistor and a capacitor on the right side.
//! The simple table does not account of the pull-down resistor affecting the voltage divider.
//!
//! Ignoring the capacitor (which doesn't affect the stable state much) but adding the pull-down resistor `R5`,
//! the *equivalent resistors* become:
//!
//! - K1 = 3000 ohm : ~1961 ohm (`1/((1/2000) + (1/100000))`)
//! - K2 = 2000 ohm : ~2913 ohm (`1/((1/3000) + (1/100000))`)
//! - K3 = 1000 ohm : ~3846 ohm (`1/((1/4000) + (1/100000))`)
//! - K4 = 0 ohm : 100K ohm
//!
//! The formula then becomes `r2 / (r1 + r2)`.
//! For instance for K1: `~1961 / (3000 + ~1961) = ~0.395`.
//!
//! These factors are stored in `f32` resolution as constants.
//! And the new table becomes:
//!
//! | Key  | Constant | Fixed percent | Old percent |
//! | ---- | -------- | ------------- | ----------- |
//! | None | -        | 0%            | 0%          |
//! | K1   | [`K1_F`] | ~39.5%        | 40%         |
//! | K2   | [`K2_F`] | ~59.3%        | 60%         |
//! | K3   | [`K3_F`] | ~79.4%        | 80%         |
//! | K4   | -        | 100%          | 100%        |
//!
#![cfg_attr(
	feature = "doc-images",
	doc = ::embed_doc_image::embed_image!("kc11b04-schema", "docs/KC11B04-schema.svg")
)]
#![cfg_attr(
	not(feature = "doc-images"),
	doc = "[kc11b04-schema]: docs/KC11B04-schema.svg"
)]

use crate::Key;

/// Maps keys to their expected ADC readings.
///
/// This depends on the ADC resolution, so you may want to use a constant for this such as [`MAP_10BIT`][crate::MAP_10BIT].
///
/// For creating a custom map, see the module documentation [`kc11b04::mapping`][crate::mapping].
#[cfg_attr(feature = "defmt-0-3", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt-0-2", derive(ufmt::derive::uDebug))]
#[derive(Debug)]
pub struct KeyMap {
	/// The expected ADC reading for K1, before margins.
	///
	/// For predefined maps it's [`K1_F`] times the max reading of the ADC.
	pub k1: u16,

	/// The expected ADC reading for K2, before margins.
	///
	/// For predefined maps it's [`K2_F`] times the max reading of the ADC.
	pub k2: u16,

	/// The expected ADC reading for K3, before margins.
	///
	/// For predefined maps it's [`K3_F`] times the max reading of the ADC.
	pub k3: u16,

	/// The expected ADC reading for K4, before margins.
	///
	/// For predefined maps it's equal to the max reading of the ADC.
	pub k4: u16,

	/// The absolute margin a reading may deviate from the above expected values.
	/// The default is `3%` of the max reading of the ADC.
	pub margin: u16,
}

impl KeyMap {
	/// Takes an ADC reading and finds whether it's in the expected range of a key.
	///
	/// Will be [`None`] when no key is pressed, but also for some simultaneous key combinations.
	pub const fn key_from_reading(&self, val: u16) -> Option<Key> {
		match val {
			v if v > self.k1 - self.margin && v < self.k1 + self.margin => Some(Key::K1),
			v if v > self.k2 - self.margin && v < self.k2 + self.margin => Some(Key::K2),
			v if v > self.k3 - self.margin && v < self.k3 + self.margin => Some(Key::K3),
			v if v > self.k4 - self.margin => Some(Key::K4),
			_ => None,
		}
	}
}

/// Pull-down resistor value 100K ohms.
const R_DOWN: f32 = 100_000.0;

macro_rules! make_factor {
	($r1:literal, $r_rest:literal, $r_down:ident) => {{
		let r2 = 1.0 / ((1.0 / $r_rest) + (1.0 / $r_down));
		r2 / ($r1 + r2)
	}};
}

/// Relative factor for K1 button, `~39.5%` of the ADC's max reading.
///
/// See the module documentation [`kc11b04::mapping`][crate::mapping] for details.
pub const K1_F: f32 = make_factor!(3000.0, 2000.0, R_DOWN);

/// Relative factor for K2 button, `~59.3%` of the ADC's max reading.
///
/// See the module documentation [`kc11b04::mapping`][crate::mapping] for details.
pub const K2_F: f32 = make_factor!(2000.0, 3000.0, R_DOWN);

/// Relative factor for K3 button, `~79.4%` of the ADC's max reading.
///
/// See the module documentation [`kc11b04::mapping`][crate::mapping] for details.
pub const K3_F: f32 = make_factor!(1000.0, 4000.0, R_DOWN);

/// Defines a [`KeyMap`] based on the max reading of the ADC and optional margin factor.<br>
/// For example `map_from_max!(1023, 0.15)` for a 10bit ADC and 15% margin.
///
/// The margin defaults to `0.03` (3%) if omitted.
///
/// ## Performance note
///
/// This macro will use floating point math. It's strongly recommended to
/// **declare the result as a constant**, to avoid this FP math at runtime.
///
/// ```rust
/// use kc11b04::{KeyMap, map_from_max};
///
/// /// 10bit map, but with 15% margin.
/// const CUSTOM_MAP: KeyMap = map_from_max!(1023, 0.15);
/// ```
#[macro_export]
macro_rules! map_from_max {
	($max:literal) => {
		map_from_max!($max, 0.03)
	};
	($max:literal, $margin:literal) => {
		KeyMap {
			k1: ($max as f32 * $crate::mapping::K1_F) as u16,
			k2: ($max as f32 * $crate::mapping::K2_F) as u16,
			k3: ($max as f32 * $crate::mapping::K3_F) as u16,
			k4: $max,
			margin: ($max as f32 * $margin) as u16,
		}
	};
}

#[test]
fn read_10bit_samples() {
	let map = crate::map_from_max!(1023, 0.03);
	assert_eq!(
		(
			map.key_from_reading(0),
			map.key_from_reading(300),
			map.key_from_reading(400),
			map.key_from_reading(500),
			map.key_from_reading(600),
			map.key_from_reading(700),
			map.key_from_reading(800),
			map.key_from_reading(900),
			map.key_from_reading(1023)
		),
		(
			None,
			None,
			Some(Key::K1),
			None,
			Some(Key::K2),
			None,
			Some(Key::K3),
			None,
			Some(Key::K4)
		)
	);
}
