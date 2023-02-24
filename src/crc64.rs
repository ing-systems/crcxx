use crate::common::*;
use crate::{
    imp_make_lut_256, imp_make_lut_32, imp_make_sliced_lut, imp_reflect_byte, imp_reflect_value,
};

imp_make_lut_32!(crc64_make_lut_32, u64, reflect_byte_64, reflect_value_64);
imp_make_lut_256!(crc64_make_lut_256, u64, reflect_byte_64, reflect_value_64);
imp_make_sliced_lut!(crc64_make_sliced_lut, u64, crc64_make_lut_256);

imp_crc_update_lut_32!(crc64_update_lut_32, u64);
imp_crc_update_lut_256!(crc64_update_lut_256, u64);

imp_reflect_value!(reflect_value_64, u64);
imp_reflect_byte!(reflect_byte_64, u64);

pub fn crc64_update<const REFLECT: bool>(
    mut crc: u64, mut bytes: &[u8], lut: &[[u64; 256]],
) -> u64 {
    if SLICES >= 32 && lut.len() >= SLICE_32 && bytes.len() >= SLICE_32 {
        (crc, bytes) = update_slice_by_32::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 16 && lut.len() >= SLICE_16 && bytes.len() >= SLICE_16 {
        (crc, bytes) = update_slice_by_16::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 8 && lut.len() >= SLICE_8 && bytes.len() >= SLICE_8 {
        (crc, bytes) = update_slice_by_8::<REFLECT>(crc, bytes, lut);
    }

    crc64_update_lut_256::<REFLECT>(crc, bytes, &lut[0])
}

#[inline]
fn update_slice_by_8<'a, const REFLECT: bool>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]],
) -> (u64, &'a [u8]) {
    const STEP: usize = 8;

    assert!(lut.len() >= STEP);

    if REFLECT {
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
fn update_slice_by_16<'a, const REFLECT: bool>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]],
) -> (u64, &'a [u8]) {
    const STEP: usize = 16;

    assert!(lut.len() >= STEP);

    if REFLECT {
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
fn update_slice_by_32<'a, const REFLECT: bool>(
    mut crc: u64, mut bytes: &'a [u8], lut: &[[u64; 256]],
) -> (u64, &'a [u8]) {
    const STEP: usize = 32;

    assert!(lut.len() >= STEP);

    if REFLECT {
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

    #[test]
    fn with_reflect() {
        // CRC-64/ECMA
        const POLY: u64 = 0x42F0E1EBA9EA3693;
        const INIT: u64 = 0xFFFFFFFFFFFFFFFF;
        const REFLECT: bool = true;
        const XOR_OUT: u64 = 0xFFFFFFFFFFFFFFFF;

        const SAMPLES: [(&str, u64); 8] = [
            ("", 0x0000000000000000),
            ("0", 0x9901423B97329582),
            ("012", 0x413D0F0670A0E4D2),
            ("0123456", 0x8C880A3669E295FF),
            ("123456789", 0x995DC9BBDF1939FA),
            ("0123456789ABCDE", 0x346E97EBC42A4A0B),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0xD5A91BD8F8425127),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0xDCFCA5E374A1D0EE,
            ),
        ];

        let lut32 = crc64_make_lut_32(POLY, REFLECT);
        let lut256 = crc64_make_lut_256(POLY, REFLECT);
        let lut256x_n = crc64_make_sliced_lut(POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                crc64_update_lut_32::<REFLECT>(INIT, sample.0.as_bytes(), &lut32) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                crc64_update_lut_256::<REFLECT>(INIT, sample.0.as_bytes(), &lut256) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                crc64_update::<REFLECT>(INIT, sample.0.as_bytes(), &lut256x_n) ^ XOR_OUT,
                sample.1
            );
        }
    }
}
