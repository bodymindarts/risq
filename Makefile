generate-protocols:
	scripts/gen-proto

build:
	cargo build

run: build
	RUST_LOG=debug target/debug/risq daemon

regtest: build
	RUST_LOG=debug target/debug/risq daemon -n BtcRegtest --tor-active=false

check:
	cargo watch -x check

test:
	cargo watch -x test

run-tor:
	scripts/run-tor
