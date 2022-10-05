.PHONY: build

build:
	cargo build

lint:
	cargo clippy -- -D warnings

release:
	cargo build --release
