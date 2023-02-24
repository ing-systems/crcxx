use crate::common::*;

imp_crc_update_lut_32!(crc8_update_lut_32, u8);
imp_crc_update_lut_256!(crc8_update_lut_256, u8);

imp_make_sliced_lut!(crc8_make_sliced_lut, u8, crate::lut::crc8_make_lut_256);

pub fn crc8_update(mut crc: u8, mut bytes: &[u8], lut: &[[u8; 256]]) -> u8 {
    const REFLECT: bool = false;

    if SLICES >= 32 && lut.len() >= SLICE_32 && bytes.len() >= SLICE_32 {
        (crc, bytes) = update_slice_by_32::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 16 && lut.len() >= SLICE_16 && bytes.len() >= SLICE_16 {
        (crc, bytes) = update_slice_by_16::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 8 && lut.len() >= SLICE_8 && bytes.len() >= SLICE_8 {
        (crc, bytes) = update_slice_by_8::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 4 && lut.len() >= SLICE_4 && bytes.len() >= SLICE_4 {
        (crc, bytes) = update_slice_by_4::<REFLECT>(crc, bytes, lut);
    }

    crc8_update_lut_256::<REFLECT>(crc, bytes, &lut[0])
}

pub fn crc8_update_ref(mut crc: u8, mut bytes: &[u8], lut: &[[u8; 256]]) -> u8 {
    const REFLECT: bool = true;

    if SLICES >= 32 && lut.len() >= SLICE_32 && bytes.len() >= SLICE_32 {
        (crc, bytes) = update_slice_by_32::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 16 && lut.len() >= SLICE_16 && bytes.len() >= SLICE_16 {
        (crc, bytes) = update_slice_by_16::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 8 && lut.len() >= SLICE_8 && bytes.len() >= SLICE_8 {
        (crc, bytes) = update_slice_by_8::<REFLECT>(crc, bytes, lut);
    }

    if SLICES >= 4 && lut.len() >= SLICE_4 && bytes.len() >= SLICE_4 {
        (crc, bytes) = update_slice_by_4::<REFLECT>(crc, bytes, lut);
    }

    crc8_update_lut_256::<REFLECT>(crc, bytes, &lut[0])
}

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
