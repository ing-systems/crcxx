//! Low level CRC-128 functions.
//!
//! NOTE: The low level API don't handle preprocessing and postprocessing of CRC value.
use crate::imp_crc;

imp_crc!(u128);

#[inline]
#[allow(clippy::missing_const_for_fn)]
fn update_slice_by_4<'a>(
    crc: u128, bytes: &'a [u8], _lut: &[[u128; 256]], _reflect: bool,
) -> (u128, &'a [u8]) {
    (crc, bytes)
}

#[inline]
#[allow(clippy::missing_const_for_fn)]
fn update_slice_by_8<'a>(
    crc: u128, bytes: &'a [u8], _lut: &[[u128; 256]], _reflect: bool,
) -> (u128, &'a [u8]) {
    (crc, bytes)
}

#[inline]
fn update_slice_by_16<'a>(
    mut crc: u128, mut bytes: &'a [u8], lut: &[[u128; 256]], reflect: bool,
) -> (u128, &'a [u8]) {
    const STEP: usize = 16;

    assert!(lut.len() >= STEP);

    if reflect {
        while bytes.len() >= STEP {
            crc = lut[0x0][bytes[0xf] as usize ^ ((crc >> 0x78) & 0xFF) as usize]
                ^ lut[0x1][bytes[0xe] as usize ^ ((crc >> 0x70) & 0xFF) as usize]
                ^ lut[0x2][bytes[0xd] as usize ^ ((crc >> 0x68) & 0xFF) as usize]
                ^ lut[0x3][bytes[0xc] as usize ^ ((crc >> 0x60) & 0xFF) as usize]
                ^ lut[0x4][bytes[0xb] as usize ^ ((crc >> 0x58) & 0xFF) as usize]
                ^ lut[0x5][bytes[0xa] as usize ^ ((crc >> 0x50) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x9] as usize ^ ((crc >> 0x48) & 0xFF) as usize]
                ^ lut[0x7][bytes[0x8] as usize ^ ((crc >> 0x40) & 0xFF) as usize]
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
            crc = lut[0xf][bytes[0x0] as usize ^ ((crc >> 0x78) & 0xFF) as usize]
                ^ lut[0xe][bytes[0x1] as usize ^ ((crc >> 0x70) & 0xFF) as usize]
                ^ lut[0xd][bytes[0x2] as usize ^ ((crc >> 0x68) & 0xFF) as usize]
                ^ lut[0xc][bytes[0x3] as usize ^ ((crc >> 0x60) & 0xFF) as usize]
                ^ lut[0xb][bytes[0x4] as usize ^ ((crc >> 0x58) & 0xFF) as usize]
                ^ lut[0xa][bytes[0x5] as usize ^ ((crc >> 0x50) & 0xFF) as usize]
                ^ lut[0x9][bytes[0x6] as usize ^ ((crc >> 0x48) & 0xFF) as usize]
                ^ lut[0x8][bytes[0x7] as usize ^ ((crc >> 0x40) & 0xFF) as usize]
                ^ lut[0x7][bytes[0x8] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x6][bytes[0x9] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x5][bytes[0xa] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x4][bytes[0xb] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x3][bytes[0xc] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x2][bytes[0xd] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x1][bytes[0xe] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x0][bytes[0xf] as usize ^ (crc & 0xFF) as usize];

            bytes = &bytes[STEP..];
        }
    }

    (crc, bytes)
}

#[inline]
fn update_slice_by_32<'a>(
    mut crc: u128, mut bytes: &'a [u8], lut: &[[u128; 256]], reflect: bool,
) -> (u128, &'a [u8]) {
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
                ^ lut[0x10][bytes[0x0f] as usize ^ ((crc >> 0x78) & 0xFF) as usize]
                ^ lut[0x11][bytes[0x0e] as usize ^ ((crc >> 0x70) & 0xFF) as usize]
                ^ lut[0x12][bytes[0x0d] as usize ^ ((crc >> 0x68) & 0xFF) as usize]
                ^ lut[0x13][bytes[0x0c] as usize ^ ((crc >> 0x60) & 0xFF) as usize]
                ^ lut[0x14][bytes[0x0b] as usize ^ ((crc >> 0x58) & 0xFF) as usize]
                ^ lut[0x15][bytes[0x0a] as usize ^ ((crc >> 0x50) & 0xFF) as usize]
                ^ lut[0x16][bytes[0x09] as usize ^ ((crc >> 0x48) & 0xFF) as usize]
                ^ lut[0x17][bytes[0x08] as usize ^ ((crc >> 0x40) & 0xFF) as usize]
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
            crc = lut[0x1f][bytes[0x00] as usize ^ ((crc >> 0x78) & 0xFF) as usize]
                ^ lut[0x1e][bytes[0x01] as usize ^ ((crc >> 0x70) & 0xFF) as usize]
                ^ lut[0x1d][bytes[0x02] as usize ^ ((crc >> 0x68) & 0xFF) as usize]
                ^ lut[0x1c][bytes[0x03] as usize ^ ((crc >> 0x60) & 0xFF) as usize]
                ^ lut[0x1b][bytes[0x04] as usize ^ ((crc >> 0x58) & 0xFF) as usize]
                ^ lut[0x1a][bytes[0x05] as usize ^ ((crc >> 0x50) & 0xFF) as usize]
                ^ lut[0x19][bytes[0x06] as usize ^ ((crc >> 0x48) & 0xFF) as usize]
                ^ lut[0x18][bytes[0x07] as usize ^ ((crc >> 0x40) & 0xFF) as usize]
                ^ lut[0x17][bytes[0x08] as usize ^ ((crc >> 0x38) & 0xFF) as usize]
                ^ lut[0x16][bytes[0x09] as usize ^ ((crc >> 0x30) & 0xFF) as usize]
                ^ lut[0x15][bytes[0x0a] as usize ^ ((crc >> 0x28) & 0xFF) as usize]
                ^ lut[0x14][bytes[0x0b] as usize ^ ((crc >> 0x20) & 0xFF) as usize]
                ^ lut[0x13][bytes[0x0c] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ lut[0x12][bytes[0x0d] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ lut[0x11][bytes[0x0e] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ lut[0x10][bytes[0x0f] as usize ^ (crc & 0xFF) as usize]
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
        // CRC-82/DARC
        const POLY: u128 = 0x0000_308C_0111_0114_0144_0411;
        const INIT: u128 = 0;
        const REFLECT: bool = true;
        const XOR_OUT: u128 = 0;
        const WIDTH: u8 = 82;

        const SAMPLES: [(&str, u128); 8] = [
            ("", 0x0000_0000_0000_0000),
            ("0", 0x0000_0000_0000_CC30_33C0_3CC0_CCC0_298A),
            ("012", 0x0000_0000_0000_19EC_1152_E845_8932_65E7),
            ("0123456", 0x0000_0000_0002_76F7_98A0_5C55_E61A_E617),
            ("123456789", 0x0000_9EA8_3F62_5023_801F_D612),
            ("0123456789ABCDE", 0x0000_0000_0000_0E44_06B8_B704_E7EF_716F),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x0000_0000_0002_0922_40A2_7B37_5F01_7F02),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0x0000_0000_0001_5FBF_43C4_FDC6_B1E4_3895,
            ),
        ];

        const fn prepare(init: u128) -> u128 {
            if REFLECT { init.reverse_bits() >> (128 - WIDTH) } else { init << (128 - WIDTH) }
        }

        const fn finalize(crc: u128) -> u128 {
            crc ^ XOR_OUT
        }

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            let init = prepare(INIT);

            assert_eq!(
                finalize(update_no_lut(init, WIDTH, POLY, REFLECT, sample.0.as_bytes())),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_32(init, sample.0.as_bytes(), &lut32, REFLECT)),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_256(init, sample.0.as_bytes(), &lut256, REFLECT)),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_256x_n::<SLICES>(
                    INIT,
                    sample.0.as_bytes(),
                    &lut256x_n,
                    REFLECT
                )),
                sample.1
            );
        }
    }

    #[test]
    fn without_reflect() {
        // CRC-82/DARC-NOREFLECT
        const POLY: u128 = 0x0000_308C_0111_0114_0144_0411;
        const INIT: u128 = 0;
        const REFLECT: bool = false;
        const XOR_OUT: u128 = 0;
        const WIDTH: u8 = 82;

        const SAMPLES: [(&str, u128); 8] = [
            ("", 0x0000_0000_0000_0000),
            ("0", 0x0002_138C_86F0_AD32_1130_CE52),
            ("012", 0x1BDB_0B61_19D3_4EF7_ECD4),
            ("0123456", 0x0889_2DE9_C18C_B428_5C72),
            ("123456789", 0x0001_2E0B_19FA_447C_0BF6_27AC),
            ("0123456789ABCDE", 0x0002_1749_BADA_FDC4_CE2B_66F9),
            ("0123456789ABCDEFGHIJKLMNOPQRSTU", 0x0001_F0F9_31A8_E08E_14AA_0A9D),
            (
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                0xFF4A_DD08_D3D5_F85C_E3AA,
            ),
        ];

        const fn prepare(init: u128) -> u128 {
            if REFLECT { init.reverse_bits() >> (128 - WIDTH) } else { init << (128 - WIDTH) }
        }

        const fn finalize(mut crc: u128) -> u128 {
            crc = crc.reverse_bits();

            crc ^ XOR_OUT
        }

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

        for sample in SAMPLES {
            let init = prepare(INIT);

            assert_eq!(
                finalize(update_no_lut(init, WIDTH, POLY, REFLECT, sample.0.as_bytes())),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_32(init, sample.0.as_bytes(), &lut32, REFLECT)),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_256(init, sample.0.as_bytes(), &lut256, REFLECT)),
                sample.1
            );
            assert_eq!(
                finalize(update_lut_256x_n::<SLICES>(
                    INIT,
                    sample.0.as_bytes(),
                    &lut256x_n,
                    REFLECT
                )),
                sample.1
            );
        }
    }
}
