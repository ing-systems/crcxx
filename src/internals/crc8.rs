//! Low level CRC-8 functions.
//!
//! NOTE: The low level API don't handle preprocessing and postprocessing of CRC value.
//!
//! # Processing bytes without using a lookup table, single byte per step
//!
//! Use *ONLY* when you don't have any spare RAM. Very slow.
//!
//! ```
//! use crcxx::internals::crc8;
//!
//! // CRC-8/DARC
//! const INIT: u8 = 0;
//! const POLY: u8 = 0x39;
//! const WIDTH: u8 = 8;
//! const REFLECT: bool = true;
//!
//! fn main() {
//!     let bytes = vec![0x55_u8; 7];
//!     let crc = crc8::update_no_lut(INIT, WIDTH, POLY, REFLECT, &bytes);
//!
//!     println!("CRC: {:02X}", crc);
//! }
//! ```
//! # Processing bytes using a lookup table with 32 entries, single byte per step
//!
//! ```
//! use crcxx::internals::crc8;
//!
//! // CRC-8/DARC
//! const INIT: u8 = 0;
//! const POLY: u8 = 0x39;
//! const WIDTH: u8 = 8;
//! const REFLECT: bool = true;
//!
//! const LUT: [u8; 32] = crc8::make_lut_32(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let bytes = vec![0x55_u8; 31];
//!     let crc = crc8::update_lut_32(INIT, &bytes, &LUT, REFLECT);
//!
//!     println!("CRC: {:02X}", crc);
//! }
//! ```
//!
//! # Processing bytes using a lookup table with 256 entries, single byte per step
//!
//! ```
//! use crcxx::internals::crc8;
//!
//! // CRC-8/DARC
//! const INIT: u8 = 0;
//! const POLY: u8 = 0x39;
//! const WIDTH: u8 = 8;
//! const REFLECT: bool = true;
//!
//! const LUT: [u8; 256] = crc8::make_lut_256(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let bytes = vec![0x55_u8; 63];
//!     let crc = crc8::update_lut_256(INIT, &bytes, &LUT, REFLECT);
//!
//!     println!("CRC: {:02X}", crc);
//! }
//! ```
//!
//! # Processing bytes using a lookup table with `256xSLICES` entries, multiple bytes per step
//!
//! ```
//! use crcxx::internals::crc8;
//!
//! // CRC-8/DARC
//! const INIT: u8 = 0;
//! const POLY: u8 = 0x39;
//! const WIDTH: u8 = 8;
//! const REFLECT: bool = true;
//!
//! const SLICES: usize = 16;
//!
//! const LUT: [[u8; 256]; SLICES] = crc8::make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);
//!
//! fn main() {
//!     let data = "123456789";
//!     let bytes = data.as_bytes();
//!     let crc = crc8::update_lut_256x_n::<SLICES>(INIT, &bytes, &LUT, REFLECT);
//!
//!     assert_eq!(crc, 0x15);
//! }
//! ```
use crate::imp_crc;

imp_crc!(u8);

#[inline]
fn update_slice_by_4<'a>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]], reflect: bool,
) -> (u8, &'a [u8]) {
    const STEP: usize = 4;

    assert!(lut.len() >= STEP);

    if reflect {
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
fn update_slice_by_8<'a>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]], reflect: bool,
) -> (u8, &'a [u8]) {
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
fn update_slice_by_16<'a>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]], reflect: bool,
) -> (u8, &'a [u8]) {
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
fn update_slice_by_32<'a>(
    mut crc: u8, mut bytes: &'a [u8], lut: &[[u8; 256]], reflect: bool,
) -> (u8, &'a [u8]) {
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
    use crate::internals::MAX_SLICES;

    use super::*;

    const SLICES: usize = MAX_SLICES;

    #[test]
    fn with_reflect() {
        // CRC-8/WCDMA
        const POLY: u8 = 0x9B;
        const INIT: u8 = 0x00;
        const REFLECT: bool = true;
        const XOR_OUT: u8 = 0x00;
        const WIDTH: u8 = 8;

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
        // CRC-8/ITU
        const POLY: u8 = 0x07;
        const INIT: u8 = 0x00;
        const REFLECT: bool = false;
        const XOR_OUT: u8 = 0x55;
        const WIDTH: u8 = 8;

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

        let lut32 = make_lut_32(WIDTH, POLY, REFLECT);
        let lut256 = make_lut_256(WIDTH, POLY, REFLECT);
        let lut256x_n = make_lut_256x_n(WIDTH, POLY, REFLECT);

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
