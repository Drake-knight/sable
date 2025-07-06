# Sable

Sable is a small, embeddable scripting language implemented in Rust. It compiles
source to bytecode and runs it on a stack based virtual machine with a mark and
sweep garbage collector, closures, arrays, maps, and a batteries included
standard library.

## Building

    cargo build --release

## Running

    cargo run --bin sable -- path/to/script.sl
    cargo run --bin sable -- --disasm path/to/script.sl
    cargo run --bin sable                 # start a read eval loop

## Language

    let x = 10;
    fn square(n) { return n * n; }
    for i in 0..x { print(square(i)); }

    let words = split("a,b,c", ",");
    let counts = map();
    for w in words { counts[w] = 1; }

The standard library covers strings, arrays, maps, math, JSON, CSV, hashing,
base32, base64, hex, URL encoding, glob matching, statistics, and more.

## Fuzzing

The `fuzz` directory contains cargo-fuzz targets. Dependencies are vendored so
the harnesses build without network access.

    cargo +nightly fuzz run source_fuzzer
    cargo +nightly fuzz run chunk_fuzzer

## Tests

    cargo test
