# exit when any command fails
set -e

input=CHANGELOG.md

tmp=tmp/changelog.md.tmp

echo "Preparing changelog ..."

echo "## [Unreleased] - ReleaseDate" > $tmp
echo "### Added" >> $tmp
echo "" >> $tmp
echo "" >> $tmp
echo "### Changed" >> $tmp
echo "" >> $tmp
echo "" >> $tmp

cat $input >> $tmp

cp $tmp $input

git unstage
git add $input
git ci -m "Prepare changelog for further work."

echo "Preparing changelog [done]"
