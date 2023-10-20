// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::KC11B04Key;

#[cfg_attr(feature = "defmt-0-3", derive(defmt::Format))]
#[derive(Debug)]
pub struct KC11B04Map {
	k1: u16,
	k2: u16,
	k3: u16,
	k4: u16,
	margin: u16,
}

impl KC11B04Map {
	pub const fn key_from_reading(&self, val: u16) -> Option<KC11B04Key> {
		match val {
			v if v > self.k1 - self.margin && v < self.k1 + self.margin => Some(KC11B04Key::K1),
			v if v > self.k2 - self.margin && v < self.k2 + self.margin => Some(KC11B04Key::K2),
			v if v > self.k3 - self.margin && v < self.k3 + self.margin => Some(KC11B04Key::K3),
			v if v > self.k4 - self.margin => Some(KC11B04Key::K4),
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
const K1_F: f32 = 0.3952569;
const K2_F: f32 = 0.5928853;
const K3_F: f32 = 0.7936507;

pub const KC11B04_10BIT: KC11B04Map = {
	let max = 1023;
	let margin = 0.03;
	KC11B04Map {
		k1: (max as f32 * K1_F) as u16,
		k2: (max as f32 * K2_F) as u16,
		k3: (max as f32 * K3_F) as u16,
		k4: max,
		margin: (max as f32 * margin) as u16,
	}
};

#[test]
fn read_10bit_samples() {
	let map = KC11B04_10BIT;
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
			Some(KC11B04Key::K1),
			None,
			Some(KC11B04Key::K2),
			None,
			Some(KC11B04Key::K3),
			None,
			Some(KC11B04Key::K4)
		)
	);
}
