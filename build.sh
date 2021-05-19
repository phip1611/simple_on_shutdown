#!/usr/bin/env bash

cargo build --all --all-targets --examples
cargo test
cargo run --example minimal
# the other examples need CTRL+C to stop

# test no-std build with some no-std target
#  but don't build tests here, because std is required for them
rustup target add thumbv6m-none-eabi
cargo build --target thumbv6m-none-eabi
