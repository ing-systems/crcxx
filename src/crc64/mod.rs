//! # CRC-64
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
//!     // singlepart data.
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
//!     // singlepart data.
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
//!     // singlepart data.
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
//! Ultimate method for processing big amounts of data on modern desktops and servers.
//! Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
//! The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.
//! ```
//! use crcxx::crc64::{*, catalog::CRC_64_XZ};
//!
//! const SLICES: usize = 16;
//! const CRC: Crc<LookupTable256xN<SLICES>> =
//!     Crc::<LookupTable256xN<SLICES>>::new(&CRC_64_XZ);
//!
//! fn main() {
//!     // singlepart data.
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
#[allow(clippy::wildcard_imports)]
use crate::internals::crc64::*;
use crate::{
    ComputeMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

pub mod catalog;

type Register = u64;

pub type NoLookupTable = GenericNoLookupTable<Register>;
pub type LookupTable32 = GenericLookupTable32<Register>;
pub type LookupTable256 = GenericLookupTable256<Register>;
pub type LookupTable256xN<const S: usize> = GenericLookupTable256xN<Register, S>;

pub struct Crc<'a, M: ComputeMethod> {
    pub params: &'a Params<Register>,
    lut: M::State,
}

#[derive(Clone)]
pub struct ComputeMultipart<'a, M: ComputeMethod> {
    crc: &'a Crc<'a, M>,
    value: Register,
}

imp_crc_initialize!(Register);
imp_crc_finalize!(Register);
imp_crc_no_lut!(Register);
imp_crc_lut_32!(Register);
imp_crc_lut_256!(Register);
imp_crc_lut_256x_n!(Register, 8);
imp_crc_lut_256x_n!(Register, 16);
imp_crc_lut_256x_n!(Register, 32);
