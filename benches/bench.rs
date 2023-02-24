use crcxx::*;
use criterion::*;

const CHUNK_SIZES: [u32; 6] = [3, 5, 7, 11, 13, 16 * 10424];
const SLICES: usize = 16;

pub fn bench_lut_32(c: &mut Criterion) {
    let lut = crc8::make_lut_32(0x9B, true);

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

pub fn bench_lut_256(c: &mut Criterion) {
    let lut = crc8::make_lut_256(0x9B, true);

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

pub fn bench_slice_by(c: &mut Criterion) {
    let lut = crc8::make_sliced_lut::<SLICES>(0x9B, true);

    let mut group = c.benchmark_group("CRC8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new(format!("update_slice_by_{}", SLICES), *size), |b| {
            b.iter(|| black_box(crc8::update_slice_by::<SLICES>(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_32(c: &mut Criterion) {
    let lut = crc16::make_lut_32(0x8005, true);

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
    let lut = crc16::make_lut_256(0x8005, true);

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

pub fn bench_crc16_slice_by(c: &mut Criterion) {
    let lut = crc16::make_sliced_lut::<SLICES>(0x8005, true);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new(format!("update_slice_by_{}", SLICES), *size), |b| {
            b.iter(|| black_box(crc16::update_slice_by::<SLICES>(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_32(c: &mut Criterion) {
    let lut = crc32::make_lut_32(0x04C1_1DB7, true);

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
    let lut = crc32::make_lut_256(0x04C1_1DB7, true);

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

pub fn bench_crc32_slice_by(c: &mut Criterion) {
    let lut = crc32::make_sliced_lut::<SLICES>(0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new(format!("update_slice_by_{}", SLICES), *size), |b| {
            b.iter(|| black_box(crc32::update_slice_by::<SLICES>(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_32(c: &mut Criterion) {
    let lut = crc64::make_lut_32(0x04C1_1DB7, true);

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
    let lut = crc64::make_lut_256(0x42f0_e1eb_a9ea_3693, true);

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

pub fn bench_crc64_slice_by(c: &mut Criterion) {
    let lut = crc64::make_sliced_lut::<SLICES>(0x42f0_e1eb_a9ea_3693, true);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new(format!("update_slice_by_{}", SLICES), *size), |b| {
            b.iter(|| black_box(crc64::update_slice_by::<SLICES>(0, black_box(&bytes), &lut, true)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_lut_32,
    bench_lut_256,
    bench_slice_by,
    bench_crc16_lut_32,
    bench_crc16_lut_256,
    bench_crc16_slice_by,
    bench_crc32_lut_32,
    bench_crc32_lut_256,
    bench_crc32_slice_by,
    bench_crc64_lut_32,
    bench_crc64_lut_256,
    bench_crc64_slice_by
);
criterion_main!(benches);
