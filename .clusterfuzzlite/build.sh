#!/bin/bash -eu
export CARGO_NET_OFFLINE=true
cargo fuzz build -O --fuzz-dir "$SRC/fuzz"
TRIPLE="$(rustc -vV | sed -n 's/host: //p')"
BINDIR="$SRC/fuzz/target/$TRIPLE/release"
for target in source_fuzzer chunk_fuzzer; do
  cp "$BINDIR/$target" "$OUT/"
done
