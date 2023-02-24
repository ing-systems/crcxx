use crate::common::*;

imp_crc_update_lut_32!(crc32_update_lut_32, u32);
imp_crc_update_ref_lut_32!(crc32_update_ref_lut_32, u32);

imp_crc_update_lut_256!(crc32_update_lut_256, u32);
imp_crc_update_ref_lut_256!(crc32_update_ref_lut_256, u32);

imp_make_sliced_lut!(crc32_make_sliced_lut, u32, crate::lut::crc32_make_lut_256);

pub fn crc32_update(mut crc: u32, mut buf: &[u8], lut: &[[u32; 256]]) -> u32 {
    // slice-by-32
    if SLICES >= 32 && lut.len() >= SLICE_32 && buf.len() >= SLICE_32 {
        while buf.len() >= SLICE_32 {
            crc = lut[0x1f][buf[0x00] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1e][buf[0x01] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1d][buf[0x02] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1c][buf[0x03] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x1b][buf[0x04] as usize]
                ^ lut[0x1a][buf[0x05] as usize]
                ^ lut[0x19][buf[0x06] as usize]
                ^ lut[0x18][buf[0x07] as usize]
                ^ lut[0x17][buf[0x08] as usize]
                ^ lut[0x16][buf[0x09] as usize]
                ^ lut[0x15][buf[0x0a] as usize]
                ^ lut[0x14][buf[0x0b] as usize]
                ^ lut[0x13][buf[0x0c] as usize]
                ^ lut[0x12][buf[0x0d] as usize]
                ^ lut[0x11][buf[0x0e] as usize]
                ^ lut[0x10][buf[0x0f] as usize]
                ^ lut[0x0f][buf[0x10] as usize]
                ^ lut[0x0e][buf[0x11] as usize]
                ^ lut[0x0d][buf[0x12] as usize]
                ^ lut[0x0c][buf[0x13] as usize]
                ^ lut[0x0b][buf[0x14] as usize]
                ^ lut[0x0a][buf[0x15] as usize]
                ^ lut[0x09][buf[0x16] as usize]
                ^ lut[0x08][buf[0x17] as usize]
                ^ lut[0x07][buf[0x18] as usize]
                ^ lut[0x06][buf[0x19] as usize]
                ^ lut[0x05][buf[0x1a] as usize]
                ^ lut[0x04][buf[0x1b] as usize]
                ^ lut[0x03][buf[0x1c] as usize]
                ^ lut[0x02][buf[0x1d] as usize]
                ^ lut[0x01][buf[0x1e] as usize]
                ^ lut[0x00][buf[0x1f] as usize];

            buf = &buf[SLICE_32..];
        }
    }

    // slice-by-16
    if SLICES >= 16 && lut.len() >= SLICE_16 && buf.len() >= SLICE_16 {
        while buf.len() >= SLICE_16 {
            crc = lut[0xf][buf[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xe][buf[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0xd][buf[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xc][buf[0x3] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0xb][buf[0x4] as usize]
                ^ lut[0xa][buf[0x5] as usize]
                ^ lut[0x9][buf[0x6] as usize]
                ^ lut[0x8][buf[0x7] as usize]
                ^ lut[0x7][buf[0x8] as usize]
                ^ lut[0x6][buf[0x9] as usize]
                ^ lut[0x5][buf[0xa] as usize]
                ^ lut[0x4][buf[0xb] as usize]
                ^ lut[0x3][buf[0xc] as usize]
                ^ lut[0x2][buf[0xd] as usize]
                ^ lut[0x1][buf[0xe] as usize]
                ^ lut[0x0][buf[0xf] as usize];

            buf = &buf[SLICE_16..];
        }
    }

    // slice-by-8
    if SLICES >= 8 && lut.len() >= SLICE_8 && buf.len() >= SLICE_8 {
        while buf.len() >= SLICE_8 {
            crc = lut[0x7][buf[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x6][buf[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x5][buf[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x4][buf[0x3] as usize ^ (crc & 0xFF) as usize]
                ^ lut[0x3][buf[0x4] as usize]
                ^ lut[0x2][buf[0x5] as usize]
                ^ lut[0x1][buf[0x6] as usize]
                ^ lut[0x0][buf[0x7] as usize];

            buf = &buf[SLICE_8..];
        }
    }

    // slice-by-4
    if SLICES >= 4 && lut.len() >= SLICE_4 && buf.len() >= SLICE_4 {
        while buf.len() >= SLICE_4 {
            crc = lut[0x3][buf[0x0] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x2][buf[0x1] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1][buf[0x2] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x0][buf[0x3] as usize ^ (crc & 0xFF) as usize];

            buf = &buf[SLICE_4..];
        }
    }

    crc32_update_lut_256(crc, buf, &lut[0])
}

pub fn crc32_update_ref(mut crc: u32, mut buf: &[u8], lut: &[[u32; 256]]) -> u32 {
    // slice-by-32
    if SLICES >= 32 && lut.len() >= SLICE_32 && buf.len() >= SLICE_32 {
        while buf.len() >= SLICE_32 {
            crc = lut[0x00][buf[0x1f] as usize]
                ^ lut[0x01][buf[0x1e] as usize]
                ^ lut[0x02][buf[0x1d] as usize]
                ^ lut[0x03][buf[0x1c] as usize]
                ^ lut[0x04][buf[0x1b] as usize]
                ^ lut[0x05][buf[0x1a] as usize]
                ^ lut[0x06][buf[0x19] as usize]
                ^ lut[0x07][buf[0x18] as usize]
                ^ lut[0x08][buf[0x17] as usize]
                ^ lut[0x09][buf[0x16] as usize]
                ^ lut[0x0a][buf[0x15] as usize]
                ^ lut[0x0b][buf[0x14] as usize]
                ^ lut[0x0c][buf[0x13] as usize]
                ^ lut[0x0d][buf[0x12] as usize]
                ^ lut[0x0e][buf[0x11] as usize]
                ^ lut[0x0f][buf[0x10] as usize]
                ^ lut[0x10][buf[0x0f] as usize]
                ^ lut[0x11][buf[0x0e] as usize]
                ^ lut[0x12][buf[0x0d] as usize]
                ^ lut[0x13][buf[0x0c] as usize]
                ^ lut[0x14][buf[0x0b] as usize]
                ^ lut[0x15][buf[0x0a] as usize]
                ^ lut[0x16][buf[0x09] as usize]
                ^ lut[0x17][buf[0x08] as usize]
                ^ lut[0x18][buf[0x07] as usize]
                ^ lut[0x19][buf[0x06] as usize]
                ^ lut[0x1a][buf[0x05] as usize]
                ^ lut[0x1b][buf[0x04] as usize]
                ^ lut[0x1c][buf[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1d][buf[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1e][buf[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x1f][buf[0x0] as usize ^ (crc & 0xFF) as usize];

            buf = &buf[SLICE_32..];
        }
    }

    // slice-by-16
    if SLICES >= 16 && lut.len() >= SLICE_16 && buf.len() >= SLICE_16 {
        while buf.len() >= SLICE_16 {
            crc = lut[0x0][buf[0xf] as usize]
                ^ lut[0x1][buf[0xe] as usize]
                ^ lut[0x2][buf[0xd] as usize]
                ^ lut[0x3][buf[0xc] as usize]
                ^ lut[0x4][buf[0xb] as usize]
                ^ lut[0x5][buf[0xa] as usize]
                ^ lut[0x6][buf[0x9] as usize]
                ^ lut[0x7][buf[0x8] as usize]
                ^ lut[0x8][buf[0x7] as usize]
                ^ lut[0x9][buf[0x6] as usize]
                ^ lut[0xa][buf[0x5] as usize]
                ^ lut[0xb][buf[0x4] as usize]
                ^ lut[0xc][buf[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0xd][buf[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0xe][buf[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0xf][buf[0x0] as usize ^ (crc & 0xFF) as usize];

            buf = &buf[SLICE_16..];
        }
    }

    // slice-by-8
    if SLICES >= 8 && lut.len() >= SLICE_8 && buf.len() >= SLICE_8 {
        while buf.len() >= SLICE_8 {
            crc = lut[0x0][buf[0x7] as usize]
                ^ lut[0x1][buf[0x6] as usize]
                ^ lut[0x2][buf[0x5] as usize]
                ^ lut[0x3][buf[0x4] as usize]
                ^ lut[0x4][buf[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x5][buf[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x6][buf[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x7][buf[0x0] as usize ^ (crc & 0xFF) as usize];

            buf = &buf[SLICE_8..];
        }
    }

    // slice-by-4
    if SLICES >= 4 && lut.len() >= SLICE_4 && buf.len() >= SLICE_4 {
        while buf.len() >= SLICE_4 {
            crc = lut[0x0][buf[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x1][buf[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x2][buf[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x3][buf[0x0] as usize ^ (crc & 0xFF) as usize];

            buf = &buf[SLICE_4..];
        }
    }

    crc32_update_ref_lut_256(crc, buf, &lut[0])
}
