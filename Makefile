generate-protocols:
	scripts/gen-proto

build:
	cargo build

build-with-checker:
	cargo build --features "checker"

run: build
	RUST_LOG=debug target/debug/risq daemon

regtest: build
	RUST_LOG=debug target/debug/risq daemon -n BtcRegtest --tor-active=false

check:
	cargo watch -x check

test:
	cargo watch -x test

test-checker:
	cargo watch -s 'cargo test --features "checker"'

run-tor:
	scripts/run-tor

no-of-deps:
	cargo tree | grep -v '(*)' | wc -l
