#!/bin/sh

set -xe
cd $(dirname $0)
pwd

# create a file used for testing
FILE=~/code/linux-configs
mkdir -p "$FILE/home"
mkdir -p "$FILE/config"

# mount point
MOUNT=~/code/personal/synkronizer
mkdir -p $MOUNT
cp -r . $MOUNT
cd $MOUNT
ls -alh .

RUST_BACKTRACE=1 cargo test --all --no-fail-fast

