#!/bin/sh

set -ex
cd $(dirname $0)

git push origin --force
git push --mirror https://github.com/mercur3/synkronizer.git

