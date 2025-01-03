.PHONY: build clean run test

BIN := hello

build: \
	target/debug/hello \
	target/debug/true \
	target/debug/false

clean:
	rm -rf target

run:
	cargo run --quiet --bin $(BIN)

test:
	cargo test

target/debug/hello: src/main.rs
	cargo build

target/debug/true: src/bin/true.rs
	cargo build

target/debug/false: src/bin/false.rs
	cargo build
