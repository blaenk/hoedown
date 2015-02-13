#!/bin/bash

set -o errexit -o nounset

([[ $TRAVIS_BRANCH != "master" ]] || [[ $TRAVIS_PULL_REQUEST != "false" ]]) && exit 0

rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "Jorge Israel Peña"
git config user.email "jorge.israel.p@gmail.com"

git remote add upstream "https://$GH_TOKEN@github.com/blaenk/hoedown.git"
git fetch upstream
git reset upstream/gh-pages

# make the index page redirect to the hoedown documentation
echo '<meta http-equiv=refresh content=0;url=hoedown/index.html>' > index.html

touch .

git add -A .
git ci -m "rebuilding docs from ${rev}"
git push -q upstream HEAD:gh-pages

