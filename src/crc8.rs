use crate::{
    imp_make_lut_256, imp_make_lut_32, imp_make_lut_slice_by, imp_reflect_byte, imp_reflect_value,
};

imp_make_lut_32!(make_lut_32, u8, reflect_byte_8, reflect_value_8);
imp_make_lut_256!(make_lut_256, u8, reflect_byte_8, reflect_value_8);
imp_make_lut_slice_by!(make_sliced_lut, u8, make_lut_256);

imp_crc_update_lut_32!(update_lut_32, u8);
imp_crc_update_lut_256!(update_lut_256, u8);
imp_crc_update_slice_by!(update_slice_by, u8);

imp_reflect_value!(reflect_value_8, u8);
imp_reflect_byte!(reflect_byte_8, u8);

#[inline]
fn update_slice_by_4<'a, const REFLECT: bool>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]],
) -> (u8, &'a [u8]) {
    const STEP: usize = 4;

    assert!(lut.len() >= STEP);

    if REFLECT {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x3] as usize]
                ^ lut[0x1][bytes[0x2] as usize]
                ^ lut[0x2][bytes[0x1] as usize]
                ^ lut[0x3][bytes[0x0] as usize ^ crc as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x3][bytes[0x0] as usize ^ crc as usize]
                ^ lut[0x2][bytes[0x1] as usize]
                ^ lut[0x1][bytes[0x2] as usize]
                ^ lut[0x0][bytes[0x3] as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_8<'a, const REFLECT: bool>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]],
) -> (u8, &'a [u8]) {
    const STEP: usize = 8;

    assert!(lut.len() >= STEP);

    if REFLECT {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0x7] as usize]
                ^ lut[0x1][bytes[0x6] as usize]
                ^ lut[0x2][bytes[0x5] as usize]
                ^ lut[0x3][bytes[0x4] as usize]
                ^ lut[0x4][bytes[0x3] as usize]
                ^ lut[0x5][bytes[0x2] as usize]
                ^ lut[0x6][bytes[0x1] as usize]
                ^ lut[0x7][bytes[0x0] as usize ^ crc as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x7][bytes[0x0] as usize ^ crc as usize]
                ^ lut[0x6][bytes[0x1] as usize]
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
fn update_slice_by_16<'a, const REFLECT: bool>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]],
) -> (u8, &'a [u8]) {
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
                ^ lut[0x8][bytes[0x7] as usize]
                ^ lut[0x9][bytes[0x6] as usize]
                ^ lut[0xa][bytes[0x5] as usize]
                ^ lut[0xb][bytes[0x4] as usize]
                ^ lut[0xc][bytes[0x3] as usize]
                ^ lut[0xd][bytes[0x2] as usize]
                ^ lut[0xe][bytes[0x1] as usize]
                ^ lut[0xf][bytes[0x0] as usize ^ crc as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0xf][bytes[0x0] as usize ^ crc as usize]
                ^ lut[0xe][bytes[0x1] as usize]
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
fn update_slice_by_32<'a, const REFLECT: bool>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]],
) -> (u8, &'a [u8]) {
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
                ^ lut[0x18][bytes[0x07] as usize]
                ^ lut[0x19][bytes[0x06] as usize]
                ^ lut[0x1a][bytes[0x05] as usize]
                ^ lut[0x1b][bytes[0x04] as usize]
                ^ lut[0x1c][bytes[0x03] as usize]
                ^ lut[0x1d][bytes[0x02] as usize]
                ^ lut[0x1e][bytes[0x01] as usize]
                ^ lut[0x1f][bytes[0x00] as usize ^ crc as usize];

            bytes = &bytes[STEP..];
        }
    } else {
        while bytes.len() >= STEP {
            crc = lut[0x1f][bytes[0x00] as usize ^ crc as usize]
                ^ lut[0x1e][bytes[0x01] as usize]
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
        // CRC-8/WCDMA
        const POLY: u8 = 0x9B;
        const INIT: u8 = 0x00;
        const REFLECT: bool = true;
        const XOR_OUT: u8 = 0x00;

        const SAMPLES: [(&str, u8); 8] = [
            ("", 0x00),
            ("0", 0x1B),
            ("012", 0x96),
            ("0123456", 0xE6),
            ("123456789", 0x25),
            ("0123456789ABCDE", 0x2C),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x67),
            ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0x78),
        ];

        let lut32 = make_lut_32(POLY, REFLECT);
        let lut256 = make_lut_256(POLY, REFLECT);
        let lut256x_n = make_sliced_lut::<SLICES>(POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_lut_32::<REFLECT>(INIT, sample.0.as_bytes(), &lut32) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256::<REFLECT>(INIT, sample.0.as_bytes(), &lut256) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_slice_by::<SLICES, REFLECT>(INIT, sample.0.as_bytes(), &lut256x_n) ^ XOR_OUT,
                sample.1
            );
        }
    }

    #[test]
    fn without_reflect() {
        // CRC-8/ITU
        const POLY: u8 = 0x07;
        const INIT: u8 = 0x00;
        const REFLECT: bool = false;
        const XOR_OUT: u8 = 0x55;

        const SAMPLES: [(&str, u8); 8] = [
            ("", 0x55),
            ("0", 0xC5),
            ("012", 0xC6),
            ("0123456", 0xDA),
            ("123456789", 0xA1),
            ("0123456789ABCDE", 0x85),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x3D),
            ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0xF6),
        ];

        let lut32 = make_lut_32(POLY, REFLECT);
        let lut256 = make_lut_256(POLY, REFLECT);
        let lut256x_n = make_sliced_lut(POLY, REFLECT);

        for sample in SAMPLES {
            assert_eq!(
                update_lut_32::<REFLECT>(INIT, sample.0.as_bytes(), &lut32) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_lut_256::<REFLECT>(INIT, sample.0.as_bytes(), &lut256) ^ XOR_OUT,
                sample.1
            );
            assert_eq!(
                update_slice_by::<SLICES, REFLECT>(INIT, sample.0.as_bytes(), &lut256x_n) ^ XOR_OUT,
                sample.1
            );
        }
    }
}
