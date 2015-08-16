#!/bin/bash

set -o errexit -o nounset

([[ $TRAVIS_BRANCH != "master" ]] || [[ $TRAVIS_PULL_REQUEST != "false" ]]) && exit 0

version=$(cat Cargo.toml | awk -F'"' '/version/{print $2}')

if ! git rev-parse $version > /dev/null 2>&1; then
  echo "Creating new git tag $version"

  git config user.name "Jorge Israel Peña"
  git config user.email "jorge.israel.p@gmail.com"

  git remote add upstream "https://$GH_TOKEN@github.com/blaenk/hoedown.git"
  git tag -a "v$version" -m "published https://crates.io/crates/hoedown/$version"
  git push upstream --tags

  echo "Publishing to crates.io"

  cargo login $CRATESIO_TOKEN

  if cargo publish; then
    echo "Published new version on crates.io"
  else
    echo "Error; Perhaps version already exists?"
  fi
else
    echo "Tag $version already exists; no tag created, no crate published"
fi
