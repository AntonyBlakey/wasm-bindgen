#!/usr/bin/env zsh

for dir in benchmarks ci crates examples guide src tests ; do
    for file in $( grep -l -r 'extern "C"' $dir ) ; do
        sed --in-place -e 's/extern "C"/extern "wasm-bindgen"/g' $file
    done
done