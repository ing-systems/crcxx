//! CRC-128
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

    #[test]
    fn test() {
        let crc = Crc::<LookupTable256xN<16>>::new(&CRC_82_DARC);

        assert_eq!(crc.compute(b"123456789"), CRC_82_DARC.check);
    }
}
