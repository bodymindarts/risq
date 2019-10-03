generate-protocols:
	scripts/gen-proto

build:
	cargo build

run: build
	RUST_LOG=debug target/debug/risq daemon

check:
	cargo watch -x check

test:
	cargo watch -x test

run-tor:
	scripts/run-tor
