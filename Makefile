all: test build

fmt:
	cargo fmt --all

clippy:
	cargo fmt --all -- --check
	cargo clippy --all -- -D warnings

build: fmt clippy
	cargo build

test: build
	cargo test --all -- --nocapture

watch: build
	cargo watch -x 'test --all -- --nocapture'

run-benchmark:
	cargo run --release -p benchmark

help:
	cat Makefile
