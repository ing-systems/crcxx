// 8, 16, or 32
// Currently I don't see any other way of doing it:
// https://github.com/rust-lang/rust/issues/49146
// https://github.com/rust-lang/cargo/issues/2980
#[cfg(feature = "slice-by-4")]
pub(crate) const SLICES: usize = 4;
#[cfg(feature = "slice-by-8")]
pub(crate) const SLICES: usize = 8;
#[cfg(feature = "slice-by-16")]
pub(crate) const SLICES: usize = 16;
#[cfg(feature = "slice-by-32")]
pub(crate) const SLICES: usize = 32;

pub(crate) const SLICE_4: usize = 4;
pub(crate) const SLICE_8: usize = 8;
pub(crate) const SLICE_16: usize = 16;
pub(crate) const SLICE_32: usize = 32;

macro_rules! imp_crc_update_lut_32 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub fn $name(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            #[allow(arithmetic_overflow)]
            #[allow(exceeding_bitshifts)]
            for &b in buf {
                let index = (((crc >> (BITS - 8)) & 0xFF) ^ <$ty>::from(b)) as usize;

                if BITS > 8 {
                    crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc << 8);
                } else {
                    crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_ref_lut_32 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub fn $name(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            #[allow(arithmetic_overflow)]
            #[allow(exceeding_bitshifts)]
            for &b in buf {
                let index = ((crc & 0xFF) ^ <$ty>::from(b)) as usize;

                if BITS > 8 {
                    crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc >> 8);
                } else {
                    crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_lut_256 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub fn $name(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            #[allow(arithmetic_overflow)]
            #[allow(exceeding_bitshifts)]
            for &b in buf {
                let index = ((crc >> (BITS - 8)) ^ <$ty>::from(b) & 0xFF) as usize;

                if BITS > 8 {
                    crc = lut[index] ^ (crc << 8);
                } else {
                    crc = lut[index];
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_ref_lut_256 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub fn $name(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            #[allow(arithmetic_overflow)]
            #[allow(exceeding_bitshifts)]
            for &b in buf {
                let index = (<$ty>::from(b) ^ crc & 0xFF) as usize;

                if BITS > 8 {
                    crc = lut[index] ^ (crc >> 8);
                } else {
                    crc = lut[index];
                }
            }

            crc
        }
    };
}

macro_rules! imp_make_sliced_lut {
    ($name: ident, $ty: ty, $make_base_lut_256: path) => {
        pub fn $name(poly: $ty, reflect: bool) -> [[$ty; 256]; SLICES] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let base_lut = $make_base_lut_256(poly, reflect);
            let mut sliced_lut = [[0 as $ty; 256]; SLICES];

            sliced_lut[0].copy_from_slice(&base_lut);

            if reflect {
                for n in 0..256 {
                    let mut crc = sliced_lut[0][n];

                    #[allow(arithmetic_overflow)]
                    #[allow(exceeding_bitshifts)]
                    for k in 1..SLICES {
                        if BITS > 8 {
                            crc = sliced_lut[0][(crc & 0xff) as usize] ^ (crc >> 8);
                        } else {
                            crc = sliced_lut[0][(crc & 0xff) as usize];
                        }
                        sliced_lut[k][n] = crc;
                    }
                }
            } else {
                for n in 0..256 {
                    let mut crc = sliced_lut[0][n];

                    #[allow(arithmetic_overflow)]
                    #[allow(exceeding_bitshifts)]
                    for k in 1..SLICES {
                        if BITS > 8 {
                            crc = sliced_lut[0][((crc >> (BITS - 8)) & 0xff) as usize] ^ (crc << 8);
                        } else {
                            crc = sliced_lut[0][((crc >> (BITS - 8)) & 0xff) as usize];
                        }
                        sliced_lut[k][n] = crc;
                    }
                }
            }

            sliced_lut
        }
    };
}
