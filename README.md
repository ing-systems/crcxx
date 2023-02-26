# crcxx

[![Crate](https://img.shields.io/crates/v/crcxx.svg)](https://crates.io/crates/crcxx)
[![API](https://docs.rs/crcxx/badge.svg)](https://docs.rs/crcxx)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/loj512o2qo6q0rwg?svg=true)](https://ci.appveyor.com/project/khrs/crcxx)

The crate computes CRC-8/16/32/64/128 using various methods. Included catalog of CRC parameters simplify usage.
It is applicable from small embedded systems to modern desktops and servers. No unsafe or architecture specific code.

### Processing using no lookup table, single byte per step

The slowest method. No additional memory required.

```rust
use crcxx::crc32::{*, catalog::CRC_32_BZIP2};

const CRC: Crc<NoLookupTable> =
    Crc::<NoLookupTable>::new(&CRC_32_BZIP2);

fn main() {
    // Singlepart data.
    let crc = CRC.compute(b"123456789");
    assert_eq!(crc, 0xFC89_1918);

    // Multipart data.
    let mut multipart = CRC.compute_multipart();
    multipart.update(b"1234");
    multipart.update(b"5678");
    multipart.update(b"9");

    let crc = multipart.value();
    assert_eq!(crc, 0xFC89_1918);
}
```

### Processing using a lookup table with 32 entries, single byte per step

Good compromise between speed and memory consumption for small embedded devices.
Depending on usage scenario usually 2-5 times faster than the previous method.

```rust
use crcxx::crc32::{*, catalog::CRC_32_BZIP2};

const CRC: Crc<LookupTable32> =
    Crc::<LookupTable32>::new(&CRC_32_BZIP2);

fn main() {
    // Singlepart data.
    let crc = CRC.compute(b"123456789");
    assert_eq!(crc, 0xFC89_1918);

    // Multipart data.
    let mut multipart = CRC.compute_multipart();
    multipart.update(b"1234");
    multipart.update(b"5678");
    multipart.update(b"9");

    let crc = multipart.value();
    assert_eq!(crc, 0xFC89_1918);
}
```

### Processing using a lookup table with 256 entries, single byte per step

Depending on usage scenario usually no more than 2 times faster than the previous method.

```rust
use crcxx::crc32::{*, catalog::CRC_32_BZIP2};

const CRC: Crc<LookupTable256> =
    Crc::<LookupTable256>::new(&CRC_32_BZIP2);

fn main() {
    // Singlepart data.
    let crc = CRC.compute(b"123456789");
    assert_eq!(crc, 0xFC89_1918);

    // Multipart data.
    let mut multipart = CRC.compute_multipart();
    multipart.update(b"1234");
    multipart.update(b"5678");
    multipart.update(b"9");

    let crc = multipart.value();
    assert_eq!(crc, 0xFC89_1918);
}
```

### Processing using a lookup table with 256 x SLICES entries, multiple bytes per step

Ultimate method for processing big amounts of data on modern desktops and servers without using architecture
specific instructions. Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.

```rust
use crcxx::crc32::{*, catalog::CRC_32_BZIP2};

const SLICES: usize = 16;
const CRC: Crc<LookupTable256xN<SLICES>> =
    Crc::<LookupTable256xN<SLICES>>::new(&CRC_32_BZIP2);

fn main() {
    // Singlepart data.
    let crc = CRC.compute(b"123456789");
    assert_eq!(crc, 0xFC89_1918);

    // Multipart data.
    let mut multipart = CRC.compute_multipart();
    multipart.update(b"1234");
    multipart.update(b"5678");
    multipart.update(b"9");

    let crc = multipart.value();
    assert_eq!(crc, 0xFC89_1918);
}
```

## MSRV

Current MSRV is 1.59. Keep in mind the official crate policy is latest stable.

## License

This project is licensed under the [Apache 2.0 license](LICENSE)
