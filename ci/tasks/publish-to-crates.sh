#!/bin/bash

set -e

pushd out-repo

cat <<EOF | cargo login
${CRATES_API_TOKEN}
EOF

cargo publish --all-features --no-verify
