#!/bin/bash

set -o errexit -o nounset

([[ $TRAVIS_BRANCH != "master" ]] || [[ $TRAVIS_PULL_REQUEST != "false" ]]) && exit 0

cargo login $CRATESIO_TOKEN

if cargo publish; then
  echo "Published new version on crates.io"

  echo "Creating new git tag"
  version=$(cat Cargo.toml | sed -ne 's/version = "\(.*\)"/\1/p')

  git config user.name "Jorge Israel Peña"
  git config user.email "jorge.israel.p@gmail.com"

  git remote add upstream "https://$GH_TOKEN@github.com/blaenk/hoedown.git"
  git tag -a "v$version" -m "published https://crates.io/crates/hoedown/$version"
  git push upstream --tags
else
  echo "Version already existed"
fi

