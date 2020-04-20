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

imp_make_lut_32!(crc8_make_lut_32, u8, reflect_byte_8, reflect_value_8);
imp_make_lut_32!(crc16_make_lut_32, u16, reflect_byte_16, reflect_value_16);
imp_make_lut_32!(crc32_make_lut_32, u32, reflect_byte_32, reflect_value_32);
imp_make_lut_32!(crc64_make_lut_32, u64, reflect_byte_64, reflect_value_64);

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

imp_make_lut_256!(crc8_make_lut_256, u8, reflect_byte_8, reflect_value_8);
imp_make_lut_256!(crc16_make_lut_256, u16, reflect_byte_16, reflect_value_16);
imp_make_lut_256!(crc32_make_lut_256, u32, reflect_byte_32, reflect_value_32);
imp_make_lut_256!(crc64_make_lut_256, u64, reflect_byte_64, reflect_value_64);

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

imp_reflect_value!(reflect_value_8, u8);
imp_reflect_value!(reflect_value_16, u16);
imp_reflect_value!(reflect_value_32, u32);
imp_reflect_value!(reflect_value_64, u64);

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

imp_reflect_byte!(reflect_byte_8, u8);
imp_reflect_byte!(reflect_byte_16, u16);
imp_reflect_byte!(reflect_byte_32, u32);
imp_reflect_byte!(reflect_byte_64, u64);
