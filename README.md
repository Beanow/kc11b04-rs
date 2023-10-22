# KC11B04 Driver &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs]

[Latest Version]: https://img.shields.io/crates/v/kc11b04.svg
[crates.io]: https://crates.io/crates/kc11b04
[Docs Badge]: https://docs.rs/kc11b04/badge.svg
[docs]: https://docs.rs/kc11b04

Driver in Rust for the KC11B04 4-button analog keypad.

A 3-pin module using a voltage divider circuit and analog pin to sense a key press.

![KC11B04 image][__link0]


## Wiring notes

The voltage on the `VCC` pin should correspond with the reference voltage (maximum value) your ADC is configured to read.

For example the default on an Arduino Uno is that it matches the operating voltage of 5V. But the ADC can be configured to use an external reference with the `AREF` pin. In this case `VCC` should match `AREF`.


## Example


```rust
use kc11b04::{Key, KC11B04, MAP_10BIT};

let mut adc = /* Configure your ADC using its HAL */
let mut ad_pin = /* Set the pin connected to AD as analog input */

// Create the keypad driver. Taking ownership of the pin,
// providing a map that matches the resolution of your ADC.
let mut keypad = KC11B04::new(ad_pin, MAP_10BIT);

// Somewhere within loop { }
// Read current key state.
match keypad
	.key_state(&mut adc)
	.expect("Problem reading ADC channel")
{
	Some(Key::K4) => { /* K4 key being pressed */ }
	Some(Key::K3) => { /* K3 key being pressed */ }
	Some(Key::K2) => { /* K2 key being pressed */ }
	Some(Key::K1) => { /* K1 key being pressed */ }
	None => { /* Either nothing, or multiple keys pressed */ }
}
```


## MSRV & Stability

This crate compiles on stable Rust `1.60` and up. It *might* still work on older rust versions, but this isn’t ensured. Upgrading the MSRV is considered SemVer breaking.


 [__link0]: docs/KC11B04.webp
