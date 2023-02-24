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

macro_rules! imp_crc_update_slice_by {
    ($name: ident, $ty: ty) => {
        pub fn $name<const SLICES: usize, const REFLECT: bool>(
            mut crc: $ty, mut bytes: &[u8], lut: &[[$ty; 256]; SLICES],
        ) -> $ty {
            if SLICES >= 32 {
                (crc, bytes) = update_slice_by_32::<REFLECT>(crc, bytes, lut);
            }

            if SLICES >= 16 {
                (crc, bytes) = update_slice_by_16::<REFLECT>(crc, bytes, lut);
            }

            if SLICES >= 8 {
                (crc, bytes) = update_slice_by_8::<REFLECT>(crc, bytes, lut);
            }

            if SLICES >= 4 {
                (crc, bytes) = update_slice_by_4::<REFLECT>(crc, bytes, lut);
            }

            update_lut_256::<REFLECT>(crc, bytes, &lut[0])
        }
    };
}
