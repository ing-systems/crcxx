//! CRC-32
#[allow(clippy::wildcard_imports)]
use crate::internals::crc32::*;
use crate::{
    CalculateMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

pub mod catalog;

type Register = u32;

pub type NoLookupTable = GenericNoLookupTable<Register>;
pub type LookupTable32 = GenericLookupTable32<Register>;
pub type LookupTable256 = GenericLookupTable256<Register>;
pub type LookupTable256xN<const S: usize> = GenericLookupTable256xN<Register, S>;

pub struct Crc<'a, M: CalculateMethod> {
    pub params: &'a Params<Register>,
    lut: M::State,
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
