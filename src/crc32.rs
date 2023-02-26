#[allow(clippy::wildcard_imports)]
use crate::internals::crc32::*;
use crate::{
    CalculateMethod, GenericLookupTable256, GenericLookupTable256xN, GenericLookupTable32,
    GenericNoLookupTable, Params,
};

type State = u32;

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
imp_crc_lut_256x_n!(State, 4);
imp_crc_lut_256x_n!(State, 8);
imp_crc_lut_256x_n!(State, 16);
imp_crc_lut_256x_n!(State, 32);
