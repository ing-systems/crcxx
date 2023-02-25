//! The crate compute CRC-8/16/32/64 using various methods. It is applicable for small embedded systems and modern desktops and servers.
//! No unsafe or architecture specific code.
//!
//! # Usage of High Level API
//! TODO !
//!
//! # Usage of Low Level API
//!
//! The low level API don't handle preprocessing and postprocessing of CRC value.
//!
//! ### Processing using no lookup table, single byte per step
//!
//! The slowest method. No additional memory required.
//!
//! ```
//! use crcxx::internals::crc32;
//!
//! // CRC-32Q
//! const INIT: u32 = 0;
//! const POLY: u32 = 0x8141_41AB;
//! const WIDTH: u8 = 32;
//! const REFLECT: bool = false;
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = crc32::update_no_lut(INIT, WIDTH, POLY, REFLECT, &bytes);
//!
//!     assert_eq!(crc, 0x3010_BF7F);
//! }
//! ```
//! ### Processing using a lookup table with 32 entries, single byte per step
//!
//! Good compromise between speed and memory consumption for small embedded devices.
//! Depending on usage scenario usually 2-5 times faster than the previous method.
//!
//! ```
//! use crcxx::internals::crc32;
//!
//! // CRC-32Q
//! const INIT: u32 = 0;
//! const POLY: u32 = 0x8141_41AB;
//! const WIDTH: u8 = 32;
//! const REFLECT: bool = false;
//!
//! const LUT: [u32; 32] = crc32::make_lut_32(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = crc32::update_lut_32(INIT, &bytes, &LUT, REFLECT);
//!
//!     assert_eq!(crc, 0x3010_BF7F);
//! }
//! ```
//! ### Processing using a lookup table with 256 entries, single byte per step
//!
//! Depending on usage scenario usually no more than 2 times faster than the previous method.
//!
//! ```
//! use crcxx::internals::crc32;
//!
//! // CRC-32Q
//! const INIT: u32 = 0;
//! const POLY: u32 = 0x8141_41AB;
//! const WIDTH: u8 = 32;
//! const REFLECT: bool = false;
//!
//! const LUT: [u32; 256] = crc32::make_lut_256(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = crc32::update_lut_256(INIT, &bytes, &LUT, REFLECT);
//!
//!     assert_eq!(crc, 0x3010_BF7F);
//! }
//! ```
//! ### Processing using a lookup table with 256 x SLICES entries, multiple bytes per step
//!
//! Ultimate method for processing big amounts of data on modern desktops and servers.
//! Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
//! The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.
//!
//! ```
//! use crcxx::internals::crc32;
//!
//! // CRC-32Q
//! const INIT: u32 = 0;
//! const POLY: u32 = 0x8141_41AB;
//! const WIDTH: u8 = 32;
//! const REFLECT: bool = false;
//!
//! const SLICES: usize = 16;
//!
//! const LUT: [[u32; 256]; SLICES] = crc32::make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = crc32::update_lut_256x_n::<SLICES>(INIT, &bytes, &LUT, REFLECT);
//!
//!     assert_eq!(crc, 0x3010_BF7F);
//! }
//! ```
#![no_std]
#![forbid(non_ascii_idents, unsafe_code)]
#![deny(
    // macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_casts,
    // trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![warn(
    // unused_crate_dependencies,
    clippy::nursery,
    clippy::pedantic,
    clippy::mutex_atomic,
    clippy::rc_buffer,
    clippy::rc_mutex,
    // clippy::expect_used,
    // clippy::unwrap_used,
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::future_not_send,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    //clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub(crate) const MAX_SLICES: usize = 32;

#[macro_use]
pub mod internals;
