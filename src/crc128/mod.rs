#[allow(clippy::wildcard_imports)]
use crate::internals::crc128::*;
use crate::{
    CalculateMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

pub mod catalog;

type State = u128;

type NoLookupTable = GenericNoLookupTable<State>;
type LookupTable32 = GenericLookupTable32<State>;
type LookupTable256 = GenericLookupTable256<State>;
type LookupTable256xN<const S: usize> = GenericLookupTable256xN<State, S>;

pub struct Calculator<'a, M: CalculateMethod> {
    pub params: &'a Params<State>,
    lut: M::State,
}

imp_crc_initialize!(State);
imp_crc_finalize!(State);
imp_crc_no_lut!(State);
imp_crc_lut_32!(State);
imp_crc_lut_256!(State);
imp_crc_lut_256x_n!(State, 16);
imp_crc_lut_256x_n!(State, 32);

#[cfg(test)]
mod tests {
    use super::catalog::CRC_82_DARC;
    use super::*;

    #[test]
    fn test() {
        let crc = Calculator::<LookupTable256xN<16>>::new(&CRC_82_DARC);

        assert_eq!(crc.calculate(b"123456789"), CRC_82_DARC.check);
    }
}
