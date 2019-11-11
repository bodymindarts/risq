#!/bin/bash

set -eu

export CARGO_HOME="$(pwd)/cargo-home"
export CARGO_TARGET_DIR="$(pwd)/cargo-target-dir"
export RISQ_BIN_DIR=${CARGO_TARGET_DIR}/debug

pushd repo

make test-in-ci
