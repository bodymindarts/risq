#!/bin/bash

set -e

pushd out-repo/git

cat <<EOF | cargo login
${CRATES_API_TOKEN}
EOF

cargo publish --features "all" --no-verify --locked
