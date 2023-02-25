use crate::{imp_make_lut_256, imp_make_lut_256x_n, imp_make_lut_32};

imp_crc!(crc16, u16);
imp_make_lut_32!(make_lut_32, u16, crc16);
imp_make_lut_256!(make_lut_256, u16, crc16);
imp_make_lut_256x_n!(make_lut_256x_n, u16, make_lut_256);

imp_crc_update_lut_32!(update_lut_32, u16);
imp_crc_update_lut_256!(update_lut_256, u16);
imp_crc_update_slice_by!(update_lut_256x_n, u16);

#[inline]
fn update_slice_by_4<'a>(
    mut crc: u16, mut bytes: &'a [u8], lut: &[[u16; 256]], reflect: bool,
) -> (u16, &'a [u8]) {
    const STEP: usize = 4;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x3] as usize]
                ^ lut[0x1][bytes[0x2] as usize]
                ^ lut[0x2][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x3][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x3][bytes[0x0] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x2][bytes[0x1] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x1][bytes[0x2] as usize]
                ^ lut[0x0][bytes[0x3] as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_8<'a>(
    mut crc: u16, mut bytes: &'a [u8], lut: &[[u16; 256]], reflect: bool,
) -> (u16, &'a [u8]) {
    const STEP: usize = 8;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x7] as usize]
                ^ lut[0x1][bytes[0x6] as usize]
                ^ lut[0x2][bytes[0x5] as usize]
                ^ lut[0x3][bytes[0x4] as usize]
                ^ lut[0x4][bytes[0x3] as usize]
                ^ lut[0x5][bytes[0x2] as usize]
                ^ lut[0x6][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x7][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x7][bytes[0x0] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x1] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x5][bytes[0x2] as usize]
                ^ lut[0x4][bytes[0x3] as usize]
                ^ lut[0x3][bytes[0x4] as usize]
                ^ lut[0x2][bytes[0x5] as usize]
                ^ lut[0x1][bytes[0x6] as usize]
                ^ lut[0x0][bytes[0x7] as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_16<'a>(
    mut crc: u16, mut bytes: &'a [u8], lut: &[[u16; 256]], reflect: bool,
) -> (u16, &'a [u8]) {
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
                ^ lut[0x8][bytes[0x7] as usize]
                ^ lut[0x9][bytes[0x6] as usize]
                ^ lut[0xa][bytes[0x5] as usize]
                ^ lut[0xb][bytes[0x4] as usize]
                ^ lut[0xc][bytes[0x3] as usize]
                ^ lut[0xd][bytes[0x2] as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xf][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0xf][bytes[0x0] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize]
                ^ lut[0xc][bytes[0x3] as usize]
                ^ lut[0xb][bytes[0x4] as usize]
                ^ lut[0xa][bytes[0x5] as usize]
                ^ lut[0x9][bytes[0x6] as usize]
                ^ lut[0x8][bytes[0x7] as usize]
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
    mut crc: u16, mut bytes: &'a [u8], lut: &[[u16; 256]], reflect: bool,
) -> (u16, &'a [u8]) {
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
                ^ lut[0x18][bytes[0x07] as usize]
                ^ lut[0x19][bytes[0x06] as usize]
                ^ lut[0x1a][bytes[0x05] as usize]
                ^ lut[0x1b][bytes[0x04] as usize]
                ^ lut[0x1c][bytes[0x03] as usize]
                ^ lut[0x1d][bytes[0x02] as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1f][bytes[0x00] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x1f][bytes[0x00] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x02] as usize]
                ^ lut[0x1c][bytes[0x03] as usize]
                ^ lut[0x1b][bytes[0x04] as usize]
                ^ lut[0x1a][bytes[0x05] as usize]
                ^ lut[0x19][bytes[0x06] as usize]
                ^ lut[0x18][bytes[0x07] as usize]
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
        // CRC-16/KERMIT
        const POLY: u16 = 0x1021;
        const INIT: u16 = 0x0000;
        const REFLECT: bool = true;
        const XOR_OUT: u16 = 0x0000;
        const WIDTH: u8 = 16;

        const SAMPLES: [(&str, u16); 8] = [
            ("", 0x0000),
            ("0", 0x3183),
            ("012", 0x3B45),
            ("0123456", 0xC7B7),
            ("123456789", 0x2189),
            ("0123456789ABCDE", 0x9FFE),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x19D4),
            ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0xDC1A),
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
        // CRC-16/CDMA2000
        const POLY: u16 = 0xC867;
        const INIT: u16 = 0xFFFF;
        const REFLECT: bool = false;
        const XOR_OUT: u16 = 0x0000;
        const WIDTH: u8 = 16;

        const SAMPLES: [(&str, u16); 8] = [
            ("", 0xFFFF),
            ("0", 0x0528),
            ("012", 0x1E42),
            ("0123456", 0x998F),
            ("123456789", 0x4C06),
            ("0123456789ABCDE", 0x86F3),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0xCDB1),
            ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0x53EC),
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
