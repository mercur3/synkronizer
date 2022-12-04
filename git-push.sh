#!/bin/sh

set -ex
cd $(dirname $0)

git push origin --all
git push origin --tags
git push https://gitlab.com/mercur3/synkronizer.git --all
git push https://gitlab.com/mercur3/synkronizer.git --tags

