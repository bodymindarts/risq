#!/bin/bash

set -eu

VERSION=$(cat version/number)
WORKSPACE="$( cd "$( dirname "${BASH_SOURCE[0]}" )/../../../" >/dev/null && pwd )"
REPO_ROOT="${WORKSPACE}/repo"
REPO_OUT="${WORKSPACE}/prepared-repo"

if [[ ! -f ${REPO_ROOT}/ci/release_notes.md ]]; then
  echo >&2 "ci/release_notes.md not found.  Did you forget to write them?"
  exit 1
fi
if [[ "$(cat ${REPO_ROOT}/ci/release_notes.md | wc -l | tr -d [:space:])" == "1" ]];then
  echo >&2 "ci/release_notes.md only contains 1 line. Did you forget to write them?"
  exit 1
fi

pushd $REPO_ROOT

cat <<EOF >new_change_log.md
# [risq release v${VERSION}](https://github.com/bodymindarts/risq/releases/tag/v${VERSION})

$(cat ci/release_notes.md)

$(cat CHANGELOG.md)
EOF
mv new_change_log.md CHANGELOG.md

sed -i'' "0,/version/{s/version.*/version = \"${VERSION}\"/}" Cargo.toml
sed -i'' "/^name = \"risq/,/version/{s/version.*/version = \"${VERSION}\"/}" ./Cargo.lock
mv ${REPO_ROOT}/ci/release_notes.md          ${REPO_OUT}/notes.md
echo "Empty - please add release notes here" > ${REPO_ROOT}/ci/release_notes.md

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
 git commit -m "Release v${VERSION}"
 git tag "v${VERSION}"
)

cp -a ${REPO_ROOT} ${REPO_OUT}/git
