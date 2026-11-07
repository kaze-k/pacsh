.DEFAULT_GOAL := all

.PHONY: all release build fix fmt check test bench clean

all: fix fmt check test bench release

release:
	cargo build --release

build:
	cargo build

fix:
	cargo fix --allow-dirty

fmt:
	cargo fmt

check:
	cargo check
	cargo clippy

test:
	cargo test

bench:
	cargo bench

clean:
	cargo clean
