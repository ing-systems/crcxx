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
pub mod lut;

#[cfg(test)]
mod tests;
