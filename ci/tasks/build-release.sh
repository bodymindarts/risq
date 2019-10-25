#!/bin/bash

set -eu

REPO=${REPO:-repo}
BINARY=risq
OUT=${OUT:-none}

export CARGO_HOME="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )/cargo-home"

pushd ${REPO}

export CARGO_TARGET_DIR="../cargo-target-dir"

make build-${TARGET}-release

if [[ "${OUT}" != "none" ]]; then
  set -x
  cd ${CARGO_TARGET_DIR}/${TARGET}/release
  mkdir ${BINARY}-${TARGET}
  mv ./${BINARY} ${BINARY}-${TARGET}/

  tar -czvf ${BINARY}-${TARGET}.tar.gz ${BINARY}-${TARGET}

  mv ${BINARY}-${TARGET}.tar.gz ${OUT}/

fi
