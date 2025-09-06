#!/bin/bash
set -e  # exit when any command fails

pushd in_definite

cargo release

popd
