@echo off
cargo build --release
if not exist ".\package" mkdir ".\package"
copy ".\target\release\sudoku_classic_minlex.dll" ".\package\sudoku_classic_minlex.pyd"
