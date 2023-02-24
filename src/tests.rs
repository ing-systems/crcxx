use std::prelude::v1::*;

use rand::{thread_rng, Rng};

use super::*;

const ITERATIONS: usize = 1_000;
const MIN_DATA_SIZE: usize = 1;
const MAX_DATA_SIZE: usize = 512;

fn gen_rand_data(min_data_size: usize, max_data_size: usize) -> Box<[u8]> {
    let mut rng = thread_rng();
    let data_size = rng.gen_range(min_data_size, max_data_size);
    let mut buf = vec![0u8; data_size];
    rng.fill(buf.as_mut_slice());

    buf.into_boxed_slice()
}

#[test]
fn test_rand_crc8() {
    let lut = crc8::crc8_make_sliced_lut(0x07, false);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc8::crc8_update_lut_256::<false>(0, &data, &lut[0]);
        let crc_fast = crc8::crc8_update::<false>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc8_reflected() {
    let lut = crc8::crc8_make_sliced_lut(0x07, true);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc8::crc8_update_lut_256::<true>(0, &data, &lut[0]);
        let crc_fast = crc8::crc8_update::<true>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc16() {
    let lut = crc16::crc16_make_sliced_lut(0x1021, false);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc16::crc16_update_lut_256::<false>(0, &data, &lut[0]);
        let crc_fast = crc16::crc16_update::<false>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc16_reflected() {
    let lut = crc16::crc16_make_sliced_lut(0x8005, true);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc16::crc16_update_lut_256::<true>(0, &data, &lut[0]);
        let crc_fast = crc16::crc16_update::<true>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc32() {
    let lut = crc32::crc32_make_sliced_lut(0x814141ab, false);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc32::crc32_update_lut_256::<false>(0, &data, &lut[0]) ^ 0xFFFFFFFF;
        let crc_fast = crc32::crc32_update::<false>(0, &data, &lut) ^ 0xFFFFFFFF;

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc32_reflected() {
    let lut = crc32::crc32_make_sliced_lut(0x04C1_1DB7, true);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc32::crc32_update_lut_256::<true>(0, &data, &lut[0]) ^ 0xFFFF_FFFF;
        let crc_fast = crc32::crc32_update::<true>(0, &data, &lut) ^ 0xFFFF_FFFF;

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc64() {
    let lut = crc64::crc64_make_sliced_lut(0x42f0_e1eb_a9ea_3693, false);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc64::crc64_update_lut_256::<false>(0, &data, &lut[0]);
        let crc_fast = crc64::crc64_update::<false>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}

#[test]
fn test_rand_crc64_reflected() {
    let lut = crc64::crc64_make_sliced_lut(0x42f0_e1eb_a9ea_3693, true);

    for _ in 0..ITERATIONS {
        let data = gen_rand_data(MIN_DATA_SIZE, MAX_DATA_SIZE);

        let crc_trusted = crc64::crc64_update_lut_256::<true>(0, &data, &lut[0]);
        let crc_fast = crc64::crc64_update::<true>(0, &data, &lut);

        assert_eq!(crc_trusted, crc_fast);
    }
}
