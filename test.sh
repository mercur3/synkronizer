#!/bin/sh

set -xe

# create a file used for testing
FILE=~/code/linux-configs
mkdir -p $FILE

# run the tests
RUST_BACkTRACE=1 cargo test --all

