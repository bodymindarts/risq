build:
	cargo build

run: build
	target/debug/risq

test-all:
	cargo test

run-tor:
	tor --DataDirectory .tor -f .torrc
