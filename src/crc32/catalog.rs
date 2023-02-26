use super::Params;

pub const CRC_17_CAN_FD: Params<u32> = Params {
    width: 17,
    poly: 0x1685b,
    init: 0x00000,
    refin: false,
    refout: false,
    xorout: 0x00000,
    check: 0x04f03,
    residue: 0x00000,
};
pub const CRC_21_CAN_FD: Params<u32> = Params {
    width: 21,
    poly: 0x0010_2899,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x000e_d841,
    residue: 0x0000_0000,
};
pub const CRC_24_BLE: Params<u32> = Params {
    width: 24,
    poly: 0x0000_065b,
    init: 0x0055_5555,
    refin: true,
    refout: true,
    xorout: 0x0000_0000,
    check: 0x00c2_5a56,
    residue: 0x0000_0000,
};
pub const CRC_24_FLEXRAY_A: Params<u32> = Params {
    width: 24,
    poly: 0x005d_6dcb,
    init: 0x00fe_dcba,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x0079_79bd,
    residue: 0x0000_0000,
};
pub const CRC_24_FLEXRAY_B: Params<u32> = Params {
    width: 24,
    poly: 0x005d_6dcb,
    init: 0x00ab_cdef,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x001f_23b8,
    residue: 0x0000_0000,
};
pub const CRC_24_INTERLAKEN: Params<u32> = Params {
    width: 24,
    poly: 0x0032_8b63,
    init: 0x00ff_ffff,
    refin: false,
    refout: false,
    xorout: 0x00ff_ffff,
    check: 0x00b4_f3e6,
    residue: 0x0014_4e63,
};
pub const CRC_24_LTE_A: Params<u32> = Params {
    width: 24,
    poly: 0x0086_4cfb,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x00cd_e703,
    residue: 0x0000_0000,
};
pub const CRC_24_LTE_B: Params<u32> = Params {
    width: 24,
    poly: 0x0080_0063,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x0023_ef52,
    residue: 0x0000_0000,
};
pub const CRC_24_OPENPGP: Params<u32> = Params {
    width: 24,
    poly: 0x0086_4cfb,
    init: 0x00b7_04ce,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x0021_cf02,
    residue: 0x0000_0000,
};
pub const CRC_24_OS_9: Params<u32> = Params {
    width: 24,
    poly: 0x0080_0063,
    init: 0x00ff_ffff,
    refin: false,
    refout: false,
    xorout: 0x00ff_ffff,
    check: 0x0020_0fa5,
    residue: 0x0080_0fe3,
};
pub const CRC_30_CDMA: Params<u32> = Params {
    width: 30,
    poly: 0x2030_b9c7,
    init: 0x3fff_ffff,
    refin: false,
    refout: false,
    xorout: 0x3fff_ffff,
    check: 0x04c3_4abf,
    residue: 0x34ef_a55a,
};
pub const CRC_31_PHILIPS: Params<u32> = Params {
    width: 31,
    poly: 0x04c1_1db7,
    init: 0x7fff_ffff,
    refin: false,
    refout: false,
    xorout: 0x7fff_ffff,
    check: 0x0ce9_e46c,
    residue: 0x4eaf_26f1,
};
pub const CRC_32_AIXM: Params<u32> = Params {
    width: 32,
    poly: 0x8141_41ab,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x3010_bf7f,
    residue: 0x0000_0000,
};
pub const CRC_32_AUTOSAR: Params<u32> = Params {
    width: 32,
    poly: 0xf4ac_fb13,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff,
    check: 0x1697_d06a,
    residue: 0x904c_ddbf,
};
pub const CRC_32_BASE91_D: Params<u32> = Params {
    width: 32,
    poly: 0xa833_982b,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff,
    check: 0x8731_5576,
    residue: 0x4527_0551,
};
pub const CRC_32_BZIP2: Params<u32> = Params {
    width: 32,
    poly: 0x04c1_1db7,
    init: 0xffff_ffff,
    refin: false,
    refout: false,
    xorout: 0xffff_ffff,
    check: 0xfc89_1918,
    residue: 0xc704_dd7b,
};
pub const CRC_32_CD_ROM_EDC: Params<u32> = Params {
    width: 32,
    poly: 0x8001_801b,
    init: 0x0000_0000,
    refin: true,
    refout: true,
    xorout: 0x0000_0000,
    check: 0x6ec2_edc4,
    residue: 0x0000_0000,
};
pub const CRC_32_CKSUM: Params<u32> = Params {
    width: 32,
    poly: 0x04c1_1db7,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0xffff_ffff,
    check: 0x765e_7680,
    residue: 0xc704_dd7b,
};
pub const CRC_32_ISCSI: Params<u32> = Params {
    width: 32,
    poly: 0x1edc_6f41,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff,
    check: 0xe306_9283,
    residue: 0xb798_b438,
};
pub const CRC_32_ISO_HDLC: Params<u32> = Params {
    width: 32,
    poly: 0x04c1_1db7,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0xffff_ffff,
    check: 0xcbf4_3926,
    residue: 0xdebb_20e3,
};
pub const CRC_32_JAMCRC: Params<u32> = Params {
    width: 32,
    poly: 0x04c1_1db7,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0x0000_0000,
    check: 0x340b_c6d9,
    residue: 0x0000_0000,
};
pub const CRC_32_MEF: Params<u32> = Params {
    width: 32,
    poly: 0x741b_8cd7,
    init: 0xffff_ffff,
    refin: true,
    refout: true,
    xorout: 0x0000_0000,
    check: 0xd2c2_2f51,
    residue: 0x0000_0000,
};
pub const CRC_32_MPEG_2: Params<u32> = Params {
    width: 32,
    poly: 0x04c1_1db7,
    init: 0xffff_ffff,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0x0376_e6e7,
    residue: 0x0000_0000,
};
pub const CRC_32_XFER: Params<u32> = Params {
    width: 32,
    poly: 0x0000_00af,
    init: 0x0000_0000,
    refin: false,
    refout: false,
    xorout: 0x0000_0000,
    check: 0xbd0b_e338,
    residue: 0x0000_0000,
};
