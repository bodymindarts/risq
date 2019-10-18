#!/bin/bash

set -eu

VERSION=$(cat version/number)
BINARY=risq
WORKSPACE="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )"
REPO_ROOT="${WORKSPACE}/repo"
RELEASE_ROOT="${WORKSPACE}/gh-release"
RELEASE_NAME="risq release"
REPO_OUT="${WORKSPACE}/out-repo"
export CARGO_HOME="${WORKSPACE}/cargo-home"

if [[ ! -f ${REPO_ROOT}/ci/release_notes.md ]]; then
  echo >&2 "ci/release_notes.md not found.  Did you forget to write them?"
  exit 1
fi

TARGET="x86_64-unknown-linux-gnu"
rustup target add ${TARGET}

pushd $REPO_ROOT

sed -i'' "0,/version/{s/version.*/version = \"${VERSION}\"/}" Cargo.toml
mv ${REPO_ROOT}/ci/release_notes.md          ${RELEASE_ROOT}/notes.md

# GIT!
if [[ -z $(git config --global user.email) ]]; then
  git config --global user.email "risqbot@misthos.io"
fi
if [[ -z $(git config --global user.name) ]]; then
  git config --global user.name "CI Bot"
fi

(cd ${REPO_ROOT}
 git merge --no-edit ${BRANCH}
 git add -A
 git status
 git commit -m "Release v${VERSION}")

export CARGO_TARGET_DIR="../cargo-target-dir"
cargo build --features "all" --target ${TARGET} --release --locked

mkdir -p ${RELEASE_ROOT}/artifacts
mv ${CARGO_TARGET_DIR}/${TARGET}/release/${BINARY} ${RELEASE_ROOT}/artifacts/${BINARY}-${TARGET}

echo "v${VERSION}"                         > ${RELEASE_ROOT}/tag
echo "${RELEASE_NAME} v${VERSION}"         > ${RELEASE_ROOT}/name

# so that future steps in the pipeline can push our changes
cp -a ${REPO_ROOT} ${REPO_OUT}/git
