#[macro_export]
macro_rules! imp_make_lut_32 {
    ($name: ident, $ty: ty, $reflect_byte: path, $reflect_value: path) => {
        pub fn $name(poly: $ty, reflect: bool) -> [$ty; 32] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const TOP_BIT: $ty = 1 << (BITS - 1);

            let mut table = [0 as $ty; 32];

            for index in 0..16usize {
                let byte = if reflect { $reflect_byte(index as $ty) } else { index as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                for _ in 0..8 {
                    if (value & TOP_BIT) != 0 { value = (value << 1) ^ poly } else { value <<= 1 }
                }

                table[index] = if reflect { $reflect_value(value) } else { value };
            }

            for index in 0..16usize {
                let byte =
                    if reflect { $reflect_byte((index << 4) as $ty) } else { (index << 4) as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                for _ in 0..8 {
                    if (value & TOP_BIT) != 0 { value = (value << 1) ^ poly } else { value <<= 1 }
                }

                table[index + 16] = if reflect { $reflect_value(value) } else { value };
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_lut_256 {
    ($name: ident, $ty: ty, $reflect_byte: path, $reflect_value: path) => {
        pub fn $name(poly: $ty, reflect: bool) -> [$ty; 256] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const TOP_BIT: $ty = 1 << (BITS - 1);

            let mut table = [0 as $ty; 256];

            for index in 0..256usize {
                let byte = if reflect { $reflect_byte(index as $ty) } else { index as $ty };

                let mut value: $ty = byte << (BITS - 8);

                // Step through all the bits in the byte
                for _ in 0..8 {
                    if (value & TOP_BIT) != 0 { value = (value << 1) ^ poly } else { value <<= 1 }
                }

                table[index] = if reflect { $reflect_value(value) } else { value };
            }

            table
        }
    };
}

#[macro_export]
macro_rules! imp_make_sliced_lut {
    ($name: ident, $ty: ty, $make_base_lut_256: path) => {
        pub fn $name(poly: $ty, reflect: bool) -> [[$ty; 256]; SLICES] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let base_lut = $make_base_lut_256(poly, reflect);
            let mut sliced_lut = [[0 as $ty; 256]; SLICES];

            sliced_lut[0].copy_from_slice(&base_lut);

            if reflect {
                for n in 0..256 {
                    let mut crc = sliced_lut[0][n];

                    for k in 1..SLICES {
                        if BITS > 8 {
                            crc = sliced_lut[0][(crc & 0xff) as usize] ^ (crc >> SHIFT);
                        } else {
                            crc = sliced_lut[0][(crc & 0xff) as usize];
                        }
                        sliced_lut[k][n] = crc;
                    }
                }
            } else {
                for n in 0..256 {
                    let mut crc = sliced_lut[0][n];

                    for k in 1..SLICES {
                        if BITS > 8 {
                            crc = sliced_lut[0][((crc >> (BITS - 8)) & 0xff) as usize]
                                ^ (crc << SHIFT);
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

#[macro_export]
macro_rules! imp_reflect_value {
    ($name: ident, $ty:ty) => {
        #[inline]
        fn $name(mut value: $ty) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let mut reflection: $ty = 0;

            for i in 0..BITS {
                if (value & 0x01) == 1 {
                    reflection |= 1 << ((BITS - 1) - i)
                }
                value >>= 1;
            }

            reflection
        }
    };
}

#[macro_export]
macro_rules! imp_reflect_byte {
    ($name: ident, $ty:ty) => {
        #[inline]
        fn $name(input: $ty) -> $ty {
            const BITS: usize = 8;

            let mut reflection: $ty = 0;
            let mut value = input;

            for i in 0..BITS {
                if (value & 0x01) == 1 {
                    reflection |= 1 << ((BITS - 1) - i)
                }
                value >>= 1;
            }

            reflection
        }
    };
}
