# crcxx

[![Crate](https://img.shields.io/crates/v/crcxx.svg)](https://crates.io/crates/crcxx)
[![API](https://docs.rs/crcxx/badge.svg)](https://docs.rs/crcxx)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/loj512o2qo6q0rwg?svg=true)](https://ci.appveyor.com/project/khrs/crcxx)

The crate compute CRC using various methods. Can handle small embedded systems and modern desktops and servers.

[Documentation](https://docs.rs/crcxx)

## Usage

To use `crcxx`, add this to your `Cargo.toml`:

```toml
[dependencies]
crcxx = "0.3"
```

## Example

```rust
// Low Level API
use crcxx::crc32;

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

## License

This project is licensed under the [Apache 2.0 license](LICENSE)
