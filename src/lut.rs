#[macro_export]
macro_rules! imp_make_lut_32 {
    ($name: ident, $ty: ty, $reflect_byte: path, $reflect_value: path) => {
        pub const fn $name(poly: $ty, reflect: bool) -> [$ty; 32] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const TOP_BIT: $ty = 1 << (BITS - 1);

            let mut table = [0 as $ty; 32];

            let mut index = 0;
            while index < 16 {
                let byte = if reflect { $reflect_byte(index as $ty) } else { index as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                let mut bit = 0;
                while bit < 8 {
                    if (value & TOP_BIT) != 0 {
                        value = (value << 1) ^ poly
                    } else {
                        value <<= 1
                    }

                    bit += 1;
                }

                table[index] = if reflect { $reflect_value(value) } else { value };

                index += 1;
            }

            let mut index = 0;
            while index < 16 {
                let byte =
                    if reflect { $reflect_byte((index << 4) as $ty) } else { (index << 4) as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                let mut bit = 0;
                while bit < 8 {
                    if (value & TOP_BIT) != 0 {
                        value = (value << 1) ^ poly
                    } else {
                        value <<= 1
                    }

                    bit += 1;
                }

                table[index + 16] = if reflect { $reflect_value(value) } else { value };

                index += 1;
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_lut_256 {
    ($name: ident, $ty: ty, $reflect_byte: path, $reflect_value: path) => {
        pub const fn $name(poly: $ty, reflect: bool) -> [$ty; 256] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const TOP_BIT: $ty = 1 << (BITS - 1);

            let mut table = [0 as $ty; 256];

            let mut index = 0;
            while index < 256 {
                let byte = if reflect { $reflect_byte(index as $ty) } else { index as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                let mut bit = 0;
                while bit < 8 {
                    if (value & TOP_BIT) != 0 {
                        value = (value << 1) ^ poly
                    } else {
                        value <<= 1
                    };

                    bit += 1;
                }

                table[index] = if reflect { $reflect_value(value) } else { value };

                index += 1;
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_sliced_lut {
    ($name: ident, $ty: ty, $make_base_lut_256: path) => {
        pub const fn $name<const SLICES: usize>(poly: $ty, reflect: bool) -> [[$ty; 256]; SLICES] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let mut lut = [[0 as $ty; 256]; SLICES];
            lut[0] = $make_base_lut_256(poly, reflect);

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

#[macro_export]
macro_rules! imp_reflect_value {
    ($name: ident, $ty:ty) => {
        #[inline]
        const fn $name(mut value: $ty) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let mut reflection: $ty = 0;

            let mut bit = 0;
            while bit < BITS {
                if (value & 0x01) == 1 {
                    reflection |= 1 << ((BITS - 1) - bit);
                }
                value >>= 1;

                bit += 1;
            }

            reflection
        }
    };
}

#[macro_export]
macro_rules! imp_reflect_byte {
    ($name: ident, $ty:ty) => {
        #[inline]
        const fn $name(input: $ty) -> $ty {
            const BITS: usize = 8;

            let mut reflection: $ty = 0;
            let mut value = input;

            let mut bit = 0;
            while bit < BITS {
                if (value & 0x01) == 1 {
                    reflection |= 1 << ((BITS - 1) - bit);
                }
                value >>= 1;

                bit += 1;
            }

            reflection
        }
    };
}
