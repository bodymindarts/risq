#!/bin/bash

set -e

VERSION="$(cat version/number)"

pushd repo

git checkout "v${VERSION}"

cat <<EOF | cargo login
${CRATES_API_TOKEN}
EOF

cargo publish --all-features --no-verify --locked
