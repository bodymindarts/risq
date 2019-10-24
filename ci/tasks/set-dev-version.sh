#!/bin/bash

WORKSPACE="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )"
REPO_ROOT="${WORKSPACE}/repo"
VERSION="$(cat version/number | sed 's/-rc.*/-dev/')"
REPO_OUT="${WORKSPACE}/out-repo"

pushd $REPO_ROOT

sed -i'' "0,/version/{s/version.*/version = \"${VERSION}\"/}" Cargo.toml
sed -i'' "/^name = \"risq/,/version/{s/version.*/version = \"${VERSION}\"/}" ./Cargo.lock

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
 git commit -m "Set version to ${VERSION}")

cp -a ${REPO_ROOT} ${REPO_OUT}/git
