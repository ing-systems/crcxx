//! Low level CRC-32 functions.
use crate::imp_crc;

imp_crc!(u32);

#[inline]
fn update_slice_by_4<'a>(
    mut crc: u32, mut bytes: &'a [u8], lut: &[[u32; 256]], reflect: bool,
) -> (u32, &'a [u8]) {
    const STEP: usize = 4;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1][bytes[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x2][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x3][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x3][bytes[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x2][bytes[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1][bytes[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x0][bytes[0x3] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_8<'a>(
    mut crc: u32, mut bytes: &'a [u8], lut: &[[u32; 256]], reflect: bool,
) -> (u32, &'a [u8]) {
    const STEP: usize = 8;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x7] as usize]
                ^ lut[0x1][bytes[0x6] as usize]
                ^ lut[0x2][bytes[0x5] as usize]
                ^ lut[0x3][bytes[0x4] as usize]
                ^ lut[0x4][bytes[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x5][bytes[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x7][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x7][bytes[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x5][bytes[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x4][bytes[0x3] as usize ^ (crc & 0xFF) as usize]
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
    mut crc: u32, mut bytes: &'a [u8], lut: &[[u32; 256]], reflect: bool,
) -> (u32, &'a [u8]) {
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
                ^ lut[0xc][bytes[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xf][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0xf][bytes[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xc][bytes[0x3] as usize ^ (crc & 0xFF) as usize]
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
    mut crc: u32, mut bytes: &'a [u8], lut: &[[u32; 256]], reflect: bool,
) -> (u32, &'a [u8]) {
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
                ^ lut[0x1c][bytes[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1f][bytes[0x0] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x1f][bytes[0x00] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x02] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1c][bytes[0x03] as usize ^ (crc & 0xFF) as usize]
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
    use crate::internals::MAX_SLICES;

    const SLICES: usize = MAX_SLICES;

    #[test]
    fn with_reflect() {
        // CRC-32/JAMCRC
        const POLY: u32 = 0x04C1_1DB7;
        const INIT: u32 = 0xFFFF_FFFF;
        const REFLECT: bool = true;
        const XOR_OUT: u32 = 0x0000_0000;
        const WIDTH: u8 = 32;

        const SAMPLES: [(&str, u32); 8] = [
            ("", 0xFFFF_FFFF),
            ("0", 0x0B24_20DE),
            ("012", 0x2A5F_954F),
            ("0123456", 0x7240_F711),
            ("123456789", 0x340B_C6D9),
            ("0123456789ABCDE", 0x1E82_86DD),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x7065_C834),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0x9F25_C09C,
            ),
        ];

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_no_lut(INIT, WIDTH, POLY, REFLECT, sample.0.as_bytes()) ^ XOR_OUT,
                sample.1
            );
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
        // CRC-32/POSIX
        const POLY: u32 = 0x04C1_1DB7;
        const INIT: u32 = 0x0000_0000;
        const REFLECT: bool = false;
        const XOR_OUT: u32 = 0xFFFF_FFFF;
        const WIDTH: u8 = 32;

        const SAMPLES: [(&str, u32); 8] = [
            ("", 0xFFFF_FFFF),
            ("0", 0x2BCD_926F),
            ("012", 0xE63B_9D78),
            ("0123456", 0x3323_2AE1),
            ("123456789", 0x765E_7680),
            ("0123456789ABCDE", 0x13AD_FEB0),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x3F88_F4AD),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0x1A9A_735F,
            ),
        ];

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_no_lut(INIT, WIDTH, POLY, REFLECT, sample.0.as_bytes()) ^ XOR_OUT,
                sample.1
            );
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
