use super::Params;

pub const CRC_3_GSM: Params<u8> = Params {
    width: 3,
    poly: 0x3,
    init: 0x0,
    refin: false,
    refout: false,
    xorout: 0x7,
    check: 0x4,
    residue: 0x2,
};
pub const CRC_3_ROHC: Params<u8> = Params {
    width: 3,
    poly: 0x3,
    init: 0x7,
    refin: true,
    refout: true,
    xorout: 0x0,
    check: 0x6,
    residue: 0x0,
};
pub const CRC_4_G_704: Params<u8> = Params {
    width: 4,
    poly: 0x3,
    init: 0x0,
    refin: true,
    refout: true,
    xorout: 0x0,
    check: 0x7,
    residue: 0x0,
};
pub const CRC_4_INTERLAKEN: Params<u8> = Params {
    width: 4,
    poly: 0x3,
    init: 0xf,
    refin: false,
    refout: false,
    xorout: 0xf,
    check: 0xb,
    residue: 0x2,
};
pub const CRC_5_EPC_C1G2: Params<u8> = Params {
    width: 5,
    poly: 0x09,
    init: 0x09,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x00,
    residue: 0x00,
};
pub const CRC_5_G_704: Params<u8> = Params {
    width: 5,
    poly: 0x15,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x07,
    residue: 0x00,
};
pub const CRC_5_USB: Params<u8> = Params {
    width: 5,
    poly: 0x05,
    init: 0x1f,
    refin: true,
    refout: true,
    xorout: 0x1f,
    check: 0x19,
    residue: 0x06,
};
pub const CRC_6_CDMA2000_A: Params<u8> = Params {
    width: 6,
    poly: 0x27,
    init: 0x3f,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x0d,
    residue: 0x00,
};
pub const CRC_6_CDMA2000_B: Params<u8> = Params {
    width: 6,
    poly: 0x07,
    init: 0x3f,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x3b,
    residue: 0x00,
};
pub const CRC_6_DARC: Params<u8> = Params {
    width: 6,
    poly: 0x19,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x26,
    residue: 0x00,
};
pub const CRC_6_G_704: Params<u8> = Params {
    width: 6,
    poly: 0x03,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x06,
    residue: 0x00,
};
pub const CRC_6_GSM: Params<u8> = Params {
    width: 6,
    poly: 0x2f,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x3f,
    check: 0x13,
    residue: 0x3a,
};
pub const CRC_7_MMC: Params<u8> = Params {
    width: 7,
    poly: 0x09,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x75,
    residue: 0x00,
};
pub const CRC_7_ROHC: Params<u8> = Params {
    width: 7,
    poly: 0x4f,
    init: 0x7f,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x53,
    residue: 0x00,
};
pub const CRC_7_UMTS: Params<u8> = Params {
    width: 7,
    poly: 0x45,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x61,
    residue: 0x00,
};
pub const CRC_8_AUTOSAR: Params<u8> = Params {
    width: 8,
    poly: 0x2f,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0xdf,
    residue: 0x42,
};
pub const CRC_8_BLUETOOTH: Params<u8> = Params {
    width: 8,
    poly: 0xa7,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x26,
    residue: 0x00,
};
pub const CRC_8_CDMA2000: Params<u8> = Params {
    width: 8,
    poly: 0x9b,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xda,
    residue: 0x00,
};
pub const CRC_8_DARC: Params<u8> = Params {
    width: 8,
    poly: 0x39,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x15,
    residue: 0x00,
};
pub const CRC_8_DVB_S2: Params<u8> = Params {
    width: 8,
    poly: 0xd5,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xbc,
    residue: 0x00,
};
pub const CRC_8_GSM_A: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x37,
    residue: 0x00,
};
pub const CRC_8_GSM_B: Params<u8> = Params {
    width: 8,
    poly: 0x49,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0x94,
    residue: 0x53,
};
pub const CRC_8_HITAG: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xb4,
    residue: 0x00,
};
pub const CRC_8_I_432_1: Params<u8> = Params {
    width: 8,
    poly: 0x07,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x55,
    check: 0xa1,
    residue: 0xac,
};
pub const CRC_8_I_CODE: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0xfd,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x7e,
    residue: 0x00,
};
pub const CRC_8_LTE: Params<u8> = Params {
    width: 8,
    poly: 0x9b,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xea,
    residue: 0x00,
};
pub const CRC_8_MAXIM_DOW: Params<u8> = Params {
    width: 8,
    poly: 0x31,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0xa1,
    residue: 0x00,
};
pub const CRC_8_MIFARE_MAD: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0xc7,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x99,
    residue: 0x00,
};
pub const CRC_8_NRSC_5: Params<u8> = Params {
    width: 8,
    poly: 0x31,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xf7,
    residue: 0x00,
};
pub const CRC_8_OPENSAFETY: Params<u8> = Params {
    width: 8,
    poly: 0x2f,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0x3e,
    residue: 0x00,
};
pub const CRC_8_ROHC: Params<u8> = Params {
    width: 8,
    poly: 0x07,
    init: 0xff,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0xd0,
    residue: 0x00,
};
pub const CRC_8_SAE_J1850: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: false,
    refout: false,
    xorout: 0xff,
    check: 0x4b,
    residue: 0xc4,
};
pub const CRC_8_SMBUS: Params<u8> = Params {
    width: 8,
    poly: 0x07,
    init: 0x00,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xf4,
    residue: 0x00,
};
pub const CRC_8_TECH_3250: Params<u8> = Params {
    width: 8,
    poly: 0x1d,
    init: 0xff,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x97,
    residue: 0x00,
};
pub const CRC_8_WCDMA: Params<u8> = Params {
    width: 8,
    poly: 0x9b,
    init: 0x00,
    refin: true,
    refout: true,
    xorout: 0x00,
    check: 0x25,
    residue: 0x00,
};
