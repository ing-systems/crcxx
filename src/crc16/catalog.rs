use super::Params;

pub const CRC_10_ATM: Params<u16> = Params {
    width: 10,
    poly: 0x233,
    init: 0x000,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0x199,
    residue: 0x000,
};
pub const CRC_10_CDMA2000: Params<u16> = Params {
    width: 10,
    poly: 0x3d9,
    init: 0x3ff,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0x233,
    residue: 0x000,
};
pub const CRC_10_GSM: Params<u16> = Params {
    width: 10,
    poly: 0x175,
    init: 0x000,
    refin: false,
    refout: false,
    xorout: 0x3ff,
    check: 0x12a,
    residue: 0x0c6,
};
pub const CRC_11_FLEXRAY: Params<u16> = Params {
    width: 11,
    poly: 0x385,
    init: 0x01a,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0x5a3,
    residue: 0x000,
};
pub const CRC_11_UMTS: Params<u16> = Params {
    width: 11,
    poly: 0x307,
    init: 0x000,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0x061,
    residue: 0x000,
};
pub const CRC_12_CDMA2000: Params<u16> = Params {
    width: 12,
    poly: 0xf13,
    init: 0xfff,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0xd4d,
    residue: 0x000,
};
pub const CRC_12_DECT: Params<u16> = Params {
    width: 12,
    poly: 0x80f,
    init: 0x000,
    refin: false,
    refout: false,
    xorout: 0x000,
    check: 0xf5b,
    residue: 0x000,
};
pub const CRC_12_GSM: Params<u16> = Params {
    width: 12,
    poly: 0xd31,
    init: 0x000,
    refin: false,
    refout: false,
    xorout: 0xfff,
    check: 0xb34,
    residue: 0x178,
};
pub const CRC_12_UMTS: Params<u16> = Params {
    width: 12,
    poly: 0x80f,
    init: 0x000,
    refin: false,
    refout: true,
    xorout: 0x000,
    check: 0xdaf,
    residue: 0x000,
};
pub const CRC_13_BBC: Params<u16> = Params {
    width: 13,
    poly: 0x1cf5,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x04fa,
    residue: 0x0000,
};
pub const CRC_14_DARC: Params<u16> = Params {
    width: 14,
    poly: 0x0805,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x082d,
    residue: 0x0000,
};
pub const CRC_14_GSM: Params<u16> = Params {
    width: 14,
    poly: 0x202d,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x3fff,
    check: 0x30ae,
    residue: 0x031e,
};
pub const CRC_15_CAN: Params<u16> = Params {
    width: 15,
    poly: 0x4599,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x059e,
    residue: 0x0000,
};
pub const CRC_15_MPT1327: Params<u16> = Params {
    width: 15,
    poly: 0x6815,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0001,
    check: 0x2566,
    residue: 0x6815,
};
pub const CRC_16_ARC: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0xbb3d,
    residue: 0x0000,
};
pub const CRC_16_CDMA2000: Params<u16> = Params {
    width: 16,
    poly: 0xc867,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x4c06,
    residue: 0x0000,
};
pub const CRC_16_CMS: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000,
};
pub const CRC_16_DDS_110: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0x800d,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x9ecf,
    residue: 0x0000,
};
pub const CRC_16_DECT_R: Params<u16> = Params {
    width: 16,
    poly: 0x0589,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0001,
    check: 0x007e,
    residue: 0x0589,
};
pub const CRC_16_DECT_X: Params<u16> = Params {
    width: 16,
    poly: 0x0589,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x007f,
    residue: 0x0000,
};
pub const CRC_16_DNP: Params<u16> = Params {
    width: 16,
    poly: 0x3d65,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0xea82,
    residue: 0x66c5,
};
pub const CRC_16_EN_13757: Params<u16> = Params {
    width: 16,
    poly: 0x3d65,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xc2b7,
    residue: 0xa366,
};
pub const CRC_16_GENIBUS: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xd64e,
    residue: 0x1d0f,
};
pub const CRC_16_GSM: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xce3c,
    residue: 0x1d0f,
};
pub const CRC_16_IBM_3740: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x29b1,
    residue: 0x0000,
};
pub const CRC_16_IBM_SDLC: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0x906e,
    residue: 0xf0b8,
};
pub const CRC_16_ISO_IEC_14443_3_A: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xc6c6,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0xbf05,
    residue: 0x0000,
};
pub const CRC_16_KERMIT: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x2189,
    residue: 0x0000,
};
pub const CRC_16_LJ1200: Params<u16> = Params {
    width: 16,
    poly: 0x6f63,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xbdf4,
    residue: 0x0000,
};
pub const CRC_16_M17: Params<u16> = Params {
    width: 16,
    poly: 0x5935,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x772b,
    residue: 0x0000,
};
pub const CRC_16_MAXIM_DOW: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0x0000,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0x44c2,
    residue: 0xb001,
};
pub const CRC_16_MCRF4XX: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x6f91,
    residue: 0x0000,
};
pub const CRC_16_MODBUS: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x4b37,
    residue: 0x0000,
};
pub const CRC_16_NRSC_5: Params<u16> = Params {
    width: 16,
    poly: 0x080b,
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0xa066,
    residue: 0x0000,
};
pub const CRC_16_OPENSAFETY_A: Params<u16> = Params {
    width: 16,
    poly: 0x5935,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x5d38,
    residue: 0x0000,
};
pub const CRC_16_OPENSAFETY_B: Params<u16> = Params {
    width: 16,
    poly: 0x755b,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x20fe,
    residue: 0x0000,
};
pub const CRC_16_PROFIBUS: Params<u16> = Params {
    width: 16,
    poly: 0x1dcf,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xa819,
    residue: 0xe394,
};
pub const CRC_16_RIELLO: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0xb2aa,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x63d0,
    residue: 0x0000,
};
pub const CRC_16_SPI_FUJITSU: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0x1d0f,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xe5cc,
    residue: 0x0000,
};
pub const CRC_16_T10_DIF: Params<u16> = Params {
    width: 16,
    poly: 0x8bb7,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xd0db,
    residue: 0x0000,
};
pub const CRC_16_TELEDISK: Params<u16> = Params {
    width: 16,
    poly: 0xa097,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x0fb3,
    residue: 0x0000,
};
pub const CRC_16_TMS37157: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0x89ec,
    refin: true,
    refout: true,
    xorout: 0x0000,
    check: 0x26b1,
    residue: 0x0000,
};
pub const CRC_16_UMTS: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xfee8,
    residue: 0x0000,
};
pub const CRC_16_USB: Params<u16> = Params {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0xb4c8,
    residue: 0xb001,
};
pub const CRC_16_XMODEM: Params<u16> = Params {
    width: 16,
    poly: 0x1021,
    init: 0x0000,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x31c3,
    residue: 0x0000,
};
