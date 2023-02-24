macro_rules! imp_crc_update_lut_32 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub const fn $name<const REFLECT: bool>(
            mut crc: $ty, bytes: &[u8], lut: &[$ty; 32],
        ) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let mut i = 0;
            while i < bytes.len() {
                let b = bytes[i];

                if REFLECT {
                    let index = ((crc & 0xFF) ^ (b as $ty)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }
                } else {
                    let index = (((crc >> (BITS - 8)) & 0xFF) ^ (b as $ty)) as usize;

                    if BITS > 8 {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index & 0xF] ^ lut[16 + ((index >> 4) & 0xF)];
                    }
                }

                i += 1;
            }

            crc
        }
    };
}

macro_rules! imp_crc_update_lut_256 {
    ($name: ident, $ty: ty) => {
        #[inline]
        pub const fn $name<const REFLECT: bool>(
            mut crc: $ty, bytes: &[u8], lut: &[$ty; 256],
        ) -> $ty {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let mut i = 0;
            while i < bytes.len() {
                let b = bytes[i];
                if REFLECT {
                    let index = ((b as $ty) ^ crc & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc >> SHIFT);
                    } else {
                        crc = lut[index];
                    }
                } else {
                    let index = ((crc >> (BITS - 8)) ^ (b as $ty) & 0xFF) as usize;

                    if BITS > 8 {
                        crc = lut[index] ^ (crc << SHIFT);
                    } else {
                        crc = lut[index];
                    }
                }

                i += 1;
            }

            crc
        }
    };
}
