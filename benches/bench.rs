use crcxx::*;
use criterion::*;

const CHUNK_SIZES: [u32; 10] = [3, 5, 7, 11, 13, 17, 31, 63, 4096, 8192];
const SLICES: usize = 16;

pub fn bench_crc8_lut_32(c: &mut Criterion) {
    let lut = crc8::make_lut_32(8, 0x9B, true);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc8::update_lut_32(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256(c: &mut Criterion) {
    let lut = crc8::make_lut_256(8, 0x9B, true);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc8::update_lut_256(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256x_n(c: &mut Criterion) {
    let lut = crc8::make_lut_256x_n::<SLICES>(8, 0x9B, true);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc8::update_lut_256x_n::<SLICES>(0, black_box(&bytes), &lut, true))
            })
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_32(c: &mut Criterion) {
    let lut = crc16::make_lut_32(16, 0x8005, true);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc16::update_lut_32(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256(c: &mut Criterion) {
    let lut = crc16::make_lut_256(16, 0x8005, true);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc16::update_lut_256(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256x_n(c: &mut Criterion) {
    let lut = crc16::make_lut_256x_n::<SLICES>(16, 0x8005, true);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc16::update_lut_256x_n::<SLICES>(0, black_box(&bytes), &lut, true))
            })
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_32(c: &mut Criterion) {
    let lut = crc32::make_lut_32(32, 0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc32::update_lut_32(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256(c: &mut Criterion) {
    let lut = crc32::make_lut_256(32, 0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc32::update_lut_256(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256x_n(c: &mut Criterion) {
    let lut = crc32::make_lut_256x_n::<SLICES>(32, 0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc32::update_lut_256x_n::<SLICES>(0, black_box(&bytes), &lut, true))
            })
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_32(c: &mut Criterion) {
    let lut = crc64::make_lut_32(64, 0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_32", *size), |b| {
            b.iter(|| black_box(crc64::update_lut_32(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256(c: &mut Criterion) {
    let lut = crc64::make_lut_256(64, 0x42f0_e1eb_a9ea_3693, true);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256", *size), |b| {
            b.iter(|| black_box(crc64::update_lut_256(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256x_n(c: &mut Criterion) {
    let lut = crc64::make_lut_256x_n::<SLICES>(64, 0x42f0_e1eb_a9ea_3693, true);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update_lut_256x_n", *size), |b| {
            b.iter(|| {
                black_box(crc64::update_lut_256x_n::<SLICES>(0, black_box(&bytes), &lut, true))
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_crc8_lut_32,
    bench_crc8_lut_256,
    bench_crc8_lut_256x_n,
    bench_crc16_lut_32,
    bench_crc16_lut_256,
    bench_crc16_lut_256x_n,
    bench_crc32_lut_32,
    bench_crc32_lut_256,
    bench_crc32_lut_256x_n,
    bench_crc64_lut_32,
    bench_crc64_lut_256,
    bench_crc64_lut_256x_n
);
criterion_main!(benches);
