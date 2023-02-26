//! The crate computes CRC-8/16/32/64/128 using various methods. Included catalog of CRC parameters simplify usage.
//! It is applicable from small embedded systems to modern desktops and servers. No unsafe or architecture specific code.
//!
//! # Usage
//!
//! ## Processing using no lookup table, single byte per step
//!
//! The slowest method. No additional memory required.
//!
//! ```
//! use crcxx::crc64::{*, catalog::CRC_64_XZ};
//!
//! const CRC: Crc<NoLookupTable> = Crc::<NoLookupTable>::new(&CRC_64_XZ);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//! }
//! ```
//! ## Processing using a lookup table with 32 entries, single byte per step
//!
//! Good compromise between speed and memory consumption for small embedded devices.
//! Depending on usage scenario usually 2-5 times faster than the previous method.
//! ```
//! use crcxx::crc64::{*, catalog::CRC_64_XZ};
//!
//! const CRC: Crc<LookupTable32> = Crc::<LookupTable32>::new(&CRC_64_XZ);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//! }
//! ```
//! ## Processing using a lookup table with 256 entries, single byte per step
//!
//! Depending on usage scenario usually no more than 2 times faster than the previous method.
//! ```
//! use crcxx::crc64::{*, catalog::CRC_64_XZ};
//!
//! const CRC: Crc<LookupTable256> = Crc::<LookupTable256>::new(&CRC_64_XZ);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//! }
//! ```
//! ## Processing using a lookup table with 256 x SLICES entries, multiple bytes per step
//!
//! Ultimate method for processing big amounts of data on modern desktops and servers without using architecture
//! specific instructions. Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
//! The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.
//! ```
//! use crcxx::crc64::{*, catalog::CRC_64_XZ};
//!
//! const SLICES: usize = 16;
//! const CRC: Crc<LookupTable256xN<SLICES>> =
//!     Crc::<LookupTable256xN<SLICES>>::new(&CRC_64_XZ);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x995D_C9BB_DF19_39FA);
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

pub trait Register: Sized {}
impl Register for u8 {}
impl Register for u16 {}
impl Register for u32 {}
impl Register for u64 {}
impl Register for u128 {}

// !!! Copy-paste from crc-catalog crate. !!!
/// CRC computation parameters
///
/// The struct describes a CRC algorithm using the fields specified by the [Catalogue of
/// parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
#[derive(Debug)]
pub struct Params<R: Register> {
    /// The number of bit cells in the linear feedback shift register; the degree of the generator
    /// polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the shift register. The
    /// least significant bit corresponds to the inward end of the shift register, and is always
    /// set. The highest-order term is omitted.
    pub poly: R,
    /// The settings of the bit cells at the start of each calculation, before reading the first
    /// message bit. The least significant bit corresponds to the inward end of the shift register.
    pub init: R,
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
    pub xorout: R,
    /// The contents of the register after initialising, reading the UTF-8 string `"123456789"` (as
    /// 8-bit characters), optionally reflecting, and applying the final XOR.
    pub check: R,
    /// The contents of the register after initialising, reading an error-free codeword and
    /// optionally reflecting the register (if [`refout`](Params::refout)=`true`), but not
    /// applying the final XOR. This is mathematically equivalent to initialising the register with
    /// the xorout parameter, reflecting it as described (if [`refout`](Params::refout)=`true`),
    /// reading as many zero bits as there are cells in the register, and reflecting the result (if
    /// [`refin`](Params::refin)=`true`). The residue of a crossed-endian model is calculated
    /// assuming that the characters of the received CRC are specially reflected before submitting
    /// the codeword.
    pub residue: R,
}

mod private {
    pub trait Sealed {}

    impl<R: super::Register> Sealed for R {}
}

/// Abstraction over CRC computation method.
pub trait ComputeMethod: private::Sealed {
    type State;
}

/// Calculate using no lookup table
pub struct GenericNoLookupTable<R: Register>(core::marker::PhantomData<R>);
/// Calculate using a lookup table with 32 entries
pub struct GenericLookupTable32<R: Register>(core::marker::PhantomData<R>);
/// Calculate using a lookup table with 256 entries
pub struct GenericLookupTable256<R: Register>(core::marker::PhantomData<R>);
/// Calculate using a lookup table with 256xN entries
pub struct GenericLookupTable256xN<R: Register, const S: usize>(core::marker::PhantomData<R>);

impl<W: Register> private::Sealed for GenericNoLookupTable<W> {}
impl<W: Register> private::Sealed for GenericLookupTable32<W> {}
impl<W: Register> private::Sealed for GenericLookupTable256<W> {}
impl<W: Register, const S: usize> private::Sealed for GenericLookupTable256xN<W, S> {}

impl<R: Register> ComputeMethod for GenericNoLookupTable<R> {
    type State = ();
}

impl<R: Register> ComputeMethod for GenericLookupTable32<R> {
    type State = [R; 32];
}

impl<R: Register> ComputeMethod for GenericLookupTable256<R> {
    type State = [R; 256];
}

impl<R: Register> ComputeMethod for GenericLookupTable256xN<R, 4> {
    type State = [[R; 256]; 4];
}

impl<R: Register> ComputeMethod for GenericLookupTable256xN<R, 8> {
    type State = [[R; 256]; 8];
}

impl<R: Register> ComputeMethod for GenericLookupTable256xN<R, 16> {
    type State = [[R; 256]; 16];
}

impl<R: Register> ComputeMethod for GenericLookupTable256xN<R, 32> {
    type State = [[R; 256]; 32];
}
