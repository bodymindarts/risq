#!/bin/bash

set -eu

VERSION=""
if [[ -f version/number ]];then
  VERSION="$(cat version/number)"
fi

REPO=${REPO:-repo}
BINARY=risq
OUT=${OUT:-none}
WORKSPACE="$(pwd)"
export CARGO_HOME="$(pwd)/cargo-home"
export CARGO_TARGET_DIR="$(pwd)/cargo-target-dir"

pushd ${REPO}


make build-${TARGET}-release

if [[ "${OUT}" != "none" ]]; then
  set -x
  cd ${CARGO_TARGET_DIR}/${TARGET}/release
  OUT_DIR="${BINARY}-${TARGET}-${VERSION}"
  mkdir "${OUT_DIR}"
  mv ./${BINARY} ${OUT_DIR}
  tar -czvf ${OUT_DIR}.tar.gz ${OUT_DIR}

  mv ${OUT_DIR}.tar.gz ${WORKSPACE}/${OUT}/

fi
