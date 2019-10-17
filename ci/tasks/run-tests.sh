#!/bin/bash

export CARGO_HOME="$(dirname $( cd "$( dirname "${BASH_SOURCE[0]}" )/../../" >/dev/null && pwd ))/cargo-home"

pushd repo

export CARGO_TARGET_DIR="../cargo-target-dir"

make test-in-ci
