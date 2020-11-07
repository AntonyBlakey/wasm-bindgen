#!/usr/bin/env zsh

for dir in benchmarks ci crates examples guide src tests ; do
    for file in $( grep -l -r 'extern "wasm-bindgen"' $dir ) ; do
        sed --in-place -e 's/extern "wasm-bindgen"/extern "C"/g' $file
    done 
done