# exit when any command fails
set -e

./test.sh

echo "Publishing..."

pushd in_definite

cargo release --no-confirm --execute

popd

./_prepare-changelog.sh
git push

echo "Publishing [done]"
