macro_rules! imp_crc_update_lut_32 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub const fn $name(mut crc: $ty, bytes: &[u8], lut: &[$ty; 32], reflect: bool) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            if reflect {
                let mut i = 0;
                while i < bytes.len() {
                    let b = bytes[i];

                    let index = ((crc & 0xFF) ^ (b as $ty)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }

                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes.len() {
                    let b = bytes[i];

                    let index = (((crc >> (BITS - 8)) & 0xFF) ^ (b as $ty)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }

                    i += 1;
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_lut_256 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub const fn $name(mut crc: $ty, bytes: &[u8], lut: &[$ty; 256], reflect: bool) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            if reflect {
                let mut i = 0;
                while i < bytes.len() {
                    let index = ((crc ^ bytes[i] as $ty) & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index];
                    }

                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes.len() {
                    let index = (((crc >> (BITS - 8)) ^ bytes[i] as $ty) & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index];
                    }

                    i += 1;
                }
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_slice_by {
    ($name: ident, $ty: ty) => {
        pub fn $name<const SLICES: usize>(
            mut crc: $ty, mut bytes: &[u8], lut: &[[$ty; 256]; SLICES], reflect: bool,
        ) -> $ty {
            $crate::cg_assert::assert_lt_eq::<SLICES, { $crate::MAX_SLICES }>();
            $crate::cg_assert::assert_power_of_two::<SLICES>();

            if SLICES >= 32 {
                (crc, bytes) = update_slice_by_32(crc, bytes, lut, reflect);
            }

            if SLICES >= 16 {
                (crc, bytes) = update_slice_by_16(crc, bytes, lut, reflect);
            }

            if SLICES >= 8 {
                (crc, bytes) = update_slice_by_8(crc, bytes, lut, reflect);
            }

            if SLICES >= 4 {
                (crc, bytes) = update_slice_by_4(crc, bytes, lut, reflect);
            }

            update_lut_256(crc, bytes, &lut[0], reflect)
        }
    };
}
