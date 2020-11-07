#!/usr/bin/env zsh

rm -rf target
for file in $(grep -l -r 'extern "C"' .) ; do
    sed --in-place -e 's/extern "C"/extern "C"/g' $file
done 