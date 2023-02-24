#[macro_export]
macro_rules! imp_make_lut_32 {
    ($name: ident, $ty: ty, $crc: path) => {
        pub const fn $name(width: u8, poly: $ty, reflect: bool) -> [$ty; 32] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut table = [0 as $ty; 32];

            let mut index = 0;
            while index < 16 {
                table[index] = $crc(poly, reflect, index as $ty);
                index += 1;
            }

            let mut index = 0;
            while index < 16 {
                table[index + 16] = $crc(poly, reflect, (index << 4) as $ty);
                index += 1;
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_lut_256 {
    ($name: ident, $ty: ty, $crc: path) => {
        pub const fn $name(width: u8, poly: $ty, reflect: bool) -> [$ty; 256] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut table = [0 as $ty; 256];

            let mut i = 0;
            while i < table.len() {
                table[i] = $crc(poly, reflect, i as $ty);
                i += 1;
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_lut_slice_by {
    ($name: ident, $ty: ty, $make_base_lut_256: path) => {
        pub const fn $name<const SLICES: usize>(
            width: u8, poly: $ty, reflect: bool,
        ) -> [[$ty; 256]; SLICES] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            $crate::cg_assert::assert_lt_eq::<SLICES, { $crate::MAX_SLICES }>();
            $crate::cg_assert::assert_power_of_two::<SLICES>();

            let mut lut = [[0 as $ty; 256]; SLICES];
            lut[0] = $make_base_lut_256(width, poly, reflect);

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
