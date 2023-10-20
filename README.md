# KC11B04 Driver

Driver in Rust for the KC11B04 4-button analog keypad.

A 3-pin module using a voltage divider circuit and analog pin to sense a key press.

## Wiring notes

The voltage on the `VCC` pin should correspond with the reference voltage (maximum value) your ADC is configured to read.

For example the default on an Arduino Uno is that it matches the operating voltage of 5V.
But the ADC can be configured to use an external reference with the `AREF` pin. In this case `VCC` should match `AREF`.
