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
        pub fn $name<const REFLECT: bool>(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            for &b in buf {
                if REFLECT {
                    let index = ((crc & 0xFF) ^ <$ty>::from(b)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }
                } else {
                    let index = (((crc >> (BITS - 8)) & 0xFF) ^ <$ty>::from(b)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_lut_256 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub fn $name<const REFLECT: bool>(mut crc: $ty, buf: &[u8], lut: &[$ty]) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            for &b in buf {
                if REFLECT {
                    let index = (<$ty>::from(b) ^ crc & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index];
                    }
                } else {
                    let index = ((crc >> (BITS - 8)) ^ <$ty>::from(b) & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index];
                    }
                }
            }

            crc
        }
    };
}
