// Copyright 2023-2023 Robin van Boven
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![deny(unsafe_code)]
// #![deny(missing_docs)]
#![no_std]

mod driver;
mod map;

pub use driver::*;
pub use map::*;

#[cfg_attr(any(feature = "debug", test), derive(Debug))]
#[cfg_attr(feature = "udebug", derive(ufmt::derive::uDebug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KC11B04Key {
	K1 = 1,
	K2,
	K3,
	K4,
}
