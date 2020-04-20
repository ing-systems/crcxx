use crcxx::*;
use criterion::*;

const CHUNK_SIZES: [u32; 6] = [3, 7, 15, 31, 63, 127];

pub fn bench_crc16(c: &mut Criterion) {
    let lut = crc16::crc16_make_sliced_lut(0x8005, true);

    let mut group = c.benchmark_group("CRC16");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update", *size), |b| {
            b.iter(|| black_box(crc16::crc16_update_ref(0, &bytes, &lut)))
        });
    }

    group.finish();
}

pub fn bench_crc32(c: &mut Criterion) {
    let lut = crc32::crc32_make_sliced_lut(0x04C1_1DB7, true);

    let mut group = c.benchmark_group("CRC32");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update", *size), |b| {
            b.iter(|| black_box(crc32::crc32_update_ref(0, &bytes, &lut)))
        });
    }

    group.finish();
}

pub fn bench_crc64(c: &mut Criterion) {
    let lut = crc64::crc64_make_sliced_lut(0x42f0_e1eb_a9ea_3693, true);

    let mut group = c.benchmark_group("CRC64");

    for size in CHUNK_SIZES.iter() {
        let bytes = vec![0x55u8; *size as usize];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(BenchmarkId::new("update", *size), |b| {
            b.iter(|| black_box(crc64::crc64_update_ref(0, &bytes, &lut)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_crc16, bench_crc32, bench_crc64);
criterion_main!(benches);
