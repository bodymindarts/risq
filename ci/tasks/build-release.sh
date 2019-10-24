#!/bin/bash

set -eu

REPO=${REPO:-repo}
BINARY=risq

export CARGO_HOME="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )/cargo-home"

pushd ${REPO}

export CARGO_TARGET_DIR="../cargo-target-dir"

make build-${TARGET}-release

if [[ "${OUT}" != "" ]]; then
mv ${CARGO_TARGET_DIR}/${TARGET}/release/${BINARY} ${RELEASE_ROOT}/artifacts/${BINARY}-${TARGET}
fi
