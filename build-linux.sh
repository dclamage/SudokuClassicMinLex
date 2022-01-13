#!/bin/bash

cargo build --release
mkdir -p ./package
cp ./target/release/libsudoku_classic_minlex.so ./package/sudoku_classic_minlex.so
