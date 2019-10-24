#!/bin/bash

set -eu

export CARGO_HOME="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )/cargo-home"

pushd repo

export CARGO_TARGET_DIR="../cargo-target-dir"

make build-minimal-in-ci
