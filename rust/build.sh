#!/usr/bin/env bash

set -e

targets=(
	x86_64-unknown-linux-gnu
	x86_64-pc-windows-gnu
	x86_64-apple-darwin
	aarch64-apple-darwin
)

for t in "${targets[@]}"; do
	echo "Building release for $t"
	cargo build --release --target "$t"

	# only uncomment if you want to build debug for other platforms
	# echo "Building debug for $t"
	# cargo build --target "$t"
done
