.PHONY: setup teardown build clean

setup: \
	vendor \
	vendor/kyclark \
	vendor/kyclark/command-line-rust

teardown:
	rm -rf vendor

build: \
	hello

clean:
	rm -rf hello

hello: hello.rs
	rustc $<

vendor:
	mkdir -p $@

vendor/kyclark:
	mkdir -p $@

vendor/kyclark/command-line-rust:
	git clone https://github.com/kyclark/command-line-rust.git $@
