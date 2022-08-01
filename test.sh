#!/bin/sh

set -xe

# create a file used for testing
FILE=~/code/linux-configs
mkdir -p $FILE

# mount point
MOUNT=~/code/personal
mkdir -p $MOUNT
cp -r . $MOUNT

# run the tests
# RUST_BACKTRACE=1
cargo test --all -- --nocapture

