# Changelog

## \[0.3.0]

- [`2fc97fa`](https://github.com/Beanow/kc11b04-rs/commit/2fc97fadb95830a9b61aa38cd212dfc19dc09300)([#10](https://github.com/Beanow/kc11b04-rs/pull/10)) **Breaking**: support being generic over `Word` instead of assuming `u16` values from the ADC.

  This changes the type from `KeyMap` to `KeyMap<Word>` and adds a type as the first argument to the `map_from_max!` macro. For example:

  ```rust
  /// Signed 11bit map. Meaning 10 bits of resolution in the positive voltage range.
  const MAP_11BIT_SIGNED: KeyMap<i16> = map_from_max!(i16, 1023);
  ```

## \[0.2.1]

- [`4e15e28`](https://github.com/Beanow/kc11b04-rs/commit/4e15e28d236994c1513c84216757c8fb67829806)([#8](https://github.com/Beanow/kc11b04-rs/pull/8)) Include doc images in package which should fix docs.rs builds.

## \[0.2.0]

- [`1c3c5f5`](https://github.com/Beanow/kc11b04-rs/commit/1c3c5f512f2a8e6f637012ffdbd6742b213cc888)([#7](https://github.com/Beanow/kc11b04-rs/pull/7)) Custom maps can now be created.

  Either using the macro `map_from_max!` or constructing a `KeyMap` manually.
  See [`kc11b04::mapping`](https://docs.rs/kc11b04/0.2.1/kc11b04/mapping/index.html) docs.

- [`4e43c76`](https://github.com/Beanow/kc11b04-rs/commit/4e43c7604ab655606ff3343a40e00afcb9922469)([#5](https://github.com/Beanow/kc11b04-rs/pull/5)) Expand documentation with examples and images.

## \[0.1.0]

- [`f6a2ab0`](https://github.com/Beanow/kc11b04-rs/commit/f6a2ab0e341048e9d4ef19034f4af3c2c8092a53) Initial release of the crate :tada:
