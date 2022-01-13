#!/bin/bash

cargo build --release
mkdir -p ./package
cp ./target/release/libsudoku_classic_minlex.dylib ./package/sudoku_classic_minlex.so
