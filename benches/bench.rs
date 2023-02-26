use crcxx::Params;
use criterion::*;

const CHUNK_SIZES: [u32; 10] = [3, 5, 7, 11, 13, 17, 31, 63, 4096, 8192];
const SLICES: usize = 16;

const CRC8_PARAMS: Params<u8> = crcxx::crc8::catalog::CRC_8_DARC;
const CRC16_PARAMS: Params<u16> = crcxx::crc16::catalog::CRC_16_KERMIT;
const CRC32_PARAMS: Params<u32> = crcxx::crc32::catalog::CRC_32_CKSUM;
const CRC64_PARAMS: Params<u64> = crcxx::crc64::catalog::CRC_64_XZ;
const CRC128_PARAMS: Params<u128> = crcxx::crc128::catalog::CRC_82_DARC;

const SANITY_CHECK_DATA: &[u8] = b"123456789";

pub fn bench_crc8_no_lut(c: &mut Criterion) {
    use crcxx::crc8::*;

    const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC8_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC8_PARAMS.check);

    let mut group = c.benchmark_group("CRC-8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("NoLookupTable", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_32(c: &mut Criterion) {
    use crcxx::crc8::*;

    const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC8_PARAMS);

    let mut group = c.benchmark_group("CRC-8");
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC8_PARAMS.check);

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable32", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256(c: &mut Criterion) {
    use crcxx::crc8::*;

    const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC8_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC8_PARAMS.check);

    let mut group = c.benchmark_group("CRC-8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc8_lut_256x_n(c: &mut Criterion) {
    use crcxx::crc8::*;

    const CRC: Calculator<LookupTable256xN<SLICES>> =
        Calculator::<LookupTable256xN<SLICES>>::new(&CRC8_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC8_PARAMS.check);

    let mut group = c.benchmark_group("CRC-8");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256xN", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc16_no_lut(c: &mut Criterion) {
    use crcxx::crc16::*;

    const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC16_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC16_PARAMS.check);

    let mut group = c.benchmark_group("CRC-16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("NoLookupTable", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_32(c: &mut Criterion) {
    use crcxx::crc16::*;

    const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC16_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC16_PARAMS.check);

    let mut group = c.benchmark_group("CRC-16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable32", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256(c: &mut Criterion) {
    use crcxx::crc16::*;

    const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC16_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC16_PARAMS.check);

    let mut group = c.benchmark_group("CRC-16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc16_lut_256x_n(c: &mut Criterion) {
    use crcxx::crc16::*;

    const CRC: Calculator<LookupTable256xN<SLICES>> =
        Calculator::<LookupTable256xN<SLICES>>::new(&CRC16_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC16_PARAMS.check);

    let mut group = c.benchmark_group("CRC-16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256xN", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc32_no_lut(c: &mut Criterion) {
    use crcxx::crc32::*;

    const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC32_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC32_PARAMS.check);

    let mut group = c.benchmark_group("CRC-32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("NoLookupTable", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_32(c: &mut Criterion) {
    use crcxx::crc32::*;

    const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC32_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC32_PARAMS.check);

    let mut group = c.benchmark_group("CRC-32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable32", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256(c: &mut Criterion) {
    use crcxx::crc32::*;

    const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC32_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC32_PARAMS.check);

    let mut group = c.benchmark_group("CRC-32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc32_lut_256x_n(c: &mut Criterion) {
    use crcxx::crc32::*;

    const CRC: Calculator<LookupTable256xN<SLICES>> =
        Calculator::<LookupTable256xN<SLICES>>::new(&CRC32_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC32_PARAMS.check);

    let mut group = c.benchmark_group("CRC-32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256xN", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc64_no_lut(c: &mut Criterion) {
    use crcxx::crc64::*;

    const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC64_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC64_PARAMS.check);

    let mut group = c.benchmark_group("CRC-64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("NoLookupTable", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_32(c: &mut Criterion) {
    use crcxx::crc64::*;

    const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC64_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC64_PARAMS.check);

    let mut group = c.benchmark_group("CRC-64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable32", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256(c: &mut Criterion) {
    use crcxx::crc64::*;

    const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC64_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC64_PARAMS.check);

    let mut group = c.benchmark_group("CRC-64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc64_lut_256x_n(c: &mut Criterion) {
    use crcxx::crc64::*;

    const CRC: Calculator<LookupTable256xN<SLICES>> =
        Calculator::<LookupTable256xN<SLICES>>::new(&CRC64_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC64_PARAMS.check);

    let mut group = c.benchmark_group("CRC-64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256xN", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc128_no_lut(c: &mut Criterion) {
    use crcxx::crc128::*;

    const CRC: Calculator<NoLookupTable> = Calculator::<NoLookupTable>::new(&CRC128_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC128_PARAMS.check);

    let mut group = c.benchmark_group("CRC-128");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("NoLookupTable", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc128_lut_32(c: &mut Criterion) {
    use crcxx::crc128::*;

    const CRC: Calculator<LookupTable32> = Calculator::<LookupTable32>::new(&CRC128_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC128_PARAMS.check);

    let mut group = c.benchmark_group("CRC-128");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable32", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc128_lut_256(c: &mut Criterion) {
    use crcxx::crc128::*;

    const CRC: Calculator<LookupTable256> = Calculator::<LookupTable256>::new(&CRC128_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC128_PARAMS.check);

    let mut group = c.benchmark_group("CRC-128");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
        });
    }

    group.finish();
}

pub fn bench_crc128_lut_256x_n(c: &mut Criterion) {
    use crcxx::crc128::*;

    const CRC: Calculator<LookupTable256xN<SLICES>> =
        Calculator::<LookupTable256xN<SLICES>>::new(&CRC128_PARAMS);
    assert_eq!(CRC.calculate(SANITY_CHECK_DATA), CRC128_PARAMS.check);

    let mut group = c.benchmark_group("CRC-128");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("LookupTable256xN", *size), |b| {
            b.iter(|| black_box(CRC.calculate(&bytes)))
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
    bench_crc64_lut_256x_n,
    bench_crc128_no_lut,
    bench_crc128_lut_32,
    bench_crc128_lut_256,
    bench_crc128_lut_256x_n,
);
criterion_main!(benches);
