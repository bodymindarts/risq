#!/bin/bash

set -eu

VERSION=$(cat version/number)
BINARY=risq
WORKSPACE="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )"
REPO_ROOT="${WORKSPACE}/out-repo/git"
RELEASE_ROOT="${WORKSPACE}/gh-release"
RELEASE_NAME="risq release"

pushd $REPO_ROOT

mv ${REPO_ROOT}/../notes.md ${RELEASE_ROOT}/notes.md

mkdir -p ${RELEASE_ROOT}/artifacts
mv x86_64-unknown-linux-gnu/risq ${RELEASE_ROOT}/artifacts/${BINARY}-x86_64-unknown-linux-gnu
mv arm-unknown-linux-gnueabihf/risq ${RELEASE_ROOT}/artifacts/${BINARY}-arm-unknown-linux-gnueabihf

echo "v${VERSION}"                         > ${RELEASE_ROOT}/tag
echo "${RELEASE_NAME} v${VERSION}"         > ${RELEASE_ROOT}/name

exit 1
