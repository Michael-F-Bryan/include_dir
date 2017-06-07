#!/bin/bash
set -x

# Only upload the built book to github pages if it's a commit to master
if [ "$TRAVIS_BRANCH" = master -a "$TRAVIS_PULL_REQUEST" = false  -a -z "$NIGHTLY" ]; then
  cargo doc
  travis-cargo doc-upload
fi
