// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};

use crate::{KC11B04Key, KC11B04Map};

type Error<Adc, ADC, Word, Pin> = nb::Error<<Adc as OneShot<ADC, Word, Pin>>::Error>;

pub struct KC11B04<Pin, ADC> {
	pin: Pin,
	map: &'static KC11B04Map,
	_adc: PhantomData<ADC>,
}

impl<Pin, ADC> KC11B04<Pin, ADC>
where
	Pin: Channel<ADC>,
{
	pub fn new(pin: Pin, map: &'static KC11B04Map) -> Self
	where
		Pin: Channel<ADC>,
	{
		Self {
			pin,
			map,
			_adc: PhantomData,
		}
	}

	pub fn key_state<Adc>(
		&mut self,
		adc: &mut Adc,
	) -> Result<Option<KC11B04Key>, Error<Adc, ADC, u16, Pin>>
	where
		Adc: OneShot<ADC, u16, Pin>,
	{
		let val = adc.read(&mut self.pin)?;
		Ok(self.map.key_from_reading(val))
	}
}

#[cfg(test)]
mod test {
	use crate::{KC11B04Key, KC11B04, KC11B04_10BIT};
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
		let mut keypad = KC11B04::new(PIN, &KC11B04_10BIT);

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
				Ok(Some(KC11B04Key::K4)),
				Ok(Some(KC11B04Key::K3)),
				Ok(Some(KC11B04Key::K2)),
				Ok(Some(KC11B04Key::K1))
			)
		);
	}
}
