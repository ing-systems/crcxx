#[allow(clippy::wildcard_imports)]
use crate::internals::crc64::*;
use crate::{Crc, NoLookupTable, NormalLookupTable, Params, SlicedLookupTable, SmallLookupTable};

type State = u64;

imp_crc_initialize!(State);
imp_crc_finalize!(State);
imp_crc_no_lut!(State);
imp_crc_lut_32!(State);
imp_crc_lut_256!(State);
imp_crc_lut_256x_n!(State, 8);
imp_crc_lut_256x_n!(State, 16);
imp_crc_lut_256x_n!(State, 32);
