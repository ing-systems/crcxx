use crate::{imp_make_lut_256, imp_make_lut_256x_n, imp_make_lut_32};

imp_crc!(crc64, u64);
imp_make_lut_32!(make_lut_32, u64, crc64);
imp_make_lut_256!(make_lut_256, u64, crc64);
imp_make_lut_256x_n!(make_lut_256x_n, u64, make_lut_256);

imp_crc_update_lut_32!(update_lut_32, u64);
imp_crc_update_lut_256!(update_lut_256, u64);
imp_crc_update_slice_by!(update_lut_256x_n, u64);

#[inline]
#[allow(clippy::missing_const_for_fn)]
fn update_slice_by_4<'a>(
    crc: u64, bytes: &'a [u8], _lut: &[[u64; 256]], _reflect: bool,
) -> (u64, &'a [u8]) {
    (crc, bytes)
}

#[inline]
fn update_slice_by_8<'a>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]], reflect: bool,
) -> (u64, &'a [u8]) {
    const STEP: usize = 8;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x07] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x1][bytes[0x06] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x2][bytes[0x05] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x3][bytes[0x04] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x4][bytes[0x03] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x5][bytes[0x02] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x01] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x7][bytes[0x00] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x7][bytes[0x0] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x1] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x5][bytes[0x2] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x4][bytes[0x3] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x3][bytes[0x4] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x2][bytes[0x5] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1][bytes[0x6] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x0][bytes[0x7] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_16<'a>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]], reflect: bool,
) -> (u64, &'a [u8]) {
    const STEP: usize = 16;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0xf] as usize]
                ^ lut[0x1][bytes[0xe] as usize]
                ^ lut[0x2][bytes[0xd] as usize]
                ^ lut[0x3][bytes[0xc] as usize]
                ^ lut[0x4][bytes[0xb] as usize]
                ^ lut[0x5][bytes[0xa] as usize]
                ^ lut[0x6][bytes[0x9] as usize]
                ^ lut[0x7][bytes[0x8] as usize]
                ^ lut[0x8][bytes[0x7] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x9][bytes[0x6] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0xa][bytes[0x5] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0xb][bytes[0x4] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0xc][bytes[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xf][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0xf][bytes[0x0] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0xc][bytes[0x3] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0xb][bytes[0x4] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xa][bytes[0x5] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x9][bytes[0x6] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x8][bytes[0x7] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x7][bytes[0x8] as usize]
                ^ lut[0x6][bytes[0x9] as usize]
                ^ lut[0x5][bytes[0xa] as usize]
                ^ lut[0x4][bytes[0xb] as usize]
                ^ lut[0x3][bytes[0xc] as usize]
                ^ lut[0x2][bytes[0xd] as usize]
                ^ lut[0x1][bytes[0xe] as usize]
                ^ lut[0x0][bytes[0xf] as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_32<'a>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]], reflect: bool,
) -> (u64, &'a [u8]) {
    const STEP: usize = 32;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x00][bytes[0x1f] as usize]
                ^ lut[0x01][bytes[0x1e] as usize]
                ^ lut[0x02][bytes[0x1d] as usize]
                ^ lut[0x03][bytes[0x1c] as usize]
                ^ lut[0x04][bytes[0x1b] as usize]
                ^ lut[0x05][bytes[0x1a] as usize]
                ^ lut[0x06][bytes[0x19] as usize]
                ^ lut[0x07][bytes[0x18] as usize]
                ^ lut[0x08][bytes[0x17] as usize]
                ^ lut[0x09][bytes[0x16] as usize]
                ^ lut[0x0a][bytes[0x15] as usize]
                ^ lut[0x0b][bytes[0x14] as usize]
                ^ lut[0x0c][bytes[0x13] as usize]
                ^ lut[0x0d][bytes[0x12] as usize]
                ^ lut[0x0e][bytes[0x11] as usize]
                ^ lut[0x0f][bytes[0x10] as usize]
                ^ lut[0x10][bytes[0x0f] as usize]
                ^ lut[0x11][bytes[0x0e] as usize]
                ^ lut[0x12][bytes[0x0d] as usize]
                ^ lut[0x13][bytes[0x0c] as usize]
                ^ lut[0x14][bytes[0x0b] as usize]
                ^ lut[0x15][bytes[0x0a] as usize]
                ^ lut[0x16][bytes[0x09] as usize]
                ^ lut[0x17][bytes[0x08] as usize]
                ^ lut[0x18][bytes[0x07] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x19][bytes[0x06] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x1a][bytes[0x05] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x1b][bytes[0x04] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x1c][bytes[0x03] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x02] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1f][bytes[0x00] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x1f][bytes[0x00] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x02] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x1c][bytes[0x03] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x1b][bytes[0x04] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1a][bytes[0x05] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x19][bytes[0x06] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x18][bytes[0x07] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x17][bytes[0x08] as usize]
                ^ lut[0x16][bytes[0x09] as usize]
                ^ lut[0x15][bytes[0x0a] as usize]
                ^ lut[0x14][bytes[0x0b] as usize]
                ^ lut[0x13][bytes[0x0c] as usize]
                ^ lut[0x12][bytes[0x0d] as usize]
                ^ lut[0x11][bytes[0x0e] as usize]
                ^ lut[0x10][bytes[0x0f] as usize]
                ^ lut[0x0f][bytes[0x10] as usize]
                ^ lut[0x0e][bytes[0x11] as usize]
                ^ lut[0x0d][bytes[0x12] as usize]
                ^ lut[0x0c][bytes[0x13] as usize]
                ^ lut[0x0b][bytes[0x14] as usize]
                ^ lut[0x0a][bytes[0x15] as usize]
                ^ lut[0x09][bytes[0x16] as usize]
                ^ lut[0x08][bytes[0x17] as usize]
                ^ lut[0x07][bytes[0x18] as usize]
                ^ lut[0x06][bytes[0x19] as usize]
                ^ lut[0x05][bytes[0x1a] as usize]
                ^ lut[0x04][bytes[0x1b] as usize]
                ^ lut[0x03][bytes[0x1c] as usize]
                ^ lut[0x02][bytes[0x1d] as usize]
                ^ lut[0x01][bytes[0x1e] as usize]
                ^ lut[0x00][bytes[0x1f] as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLICES: usize = crate::MAX_SLICES;

    #[test]
    fn with_reflect() {
        // CRC-64/XZ
        const POLY: u64 = 0x42F0_E1EB_A9EA_3693;
        const INIT: u64 = 0xFFFF_FFFF_FFFF_FFFF;
        const REFLECT: bool = true;
        const XOR_OUT: u64 = 0xFFFF_FFFF_FFFF_FFFF;
        const WIDTH: u8 = 64;

        const SAMPLES: [(&str, u64); 8] = [
            ("", 0x0000_0000_0000_0000),
            ("0", 0x9901_423B_9732_9582),
            ("012", 0x413D_0F06_70A0_E4D2),
            ("0123456", 0x8C88_0A36_69E2_95FF),
            ("123456789", 0x995D_C9BB_DF19_39FA),
            ("0123456789ABCDE", 0x346E_97EB_C42A_4A0B),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0xD5A9_1BD8_F842_5127),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0xDCFC_A5E3_74A1_D0EE,
            ),
        ];

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_lut_32(INIT, sample.0.as_bytes(), &lut32, REFLECT) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256(INIT, sample.0.as_bytes(), &lut256, REFLECT) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256x_n::<SLICES>(INIT, sample.0.as_bytes(), &lut256x_n, REFLECT)
                    ^ XOR_OUT,
                sample.1
            );
        }
    }

    #[test]
    fn without_reflect() {
        // CRC-64/WE
        const POLY: u64 = 0x42F0_E1EB_A9EA_3693;
        const INIT: u64 = 0xFFFF_FFFF_FFFF_FFFF;
        const REFLECT: bool = false;
        const XOR_OUT: u64 = 0xFFFF_FFFF_FFFF_FFFF;
        const WIDTH: u8 = 64;

        const SAMPLES: [(&str, u64); 8] = [
            ("", 0x0000_0000_0000_0000),
            ("0", 0x30BB_6F26_7FA7_3BC9),
            ("012", 0x0715_CFE7_49A8_4CAB),
            ("0123456", 0x31C7_86D3_5D62_56F8),
            ("123456789", 0x62EC_59E3_F1A4_F00A),
            ("0123456789ABCDE", 0xB8E6_113E_7DD1_4D64),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0xDE67_FFFA_1EB9_7FED),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0x2FF6_ED4D_2950_A870,
            ),
        ];

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_lut_32(INIT, sample.0.as_bytes(), &lut32, REFLECT) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256(INIT, sample.0.as_bytes(), &lut256, REFLECT) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256x_n::<SLICES>(INIT, sample.0.as_bytes(), &lut256x_n, REFLECT)
                    ^ XOR_OUT,
                sample.1
            );
        }
    }
}
