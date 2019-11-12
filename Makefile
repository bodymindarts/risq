generate-protocols:
	scripts/gen-proto

build:
	cargo build

build-with-checker:
	cargo build --no-default-features --features "checker"

build-with-stats:
	cargo build --no-default-features --features "statistics"

build-all:
	cargo build --features-all

run: build
	RUST_LOG=debug target/debug/risq daemon

run-dummy-seed:
	cargo build --features "all dummy-seed"
	target/debug/risq dummy-seed
run-alice:
	./target/debug/risq d -n BtcRegtest --no-tor
run-bob:
	./target/debug/risq d -p 6000 --api-port 8488 -n BtcRegtest --no-tor

check:
	cargo watch -x clippy

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test --no-default-features --features "all dummy-seed" -- --nocapture'

test-in-ci:
	cargo clippy --all-features
	cargo test --all-features --verbose --locked

integration:
	cargo build --features "statistics dummy-seed"
	export RISQ_BIN_DIR="$(if $(RISQ_BIN_DIR),$(RISQ_BIN_DIR),./target/debug)" && bats -t -r test/integration

build-minimal-release:
	cargo build --locked --release --no-default-features --features "fail-on-warnings"

build-arm-unknown-linux-gnueabihf-release:
	cargo build --locked --release --target arm-unknown-linux-gnueabihf

build-x86_64-unknown-linux-gnu-release:
	cargo build --locked --release --target x86_64-unknown-linux-gnu

run-tor:
	scripts/run-tor

no-of-deps:
	cargo tree | grep -v '(*)' | grep -v '\[' | wc -l

.PHONY: test
