use super::Params;

pub const CRC_40_GSM: Params<u64> = Params {
    width: 40,
    poly: 0x0000_0482_0009,
    init: 0x0000_0000_0000,
    refin: false,
    refout: false,
    xorout: 0x00ff_ffff_ffff,
    check: 0x00d4_164f_c646,
    residue: 0x00c4_ff80_71ff,
};
pub const CRC_64_ECMA_182: Params<u64> = Params {
    width: 64,
    poly: 0x42f0_e1eb_a9ea_3693,
    init: 0x0000_0000_0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000_0000_0000,
    check: 0x6c40_df5f_0b49_7347,
    residue: 0x0000_0000_0000_0000,
};
pub const CRC_64_GO_ISO: Params<u64> = Params {
    width: 64,
    poly: 0x0000_0000_0000_001b,
    init: 0xffff_ffff_ffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff_ffff_ffff,
    check: 0xb909_56c7_75a4_1001,
    residue: 0x5300_0000_0000_0000,
};
pub const CRC_64_MS: Params<u64> = Params {
    width: 64,
    poly: 0x259c_84cb_a642_6349,
    init: 0xffff_ffff_ffff_ffff,
    refin: true,
    refout: true,
    xorout: 0x0000_0000_0000_0000,
    check: 0x75d4_b74f_024e_ceea,
    residue: 0x0000_0000_0000_0000,
};
pub const CRC_64_REDIS: Params<u64> = Params {
    width: 64,
    poly: 0xad93_d235_94c9_35a9,
    init: 0x0000_0000_0000_0000,
    refin: true,
    refout: true,
    xorout: 0x0000_0000_0000_0000,
    check: 0xe9c6_d914_c4b8_d9ca,
    residue: 0x0000_0000_0000_0000,
};
pub const CRC_64_WE: Params<u64> = Params {
    width: 64,
    poly: 0x42f0_e1eb_a9ea_3693,
    init: 0xffff_ffff_ffff_ffff,
    refin: false,
    refout: false,
    xorout: 0xffff_ffff_ffff_ffff,
    check: 0x62ec_59e3_f1a4_f00a,
    residue: 0xfcac_bebd_5931_a992,
};
pub const CRC_64_XZ: Params<u64> = Params {
    width: 64,
    poly: 0x42f0_e1eb_a9ea_3693,
    init: 0xffff_ffff_ffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff_ffff_ffff,
    check: 0x995d_c9bb_df19_39fa,
    residue: 0x4995_8c9a_bd7d_353f,
};
