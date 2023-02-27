//! # CRC-16
//!
//! ## Processing using no lookup table, single byte per step
//!
//!
//! ```
//! use crcxx::crc16::{*, catalog::CRC_16_KERMIT};
//!
//! const CRC: Crc<NoLookupTable> = Crc::<NoLookupTable>::new(&CRC_16_KERMIT);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x2189);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x2189);
//! }
//! ```
//! ## Processing using a lookup table with 32 entries, single byte per step
//!
//! ```
//! use crcxx::crc16::{*, catalog::CRC_16_KERMIT};
//!
//! const CRC: Crc<LookupTable32> = Crc::<LookupTable32>::new(&CRC_16_KERMIT);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x2189);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x2189);
//! }
//! ```
//! ## Processing using a lookup table with 256 entries, single byte per step
//!
//! ```
//! use crcxx::crc16::{*, catalog::CRC_16_KERMIT};
//!
//! const CRC: Crc<LookupTable256> = Crc::<LookupTable256>::new(&CRC_16_KERMIT);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x2189);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x2189);
//! }
//! ```
//! ## Processing using a lookup table with 256 x SLICES entries, multiple bytes per step
//!
//! ```
//! use crcxx::crc16::{*, catalog::CRC_16_KERMIT};
//!
//! const SLICES: usize = 16;
//! const CRC: Crc<LookupTable256xN<SLICES>> =
//!     Crc::<LookupTable256xN<SLICES>>::new(&CRC_16_KERMIT);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x2189);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x2189);
//! }
//! ```
#[allow(clippy::wildcard_imports)]
use crate::internals::crc16::*;
use crate::{
    ComputeMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

pub mod catalog;

type Register = u16;

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
imp_crc_lut_256x_n!(Register, 4);
imp_crc_lut_256x_n!(Register, 8);
imp_crc_lut_256x_n!(Register, 16);
imp_crc_lut_256x_n!(Register, 32);

#[cfg(test)]
mod tests {
    use super::catalog::{CRC_10_ATM, CRC_12_UMTS, CRC_14_DARC, CRC_16_ARC, CRC_16_CDMA2000};
    use super::*;

    const DATA: &[u8] = b"123456789";
    const PARAMS_SET: [Params<u16>; 5] =
        [CRC_10_ATM, CRC_12_UMTS, CRC_14_DARC, CRC_16_ARC, CRC_16_CDMA2000];

    #[test]
    fn test() {
        for ref params in PARAMS_SET {
            assert_eq!(Crc::<NoLookupTable>::new(params).compute(DATA), params.check);
            assert_eq!(
                Crc::<NoLookupTable>::new(params).compute_multipart().update(DATA).value(),
                params.check
            );

            assert_eq!(Crc::<LookupTable32>::new(params).compute(DATA), params.check);
            assert_eq!(
                Crc::<LookupTable32>::new(params).compute_multipart().update(DATA).value(),
                params.check
            );

            assert_eq!(Crc::<LookupTable256>::new(params).compute(DATA), params.check);
            assert_eq!(
                Crc::<LookupTable256>::new(params).compute_multipart().update(DATA).value(),
                params.check
            );

            assert_eq!(Crc::<LookupTable256xN<32>>::new(params).compute(DATA), params.check);
            assert_eq!(
                Crc::<LookupTable256xN<32>>::new(params).compute_multipart().update(DATA).value(),
                params.check
            );
        }
    }
}
