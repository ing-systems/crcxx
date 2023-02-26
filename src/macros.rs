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
        impl<'a> Calculator<'a, NoLookupTable> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: () }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                let crc = self.update(initialize(self.params, self.params.init), bytes);

                finalize(self.params, crc)
            }

            pub const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_no_lut(crc, self.params.width, self.params.poly, self.params.refin, bytes)
            }
        }
    };
}

macro_rules! imp_crc_lut_32 {
    ($ty: ty) => {
        impl<'a> Calculator<'a, LookupTable32> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_32(params.width, params.poly, params.refin) }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                let crc = self.update(initialize(self.params, self.params.init), bytes);

                finalize(self.params, crc)
            }

            pub const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_32(crc, bytes, &self.lut, self.params.refin)
            }
        }
    };
}

macro_rules! imp_crc_lut_256 {
    ($ty: ty) => {
        impl<'a> Calculator<'a, LookupTable256> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_256(params.width, params.poly, params.refin) }
            }

            #[inline]
            pub const fn calculate(&self, bytes: &[u8]) -> $ty {
                let crc = self.update(initialize(self.params, self.params.init), bytes);

                finalize(self.params, crc)
            }

            pub const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_256(crc, bytes, &self.lut, self.params.refin)
            }
        }
    };
}

macro_rules! imp_crc_lut_256x_n {
    ($ty: ty, $slices: literal) => {
        impl<'a> Calculator<'a, LookupTable256xN<$slices>> {
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self {
                    params,
                    lut: make_lut_256x_n::<$slices>(params.width, params.poly, params.refin),
                }
            }

            #[inline]
            pub fn calculate(&self, bytes: &[u8]) -> $ty {
                let crc = self.update(initialize(self.params, self.params.init), bytes);

                finalize(self.params, crc)
            }

            pub fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_256x_n(crc, bytes, &self.lut, self.params.refin)
            }
        }
    };
}
