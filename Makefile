.PHONY: setup

setup: \
	vendor \
	vendor/kyclark \
	vendor/kyclark/command-line-rust

teardown:
	rm -rf vendor

vendor:
	mkdir -p $@

vendor/kyclark:
	mkdir -p $@

vendor/kyclark/command-line-rust:
	git clone https://github.com/kyclark/command-line-rust.git $@
