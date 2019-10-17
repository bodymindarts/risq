#!/bin/bash

export CARGO_HOME="$(dirname $( cd "$( dirname "${BASH_SOURCE[0]}" )/../../" >/dev/null && pwd ))/cargo-home"

pushd repo

make test-in-ci
