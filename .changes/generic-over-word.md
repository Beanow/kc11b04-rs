---
"kc11b04": minor
---

**Breaking**: support being generic over `Word` instead of assuming `u16` values from the ADC.

This changes the type from `KeyMap` to `KeyMap<Word>` and adds a type as the first argument to the `map_from_max!` macro. For example:

```rust
/// Signed 11bit map. Meaning 10 bits of resolution in the positive voltage range.
const MAP_11BIT_SIGNED: KeyMap<i16> = map_from_max!(i16, 1023);
```
