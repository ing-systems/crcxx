//! The crate compute CRC-8/16/32/64/128 using various methods. Included catalog of CRC parameters simplify usage.
//! It is applicable from small embedded systems to modern desktops and servers. No unsafe or architecture specific code.
//!
//! ### Processing using no lookup table, single byte per step
//!
//! The slowest method. No additional memory required.
//!
//! ```
//! use crcxx::crc32::{*, catalog::CRC_32_BZIP2};
//!
//! const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC_32_BZIP2);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = CRC.calculate(&bytes);
//!
//!     assert_eq!(crc, 0xFC89_1918);
//! }
//! ```
//! ### Processing using a lookup table with 32 entries, single byte per step
//!
//! Good compromise between speed and memory consumption for small embedded devices.
//! Depending on usage scenario usually 2-5 times faster than the previous method.
//! ```
//! use crcxx::crc32::{*, catalog::CRC_32_BZIP2};
//!
//! const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC_32_BZIP2);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = CRC.calculate(&bytes);
//!
//!     assert_eq!(crc, 0xFC89_1918);
//! }
//! ```
//! ### Processing using a lookup table with 256 entries, single byte per step
//!
//! Depending on usage scenario usually no more than 2 times faster than the previous method.
//! ```
//! use crcxx::crc32::{*, catalog::CRC_32_BZIP2};
//!
//! const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC_32_BZIP2);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = CRC.calculate(&bytes);
//!
//!     assert_eq!(crc, 0xFC89_1918);
//! }
//! ```
//! ### Processing using a lookup table with 256 x SLICES entries, multiple bytes per step
//!
//! Ultimate method for processing big amounts of data on modern desktops and servers.
//! Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
//! The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.
//! ```
//! use crcxx::crc32::{*, catalog::CRC_32_BZIP2};
//!
//! const SLICES: usize = 16;
//! const CRC: Calculator<LookupTable256xN<SLICES>> = Calculator::<LookupTable256xN<SLICES>>::new(&CRC_32_BZIP2);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = CRC.calculate(&bytes);
//!
//!     assert_eq!(crc, 0xFC89_1918);
//! }
//! ```
#![no_std]
#![forbid(non_ascii_idents, unsafe_code)]
#![deny(
    // macro_use_extern_crate,
    missing_copy_implementations,
    // missing_debug_implementations,
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

use num_traits::{PrimInt, Unsigned};

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod internals;
#[macro_use]
mod macros;

pub mod crc128;
pub mod crc16;
pub mod crc32;
pub mod crc64;
pub mod crc8;

pub trait Width: PrimInt + Unsigned {}
impl Width for u8 {}
impl Width for u16 {}
impl Width for u32 {}
impl Width for u64 {}
impl Width for u128 {}

// !!! Copy-paste from crc-catalog crate. !!!
/// CRC calculation paremeters
///
/// The struct describes a CRC algorithm using the fields specified by the [Catalogue of
/// parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
#[derive(Debug)]
pub struct Params<W: Width> {
    /// The number of bit cells in the linear feedback shift register; the degree of the generator
    /// polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the shift register. The
    /// least significant bit corresponds to the inward end of the shift register, and is always
    /// set. The highest-order term is omitted.
    pub poly: W,
    /// The settings of the bit cells at the start of each calculation, before reading the first
    /// message bit. The least significant bit corresponds to the inward end of the shift register.
    pub init: W,
    /// If equal to `false`, specifies that the characters of the message are read bit-by-bit, most
    /// significant bit (MSB) first; if equal to `true`, the characters are read bit-by-bit, least
    /// significant bit (LSB) first. Each sampled message bit is then XORed with the bit being
    /// simultaneously shifted out of the register at the most significant end, and the result is
    /// passed to the feedback taps.
    pub refin: bool,
    /// If equal to `false`, specifies that the contents of the register after reading the last
    /// message bit are unreflected before presentation; if equal to `true`, it specifies that they
    /// are reflected, character-by-character, before presentation. For the purpose of this
    /// definition, the reflection is performed by swapping the content of each cell with that of
    /// the cell an equal distance from the opposite end of the register; the characters of the CRC
    /// are then true images of parts of the reflected register, the character containing the
    /// original MSB always appearing first.
    pub refout: bool,
    /// The XOR value applied to the contents of the register after the last message bit has been
    /// read and after the optional reflection. It has the same endianness as the CRC such that its
    /// true image appears in the characters of the CRC.
    pub xorout: W,
    /// The contents of the register after initialising, reading the UTF-8 string `"123456789"` (as
    /// 8-bit characters), optionally reflecting, and applying the final XOR.
    pub check: W,
    /// The contents of the register after initialising, reading an error-free codeword and
    /// optionally reflecting the register (if [`refout`](Params::refout)=`true`), but not
    /// applying the final XOR. This is mathematically equivalent to initialising the register with
    /// the xorout parameter, reflecting it as described (if [`refout`](Params::refout)=`true`),
    /// reading as many zero bits as there are cells in the register, and reflecting the result (if
    /// [`refin`](Params::refin)=`true`). The residue of a crossed-endian model is calculated
    /// assuming that the characters of the received CRC are specially reflected before submitting
    /// the codeword.
    pub residue: W,
}

mod private {
    pub trait Sealed {}

    impl<W: super::Width> Sealed for W {}
}

/// Abstraction over CRC calculation method.
pub trait CalculateMethod: private::Sealed {
    type State;
}

/// Calculate using no lookup table
pub struct GenericNoLookupTable<W: Width>(core::marker::PhantomData<W>);
/// Calculate using a lookup table with 32 entries
pub struct GenericLookupTable32<W: Width>(core::marker::PhantomData<W>);
/// Calculate using a lookup table with 256 entries
pub struct GenericLookupTable256<W: Width>(core::marker::PhantomData<W>);
/// Calculate using a lookup table with 256xN entries
pub struct GenericLookupTable256xN<W: Width, const S: usize>(core::marker::PhantomData<W>);

impl<W: Width> private::Sealed for GenericNoLookupTable<W> {}
impl<W: Width> private::Sealed for GenericLookupTable32<W> {}
impl<W: Width> private::Sealed for GenericLookupTable256<W> {}
impl<W: Width, const S: usize> private::Sealed for GenericLookupTable256xN<W, S> {}

impl<W: Width> CalculateMethod for GenericNoLookupTable<W> {
    type State = ();
}

impl<W: Width> CalculateMethod for GenericLookupTable32<W> {
    type State = [W; 32];
}

impl<W: Width> CalculateMethod for GenericLookupTable256<W> {
    type State = [W; 256];
}

impl<W: Width> CalculateMethod for GenericLookupTable256xN<W, 4> {
    type State = [[W; 256]; 4];
}

impl<W: Width> CalculateMethod for GenericLookupTable256xN<W, 8> {
    type State = [[W; 256]; 8];
}

impl<W: Width> CalculateMethod for GenericLookupTable256xN<W, 16> {
    type State = [[W; 256]; 16];
}

impl<W: Width> CalculateMethod for GenericLookupTable256xN<W, 32> {
    type State = [[W; 256]; 32];
}
