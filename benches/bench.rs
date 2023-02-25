use crcxx::internals::*;
use criterion::*;

const CHUNK_SIZES: [u32; 10] = [3, 5, 7, 11, 13, 17, 31, 63, 4096, 8192];
const SLICES: usize = 16;

const CRC8_WIDTH: u8 = 8;
const CRC8_INIT: u8 = 0;
const CRC8_POLY: u8 = 0x9B;
const CRC8_REFLECT: bool = true;

const CRC16_WIDTH: u8 = 16;
const CRC16_INIT: u16 = 0;
const CRC16_POLY: u16 = 0x8005;
const CRC16_REFLECT: bool = true;

const CRC32_WIDTH: u8 = 32;
const CRC32_INIT: u32 = 0;
const CRC32_POLY: u32 = 0x04C1_1DB7;
const CRC32_REFLECT: bool = true;

const CRC64_WIDTH: u8 = 64;
const CRC64_INIT: u64 = 0;
const CRC64_POLY: u64 = 0x42F0_E1EB_A9EA_3693;
const CRC64_REFLECT: bool = true;

pub fn bench_crc8_no_lut(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_no_lut", *size), |b| {
            b.iter(|| {
                black_box(crc8::update_no_lut(
                    CRC8_INIT,
                    CRC8_WIDTH,
                    CRC8_POLY,
                    CRC8_REFLECT,
                    black_box(&bytes),
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_32(c: &mut Criterion) {
    let lut = crc8::make_lut_32(CRC8_WIDTH, CRC8_POLY, CRC8_REFLECT);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc8::update_lut_32(0, black_box(&bytes), &lut, CRC8_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256(c: &mut Criterion) {
    let lut = crc8::make_lut_256(CRC8_WIDTH, CRC8_POLY, CRC8_REFLECT);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc8::update_lut_256(0, black_box(&bytes), &lut, CRC8_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256x_n(c: &mut Criterion) {
    let lut = crc8::make_lut_256x_n::<SLICES>(CRC8_WIDTH, CRC8_POLY, CRC8_REFLECT);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc8::update_lut_256x_n::<SLICES>(
                    0,
                    black_box(&bytes),
                    &lut,
                    CRC8_REFLECT,
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc16_no_lut(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_no_lut", *size), |b| {
            b.iter(|| {
                black_box(crc16::update_no_lut(
                    CRC16_INIT,
                    CRC16_WIDTH,
                    CRC16_POLY,
                    CRC16_REFLECT,
                    black_box(&bytes),
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_32(c: &mut Criterion) {
    let lut = crc16::make_lut_32(CRC16_WIDTH, CRC16_POLY, CRC16_REFLECT);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc16::update_lut_32(0, black_box(&bytes), &lut, CRC16_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256(c: &mut Criterion) {
    let lut = crc16::make_lut_256(CRC16_WIDTH, CRC16_POLY, CRC16_REFLECT);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc16::update_lut_256(0, black_box(&bytes), &lut, CRC16_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256x_n(c: &mut Criterion) {
    let lut = crc16::make_lut_256x_n::<SLICES>(CRC16_WIDTH, CRC16_POLY, CRC16_REFLECT);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc16::update_lut_256x_n::<SLICES>(
                    0,
                    black_box(&bytes),
                    &lut,
                    CRC16_REFLECT,
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc32_no_lut(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_no_lut", *size), |b| {
            b.iter(|| {
                black_box(crc32::update_no_lut(
                    CRC32_INIT,
                    CRC32_WIDTH,
                    CRC32_POLY,
                    CRC32_REFLECT,
                    black_box(&bytes),
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_32(c: &mut Criterion) {
    let lut = crc32::make_lut_32(CRC32_WIDTH, CRC32_POLY, CRC32_REFLECT);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc32::update_lut_32(0, black_box(&bytes), &lut, CRC32_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256(c: &mut Criterion) {
    let lut = crc32::make_lut_256(CRC32_WIDTH, CRC32_POLY, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc32::update_lut_256(0, black_box(&bytes), &lut, CRC32_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256x_n(c: &mut Criterion) {
    let lut = crc32::make_lut_256x_n::<SLICES>(CRC32_WIDTH, CRC32_POLY, CRC32_REFLECT);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc32::update_lut_256x_n::<SLICES>(
                    0,
                    black_box(&bytes),
                    &lut,
                    CRC32_REFLECT,
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc64_no_lut(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_no_lut", *size), |b| {
            b.iter(|| {
                black_box(crc64::update_no_lut(
                    CRC64_INIT,
                    CRC64_WIDTH,
                    CRC64_POLY,
                    CRC64_REFLECT,
                    black_box(&bytes),
                ))
            })
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_32(c: &mut Criterion) {
    let lut = crc64::make_lut_32(CRC64_WIDTH, CRC64_POLY, CRC64_REFLECT);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc64::update_lut_32(0, black_box(&bytes), &lut, CRC64_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256(c: &mut Criterion) {
    let lut = crc64::make_lut_256(CRC64_WIDTH, CRC64_POLY, CRC64_REFLECT);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc64::update_lut_256(0, black_box(&bytes), &lut, CRC64_REFLECT)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256x_n(c: &mut Criterion) {
    let lut = crc64::make_lut_256x_n::<SLICES>(CRC64_WIDTH, CRC64_POLY, CRC64_REFLECT);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc64::update_lut_256x_n::<SLICES>(
                    0,
                    black_box(&bytes),
                    &lut,
                    CRC64_REFLECT,
                ))
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_crc8_no_lut,
    bench_crc8_lut_32,
    bench_crc8_lut_256,
    bench_crc8_lut_256x_n,
    bench_crc16_no_lut,
    bench_crc16_lut_32,
    bench_crc16_lut_256,
    bench_crc16_lut_256x_n,
    bench_crc32_no_lut,
    bench_crc32_lut_32,
    bench_crc32_lut_256,
    bench_crc32_lut_256x_n,
    bench_crc64_no_lut,
    bench_crc64_lut_32,
    bench_crc64_lut_256,
    bench_crc64_lut_256x_n
);
criterion_main!(benches);
