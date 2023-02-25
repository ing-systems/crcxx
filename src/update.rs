macro_rules! imp_crc {
    ($name: ident, $ty: ty) => {
        #[inline]
        const fn $name(poly: $ty, reflect: bool, mut value: $ty) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            if reflect {
                let mut i = 0;
                while i < 8 {
                    value = (value >> 1) ^ ((value & 1) * poly);
                    i += 1;
                }
            } else {
                value <<= (BITS - 8);

                let mut i = 0;
                while i < 8 {
                    value = (value << 1) ^ (((value >> (BITS - 1)) & 1) * poly);
                    i += 1;
                }
            }

            value
        }
    };
}

macro_rules! imp_update_no_lut {
    ($ty: ty, $crc: path) => {
        #[inline]
        pub const fn update_no_lut(
            mut crc: $ty, width: u8, poly: $ty, reflect: bool, bytes: &[u8],
        ) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            if BITS > 8 {
                if reflect {
                    let mut i = 0;
                    while i < bytes.len() {
                        let value = (crc ^ bytes[i] as $ty) & 0xFF;
                        crc = $crc(poly, reflect, value) ^ (crc >> SHIFT);

                        i += 1;
                    }
                } else {
                    let mut i = 0;
                    while i < bytes.len() {
                        let value = ((crc >> (BITS - 8)) ^ bytes[i] as $ty) & 0xFF;
                        crc = $crc(poly, reflect, value) ^ (crc << SHIFT);

                        i += 1;
                    }
                }
            } else {
                let mut i = 0;
                while i < bytes.len() {
                    crc = $crc(poly, reflect, crc ^ bytes[i] as $ty);

                    i += 1;
                }
            }

            crc
        }
    };
}

macro_rules! imp_update_lut_32 {
    ($ty: ty) => {
        #[inline]
        pub const fn update_lut_32(
            mut crc: $ty, bytes: &[u8], lut: &[$ty; 32], reflect: bool,
        ) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            if BITS > 8 {
                if reflect {
                    let mut i = 0;
                    while i < bytes.len() {
                        let index = ((crc & 0xFF) ^ (bytes[i] as $ty)) as usize;
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc >> SHIFT);

                        i += 1;
                    }
                } else {
                    let mut i = 0;
                    while i < bytes.len() {
                        let index = (((crc >> (BITS - 8)) & 0xFF) ^ (bytes[i] as $ty)) as usize;
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc << SHIFT);

                        i += 1;
                    }
                }
            } else {
                let mut i = 0;
                while i < bytes.len() {
                    let index = ((crc & 0xFF) ^ (bytes[i] as $ty)) as usize;

                    crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];

                    i += 1;
                }
            }

            crc
        }
    };
}

macro_rules! imp_update_lut_256 {
    ($ty: ty) => {
        #[inline]
        pub const fn update_lut_256(
            mut crc: $ty, bytes: &[u8], lut: &[$ty; 256], reflect: bool,
        ) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            if BITS > 8 {
                if reflect {
                    let mut i = 0;
                    while i < bytes.len() {
                        let index = ((crc ^ bytes[i] as $ty) & 0xFF) as usize;
                        crc = lut[index] ^ (crc >> SHIFT);

                        i += 1;
                    }
                } else {
                    let mut i = 0;
                    while i < bytes.len() {
                        let index = (((crc >> (BITS - 8)) ^ bytes[i] as $ty) & 0xFF) as usize;
                        crc = lut[index] ^ (crc << SHIFT);

                        i += 1;
                    }
                }
            } else {
                let mut i = 0;
                while i < bytes.len() {
                    let index = ((crc ^ bytes[i] as $ty) & 0xFF) as usize;
                    crc = lut[index];

                    i += 1;
                }
            }

            crc
        }
    };
}

macro_rules! imp_update_lut_256x_n {
    ($ty: ty) => {
        pub fn update_lut_256x_n<const SLICES: usize>(
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
