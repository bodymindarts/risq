generate-protocols:
	scripts/gen-proto

build:
	cargo build

run: build
	target/debug/risq

test-all:
	cargo test

run-tor:
	scripts/run-tor
