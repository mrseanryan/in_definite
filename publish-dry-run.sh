#!/bin/bash
set -e  # exit when any command fails

./pre-publish.sh

pushd in_definite

cargo release

popd
