#[macro_use]
mod lut_macros;
#[macro_use]
mod update_macros;

#[macro_export]
macro_rules! imp_crc {
    ($ty: ty) => {
        imp_make_lut_32!($ty);
        imp_make_lut_256!($ty);
        imp_make_lut_256x_n!($ty);

        imp_update!($ty);
        imp_update_no_lut!($ty);
        imp_update_lut_32!($ty);
        imp_update_lut_256!($ty);
        imp_update_lut_256x_n!($ty);
    };
}
