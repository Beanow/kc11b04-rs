// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! TODO: module docs

use crate::Key;

/// Maps keys to their expected ADC readings.
///
/// This depends on the ADC resolution, so you may want to use a constant for this such as [`MAP_10BIT`][crate::MAP_10BIT].
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

/*
	Voltage divider with equivalent parallel resistors.
	NONE = AD is floating
	K1 = 3000 : ~1960 ... 1/((1/2000) + (1/100000))
	K2 = 2000 : ~2912 ... 1/((1/3000) + (1/100000))
	K3 = 1000 : ~3846 ... 1/((1/4000) + (1/100000))
	K4 = 0 : 100K
*/

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
///	For example `map_from_max!(1023, 0.15)` for a 10bit ADC and 15% margin.
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

/// [`KeyMap`] for 8bit ADCs with a maximum reading of `255`.
pub const MAP_8BIT: KeyMap = map_from_max!(255);

/// [`KeyMap`] for 10bit ADCs with a maximum reading of `1023`.
pub const MAP_10BIT: KeyMap = map_from_max!(1023);

/// [`KeyMap`] for 12bit ADCs with a maximum reading of `4095`.
pub const MAP_12BIT: KeyMap = map_from_max!(4095);

/// [`KeyMap`] for 16bit ADCs with a maximum reading of `65535`.
pub const MAP_16BIT: KeyMap = map_from_max!(65535);

#[test]
fn read_10bit_samples() {
	let map = MAP_10BIT;
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
