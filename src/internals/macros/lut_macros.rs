macro_rules! imp_make_lut_32 {
    ($ty: ty) => {
        /// Create lookup table with 32 entries.
        pub const fn make_lut_32(width: u8, poly: $ty, reflect: bool) -> [$ty; 32] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            assert!(width <= BITS as u8);

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut lut = [0 as $ty; 32];

            let mut index = 0;
            while index < 16 {
                lut[index] = update(poly, reflect, index as $ty);
                index += 1;
            }

            let mut index = 0;
            while index < 16 {
                lut[index + 16] = update(poly, reflect, (index << 4) as $ty);
                index += 1;
            }

            lut
        }
    };
}

macro_rules! imp_make_lut_256 {
    ($ty: ty) => {
        /// Create lookup table with 256 entries.
        ///
        /// Compromise between speed and memory footprint.
        pub const fn make_lut_256(width: u8, poly: $ty, reflect: bool) -> [$ty; 256] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            assert!(width <= BITS as u8);

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut lut = [0 as $ty; 256];

            let mut i = 0;
            while i < lut.len() {
                lut[i] = update(poly, reflect, i as $ty);
                i += 1;
            }

            lut
        }
    };
}

macro_rules! imp_make_lut_256x_n {
    ($ty: ty) => {
        /// Create lookup table with `256xSLICES` entries.
        ///
        /// Highest performance at the expense of memory consumption.
        ///
        /// NOTE: The `SLICES` must be multiple of two less or equal to 32. The constraint is validated at compile time.
        pub const fn make_lut_256x_n<const SLICES: usize>(
            width: u8, poly: $ty, reflect: bool,
        ) -> [[$ty; 256]; SLICES] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            $crate::internals::cg_assert::assert_lt_eq::<SLICES, { $crate::internals::MAX_SLICES }>(
            );
            $crate::internals::cg_assert::assert_power_of_two::<SLICES>();

            assert!(width <= BITS as u8);

            let mut lut = [[0 as $ty; 256]; SLICES];
            lut[0] = make_lut_256(width, poly, reflect);

            if reflect {
                let mut n = 0;
                while n < 256 {
                    let mut crc = lut[0][n];

                    let mut k = 1;
                    while k < SLICES {
                        if BITS > 8 {
                            crc = lut[0][(crc & 0xff) as usize] ^ (crc >> SHIFT);
                        } else {
                            crc = lut[0][(crc & 0xff) as usize];
                        }
                        lut[k][n] = crc;

                        k += 1;
                    }

                    n += 1;
                }
            } else {
                let mut n = 0;
                while n < 256 {
                    let mut crc = lut[0][n];

                    let mut k = 1;
                    while k < SLICES {
                        if BITS > 8 {
                            crc = lut[0][((crc >> (BITS - 8)) & 0xff) as usize] ^ (crc << SHIFT);
                        } else {
                            crc = lut[0][((crc >> (BITS - 8)) & 0xff) as usize];
                        }
                        lut[k][n] = crc;

                        k += 1;
                    }

                    n += 1;
                }
            }

            lut
        }
    };
}
