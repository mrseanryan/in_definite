#!/bin/bash
set -e  # exit when any command fails

./_pre-publish.sh

pushd in_definite

cargo release

popd
