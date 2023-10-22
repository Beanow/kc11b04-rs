// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};

use crate::{Key, KeyMap};

/// KC11B04 analog keypad driver. Constructed with [`KC11B04::new`].
pub struct KC11B04<Pin, ADC> {
	pin: Pin,
	map: KeyMap,
	_adc: PhantomData<ADC>,
}

type Error<Adc, ADC, Word, Pin> = nb::Error<<Adc as OneShot<ADC, Word, Pin>>::Error>;

impl<Pin, ADC> KC11B04<Pin, ADC>
where
	Pin: Channel<ADC>,
{
	/// Create a [`KC11B04`] instance for the given ADC pin / channel and mapping.
	///
	/// ```rust
	/// # use embedded_hal_mock::adc::*;
	/// # use kc11b04::{Key, KC11B04};
	/// #
	/// # let mut adc = Mock::new(&[Transaction::read(0, 1023)]);
	/// # let analog_pin = MockChan0;
	/// #
	/// // Create the keypad driver. Taking ownership of the pin,
	/// // providing a map that matches the resolution of your ADC.
	/// let mut keypad = KC11B04::new(analog_pin, kc11b04::MAP_10BIT);
	/// ```
	pub fn new(pin: Pin, map: KeyMap) -> Self
	where
		Pin: Channel<ADC>,
	{
		Self {
			pin,
			map,
			_adc: PhantomData,
		}
	}

	/// Takes an ADC reading and finds whether a key is currently being pressed.
	///
	/// Will be [`None`] when no key is pressed, but also for some simultaneous key combinations.
	pub fn key_state<Adc>(
		&mut self,
		adc: &mut Adc,
	) -> Result<Option<Key>, Error<Adc, ADC, u16, Pin>>
	where
		Adc: OneShot<ADC, u16, Pin>,
	{
		let val = adc.read(&mut self.pin)?;
		Ok(self.map.key_from_reading(val))
	}
}

#[cfg(test)]
mod test {
	use crate::{Key, KC11B04, MAP_10BIT};
	use embedded_hal::adc::Channel;
	use embedded_hal_mock::adc::{Mock, MockChan0, Transaction};

	#[test]
	fn reads_given_channel() {
		use MockChan0 as PIN;

		let expected = [
			Transaction::read(PIN::channel(), 0),
			Transaction::read(PIN::channel(), 1023),
			Transaction::read(PIN::channel(), 800),
			Transaction::read(PIN::channel(), 600),
			Transaction::read(PIN::channel(), 400),
		];

		let mut adc = Mock::new(&expected);
		let mut keypad = KC11B04::new(PIN, MAP_10BIT);

		assert_eq!(
			(
				keypad.key_state(&mut adc),
				keypad.key_state(&mut adc),
				keypad.key_state(&mut adc),
				keypad.key_state(&mut adc),
				keypad.key_state(&mut adc)
			),
			(
				Ok(None),
				Ok(Some(Key::K4)),
				Ok(Some(Key::K3)),
				Ok(Some(Key::K2)),
				Ok(Some(Key::K1))
			)
		);
	}
}
