//! # CRC-128
//!
//! ## Processing using no lookup table, single byte per step
//!
//!
//! ```
//! use crcxx::crc128::{*, catalog::CRC_82_DARC};
//!
//! const CRC: Crc<NoLookupTable> = Crc::<NoLookupTable>::new(&CRC_82_DARC);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//! }
//! ```
//! ## Processing using a lookup table with 32 entries, single byte per step
//!
//! ```
//! use crcxx::crc128::{*, catalog::CRC_82_DARC};
//!
//! const CRC: Crc<LookupTable32> = Crc::<LookupTable32>::new(&CRC_82_DARC);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//! }
//! ```
//! ## Processing using a lookup table with 256 entries, single byte per step
//!
//! ```
//! use crcxx::crc128::{*, catalog::CRC_82_DARC};
//!
//! const CRC: Crc<LookupTable256> = Crc::<LookupTable256>::new(&CRC_82_DARC);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//! }
//! ```
//! ## Processing using a lookup table with 256 x SLICES entries, multiple bytes per step
//!
//! NOTE: The minimum slice size is 16.
//!
//! ```
//! use crcxx::crc128::{*, catalog::CRC_82_DARC};
//!
//! const SLICES: usize = 16;
//! const CRC: Crc<LookupTable256xN<SLICES>> =
//!     Crc::<LookupTable256xN<SLICES>>::new(&CRC_82_DARC);
//!
//! fn main() {
//!     // Singlepart data.
//!     let crc = CRC.compute(b"123456789");
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//!
//!     // Multipart data.
//!     let mut multipart = CRC.compute_multipart();
//!     multipart.update(b"1234");
//!     multipart.update(b"5678");
//!     multipart.update(b"9");
//!
//!     let crc = multipart.value();
//!     assert_eq!(crc, 0x9EA8_3F62_5023_801F_D612);
//! }
//! ```
#[allow(clippy::wildcard_imports)]
use crate::internals::crc128::*;
use crate::{
    ComputeMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

pub mod catalog;

type Register = u128;

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
imp_crc_lut_256x_n!(Register, 16);
imp_crc_lut_256x_n!(Register, 32);

#[cfg(test)]
mod tests {
    use super::catalog::CRC_82_DARC;
    use super::*;

    const DATA: &[u8] = b"123456789";
    const PARAMS_SET: [Params<u128>; 1] = [CRC_82_DARC];

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
        }
    }
}
