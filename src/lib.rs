#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod common;

pub mod crc16;
pub mod crc32;
pub mod crc64;
pub mod crc8;
#[macro_use]
pub mod lut;

#[cfg(test)]
mod tests;
