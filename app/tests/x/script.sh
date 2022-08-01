#!/bin/sh

set -ex
cd $(dirname $0)
pwd

FILE=./target
if [ -d "$FILE" ]; then
    echo "exists"
    rm -r $FILE
else
    echo "does not exists"
fi

mkdir $FILE
cp -r "./target_copy/"* $FILE

