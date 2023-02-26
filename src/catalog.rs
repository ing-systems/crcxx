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
