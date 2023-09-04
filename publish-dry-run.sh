# exit when any command fails
set -e

pushd in_definite

cargo release

popd
