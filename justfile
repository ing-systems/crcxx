# eg triples
# armv7-unknown-linux-gnueabihf
# aarch64-unknown-linux-gnu
# x86_64-unknown-linux-gnu

default:
  @just --list --unsorted --color=always | rg -v "    default"

# Format source code
fmt:
    cargo +nightly fmt

clippy:
  cargo +nightly clippy --workspace

# Build selected TARGET for ARCH
build TRIPLE:
  cross build --target {{ TRIPLE }} --release
