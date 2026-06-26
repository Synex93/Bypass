#!/usr/bin/env bash
set -euo pipefail

TARGET="${RUST_TARGET:-x86_64-pc-windows-gnu}"
WORKDIR="${BUILD_WORKDIR:-/workspace}"
OUT_DIR="${BUILD_OUT_DIR:-/out}"
PROFILE="release"

cd "$WORKDIR"

echo "[builder] rustc: $(rustc --version)"
echo "[builder] cargo: $(cargo --version)"
echo "[builder] target: $TARGET"
echo "[builder] workdir: $WORKDIR"
echo "[builder] out_dir: $OUT_DIR"
echo "[builder] args: $*"

cargo build --release --target "$TARGET" "$@"

mkdir -p "$OUT_DIR"

BUILD_DIR="$WORKDIR/target/$TARGET/$PROFILE"

find "$BUILD_DIR" \
    -maxdepth 1 \
    -type f \
    \( -name "*.exe" -o -name "*.dll" \) \
    -exec cp -v {} "$OUT_DIR/" \;

echo "[builder] output files:"
ls -lah "$OUT_DIR"
