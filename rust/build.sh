#!/usr/bin/env bash

set -e

targets=(
	x86_64-unknown-linux-gnu
	aarch64-unknown-linux-gnu
	x86_64-apple-darwin
	aarch64-apple-darwin
	x86_64-pc-windows-gnu
	wasm32-unknown-emscripten
)

for t in "${targets[@]}"; do
	if [ "$t" = "wasm32-unknown-emscripten" ]; then
		echo "Building release for $t"
		cargo +nightly build -Zbuild-std --release --target wasm32-unknown-emscripten
	else
		echo "Building release for $t"
		cargo build --release --target "$t"
	fi

	# only uncomment if you want to build debug for other platforms
	# if [ "$t" = "wasm32-unknown-emscripten" ]; then
	# 	echo "Building release for $t"
	# 	cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
	# else
	# 	echo "Building debug for $t"
	# 	cargo build --target "$t"
	# fi
done
