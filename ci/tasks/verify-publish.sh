#!/bin/bash

cargo install --all-features risq
if [[ $(risq --version) != "risq $(cat version/number)" ]]; then
  echo "Installed risq does not have expected version number"
  exit 1
fi
risq help
