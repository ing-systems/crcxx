#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub(crate) const MAX_SLICES: usize = 32;

#[macro_use]
mod cg_assert;
#[macro_use]
pub mod lut;
#[macro_use]
mod update;

pub mod crc16;
pub mod crc32;
pub mod crc64;
pub mod crc8;

#[cfg(test)]
mod tests;
