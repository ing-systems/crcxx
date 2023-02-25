#![no_std]
#![forbid(non_ascii_idents, unsafe_code)]
#![deny(
    // macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_casts,
    // trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![warn(
    // unused_crate_dependencies,
    clippy::nursery,
    clippy::pedantic,
    clippy::mutex_atomic,
    clippy::rc_buffer,
    clippy::rc_mutex,
    // clippy::expect_used,
    // clippy::unwrap_used,
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::future_not_send,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    //clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub(crate) const MAX_SLICES: usize = 32;

mod cg_assert;
#[macro_use]
mod macros;

pub mod crc16;
pub mod crc32;
pub mod crc64;
pub mod crc8;
