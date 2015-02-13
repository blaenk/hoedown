#!/bin/bash

set -o errexit -o nounset

[ $TRAVIS_BRANCH = master ] && [ $TRAVIS_PULL_REQUEST = false ] && exit 0

cargo login $CRATESIO_TOKEN

if cargo publish; then
  echo "Published new version on crates.io"
else
  echo "Version already existed"
fi

