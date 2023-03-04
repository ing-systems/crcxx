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
        impl<'a> Crc<'a, NoLookupTable> {
            imp_const_compute_methods!($ty, NoLookupTable);

            #[inline]
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: () }
            }

            #[inline]
            const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_no_lut(crc, self.params.width, self.params.poly, self.params.refin, bytes)
            }
        }

        imp_compute_multipart!($ty, NoLookupTable);
    };
}

macro_rules! imp_crc_lut_32 {
    ($ty: ty) => {
        impl<'a> Crc<'a, LookupTable32> {
            imp_const_compute_methods!($ty, LookupTable32);

            #[inline]
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_32(params.width, params.poly, params.refin) }
            }

            #[inline]
            const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_32(crc, bytes, &self.lut, self.params.refin)
            }
        }

        imp_compute_multipart!($ty, LookupTable32);
    };
}

macro_rules! imp_crc_lut_256 {
    ($ty: ty) => {
        impl<'a> Crc<'a, LookupTable256> {
            imp_const_compute_methods!($ty, LookupTable256);

            #[inline]
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self { params, lut: make_lut_256(params.width, params.poly, params.refin) }
            }

            #[inline]
            const fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_256(crc, bytes, &self.lut, self.params.refin)
            }
        }

        imp_compute_multipart!($ty, LookupTable256);
    };
}

macro_rules! imp_crc_lut_256x_n {
    ($ty: ty, $slices: literal) => {
        impl<'a> Crc<'a, LookupTable256xN<$slices>> {
            #[inline]
            pub const fn new(params: &'a Params<$ty>) -> Self {
                Self {
                    params,
                    lut: make_lut_256x_n::<$slices>(params.width, params.poly, params.refin),
                }
            }

            #[inline]
            fn update(&self, crc: $ty, bytes: &[u8]) -> $ty {
                update_lut_256x_n(crc, bytes, &self.lut, self.params.refin)
            }

            /// Compute final CRC value for `bytes` using default initial value.
            #[inline]
            pub fn compute(&self, bytes: &[u8]) -> $ty {
                self.compute_with_initial(self.params.init, bytes)
            }

            /// Compute final CRC value for `bytes` using `initial` value.
            #[inline]
            pub fn compute_with_initial(&self, initial: $ty, bytes: &[u8]) -> $ty {
                let crc = self.update(initialize(self.params, initial), bytes);

                finalize(self.params, crc)
            }

            #[inline]
            pub const fn compute_multipart(
                &'a self,
            ) -> ComputeMultipart<'a, LookupTable256xN<$slices>> {
                self.compute_multipart_with_initial(self.params.init)
            }

            #[inline]
            pub const fn compute_multipart_with_initial(
                &'a self, initial: $ty,
            ) -> ComputeMultipart<'a, LookupTable256xN<$slices>> {
                ComputeMultipart { crc: self, value: initialize(self.params, initial) }
            }
        }

        imp_compute_multipart!($ty, LookupTable256xN<$slices>);
    };
}

macro_rules! imp_const_compute_methods {
    ($ty: ty, $table: ty) => {
        /// Compute final CRC value for `bytes` using default initial value.
        #[inline]
        pub const fn compute(&self, bytes: &[u8]) -> $ty {
            self.compute_with_initial(self.params.init, bytes)
        }

        /// Compute final CRC value for `bytes` using `initial` value.
        #[inline]
        pub const fn compute_with_initial(&self, initial: $ty, bytes: &[u8]) -> $ty {
            let crc = self.update(initialize(self.params, initial), bytes);

            finalize(self.params, crc)
        }

        #[inline]
        pub const fn compute_multipart(&'a self) -> ComputeMultipart<'a, $table> {
            self.compute_multipart_with_initial(self.params.init)
        }

        #[inline]
        pub const fn compute_multipart_with_initial(
            &'a self, initial: $ty,
        ) -> ComputeMultipart<'a, $table> {
            ComputeMultipart { crc: self, value: initialize(self.params, initial) }
        }
    };
}

macro_rules! imp_compute_multipart {
    ($ty: ty, $table: ty) => {
        impl<'a> ComputeMultipart<'a, $table> {
            #[inline]
            pub fn update(&mut self, bytes: &[u8]) -> &mut Self {
                self.value = self.crc.update(self.value, bytes);

                self
            }

            #[inline]
            pub const fn value(&self) -> $ty {
                finalize(self.crc.params, self.value)
            }
        }
    };
}
