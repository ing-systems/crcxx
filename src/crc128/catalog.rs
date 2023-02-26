use super::Params;

pub const CRC_82_DARC: Params<u128> = Params {
    width: 82,
    poly: 0x0000_308c_0111_0114_0144_0411,
    init: 0x0000_0000_0000_0000_0000_0000,
    refin: true,
    refout: true,
    xorout: 0x0000_0000_0000_0000_0000_0000,
    check: 0x0000_9ea8_3f62_5023_801f_d612,
    residue: 0x0000_0000_0000_0000_0000_0000,
};
