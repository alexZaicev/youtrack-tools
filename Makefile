.PHONY: codegen fmt lint build

fmt:
	cargo fmt --all

lint:
	cargo clippy -- -D warnings

build: fmt
	cargo build --release