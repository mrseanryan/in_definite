# exit when any command fails
set -e

./test.sh

echo "Publishing..."

pushd in_definite

cargo release

popd

./_prepare-changelog.sh

echo "Publishing [done]"
