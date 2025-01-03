.PHONY: build clean run test

BIN := hello

build: src/main.rs
	cargo build

clean:
	rm -rf hello

run:
	cargo run --quiet --bin $(BIN)

test:
	cargo test
