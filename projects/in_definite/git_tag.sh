if [ "$#" -ne 1 ]; then
    echo "USAGE: $0 [version]"
    exit 1
fi

git tag -a $1

git tag

git push origin $1
