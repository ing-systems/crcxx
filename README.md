# crcxx

[![Crate](https://img.shields.io/crates/v/crcxx.svg)](https://crates.io/crates/crcxx)
[![API](https://docs.rs/crcxx/badge.svg)](https://docs.rs/crcxx)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/loj512o2qo6q0rwg?svg=true)](https://ci.appveyor.com/project/khrs/crcxx)

The crate compute CRC-8/16/32/64 using various methods. It is applicable for small embedded systems and modern desktops and servers.
No unsafe or architecture specific code.

### Processing using no lookup table, single byte per step

The slowest method. No additional memory required.

```rust
use crcxx::internals::crc32;

// CRC-32Q
const INIT: u32 = 0;
const POLY: u32 = 0x8141_41AB;
const WIDTH: u8 = 32;
const REFLECT: bool = false;

fn main() {
    let data = "123456789";
    let bytes = data.as_bytes();
    let crc = crc32::update_no_lut(INIT, WIDTH, POLY, REFLECT, &bytes);

    assert_eq!(crc, 0x3010_BF7F);
}
```

### Processing using a lookup table with 32 entries, single byte per step

Good compromise between speed and memory consumption for small embedded devices.
Depending on usage scenario usually 2-5 times faster than the previous method.

```rust
use crcxx::internals::crc32;

// CRC-32Q
const INIT: u32 = 0;
const POLY: u32 = 0x8141_41AB;
const WIDTH: u8 = 32;
const REFLECT: bool = false;

const LUT: [u32; 32] = crc32::make_lut_32(WIDTH, POLY, REFLECT);

fn main() {
    let data = "123456789";
    let bytes = data.as_bytes();
    let crc = crc32::update_lut_32(INIT, &bytes, &LUT, REFLECT);

    assert_eq!(crc, 0x3010_BF7F);
}
```

### Processing using a lookup table with 256 entries, single byte per step

Depending on usage scenario usually no more than 2 times faster than the previous method.

```rust
use crcxx::internals::crc32;

// CRC-32Q
const INIT: u32 = 0;
const POLY: u32 = 0x8141_41AB;
const WIDTH: u8 = 32;
const REFLECT: bool = false;

const LUT: [u32; 256] = crc32::make_lut_256(WIDTH, POLY, REFLECT);

fn main() {
    let data = "123456789";
    let bytes = data.as_bytes();
    let crc = crc32::update_lut_256(INIT, &bytes, &LUT, REFLECT);

    assert_eq!(crc, 0x3010_BF7F);
}
```

### Processing using a lookup table with 256 x SLICES entries, multiple bytes per step

Ultimate method for processing big amounts of data on modern desktops and servers.
Depending on usage scenario (prefer bigger chunks) usually 6 times faster than the previous method.
The recommended number of slices is 16. There is usually less than 10% improvement when going from 16 to 32.

```rust
use crcxx::internals::crc32;

// CRC-32Q
const INIT: u32 = 0;
const POLY: u32 = 0x8141_41AB;
const WIDTH: u8 = 32;
const REFLECT: bool = false;

const SLICES: usize = 16;

const LUT: [[u32; 256]; SLICES] = crc32::make_lut_256x_n::<SLICES>(WIDTH, POLY, REFLECT);

fn main() {
    let data = "123456789";
    let bytes = data.as_bytes();
    let crc = crc32::update_lut_256x_n::<SLICES>(INIT, &bytes, &LUT, REFLECT);

    assert_eq!(crc, 0x3010_BF7F);
}
```

## MSRV

Current MSRV is 1.59. Keep in mind the official crate policy is latest stable.

## License

This project is licensed under the [Apache 2.0 license](LICENSE)
