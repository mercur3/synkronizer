#!/bin/sh

set -xe
cd $(dirname $0)
pwd

# create a file used for testing
FILE=~/code/linux-configs
mkdir -p $FILE

# mount point
MOUNT=~/code/personal
mkdir -p $MOUNT
cp -r . $MOUNT
ls -alh $MOUNT

# run the tests
# RUST_BACKTRACE=1
cargo test --all -- --nocapture

