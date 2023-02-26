macro_rules! imp_crc_initialize {
    ($ty: ty) => {
        #[inline]
        const fn initialize(params: &Params<$ty>, initial: $ty) -> $ty {
            if params.refin {
                initial.reverse_bits() >> (<$ty>::BITS as u8 - params.width)
            } else {
                initial << (<$ty>::BITS as u8 - params.width)
            }
        }
    };
}

macro_rules! imp_crc_finalize {
    ($ty: ty) => {
        #[inline]
        const fn finalize(params: &Params<$ty>, mut crc: $ty) -> $ty {
            if params.refin ^ params.refout {
                crc = crc.reverse_bits();
            }

            if !params.refout {
                crc >>= <$ty>::BITS as u8 - params.width;
            }

            crc ^ params.xorout
        }
    };
}

macro_rules! imp_crc_no_lut {
    ($ty: ty) => {
        impl<'a> Crc<'a, NoLookupTable<$ty>> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: () }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                self.calculate_with(self.params.init, bytes)
            }

            pub const fn calculate_with(&self, initial: $ty, bytes: &[u8]) -> $ty {
                let crc = update_no_lut(
                    initialize(self.params, initial),
                    self.params.width,
                    self.params.poly,
                    self.params.refin,
                    bytes,
                );

                finalize(self.params, crc)
            }
        }
    };
}

macro_rules! imp_crc_lut_32 {
    ($ty: ty) => {
        impl<'a> Crc<'a, LookupTable32<$ty>> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_32(params.width, params.poly, params.refin) }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                self.calculate_with(self.params.init, bytes)
            }

            pub const fn calculate_with(&self, initial: $ty, bytes: &[u8]) -> $ty {
                let crc = update_lut_32(
                    initialize(self.params, initial),
                    bytes,
                    &self.lut,
                    self.params.refin,
                );

                finalize(self.params, crc)
            }
        }
    };
}

macro_rules! imp_crc_lut_256 {
    ($ty: ty) => {
        impl<'a> Crc<'a, LookupTable256<$ty>> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_256(params.width, params.poly, params.refin) }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                self.calculate_with(self.params.init, bytes)
            }

            pub const fn calculate_with(&self, initial: $ty, bytes: &[u8]) -> $ty {
                let crc = update_lut_256(
                    initialize(self.params, initial),
                    bytes,
                    &self.lut,
                    self.params.refin,
                );

                finalize(self.params, crc)
            }
        }
    };
}

macro_rules! imp_crc_lut_256x_n {
    ($ty: ty, $slices: literal) => {
        impl<'a> Crc<'a, LookupTable256xN<$ty, $slices>> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self {
                    params,
                    lut: make_lut_256x_n::<$slices>(params.width, params.poly, params.refin),
                }
            }

            #[inline]
            pub fn calculate(&self, bytes: &[u8]) -> $ty {
                self.calculate_with(self.params.init, bytes)
            }

            pub fn calculate_with(&self, initial: $ty, bytes: &[u8]) -> $ty {
                let crc = update_lut_256x_n(
                    initialize(self.params, initial),
                    bytes,
                    &self.lut,
                    self.params.refin,
                );

                finalize(self.params, crc)
            }
        }
    };
}
