# crcxx

[![Crate](https://img.shields.io/crates/v/crcxx.svg)](https://crates.io/crates/crcxx)
[![API](https://docs.rs/crcxx/badge.svg)](https://docs.rs/crcxx)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/ing-systems/crcxx.svg?branch=master)](https://travis-ci.org/ing-systems/crcxx)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/loj512o2qo6q0rwg?svg=true)](https://ci.appveyor.com/project/khrs/crcxx)

universal slice-by-4/8/16/32 implementation of CRC-8/16/32/64 algorithms.

[Documentation](https://docs.rs/crcxx)

## Usage

To use `crcxx`, add this to your `Cargo.toml`:

```toml
[dependencies]
crcxx = "0.2"
```

default slice-by-16 algorithm is used. You can change to slice-by-4/8/16/32:

```toml
crcxx = { version = "0.2", default-features = false, features = ["slice-by-8"] }
```

## Example

```rust
use crcxx::crc16;

fn main() {
    // CRC-16/ARC
    let lut = crc16::crc16_make_sliced_lut(0x8005, true);

    let data = b"123456789";
    let crc = crc16::crc16_update_ref(0, data, &lut);

    println!("CRC: {:02X}", crc);
}
```

## License

This project is licensed under the [Apache 2.0 license](LICENSE)
