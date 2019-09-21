echo "Publishing..."

pushd in_definite

cargo release

popd

./prepare-changelog.sh

echo "Publishing [done]"
