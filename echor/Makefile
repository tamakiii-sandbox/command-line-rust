.PHONY: build clean run test

build: \
	target/debug/main

clean:
	cargo clean

run:
	cargo run --quiet

test:
	cargo test

target/debug/main: src/main.rs
	cargo build
