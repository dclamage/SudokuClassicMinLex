# SudokuX MinLexer

*Created by David Clamage (Rangsk)*

This Rust library is intended to hook into Python 3 and allow for converting a Classic Sudoku puzzle into its minimal lexicographical form. This is useful for keeping a database of known puzzles in canonical form without repeating symmetrically identical ones.

Solver functionality is also provided.

The following symmetries are used:
 - Rotate 90 degrees [2]
 - Swap bands [3!]
 - Swap stacks [3!]
 - Swap rows within bands [(3!)^3]
 - Swap cols within stacks [(3!)^3]

Total symmetries: 2 * (3!)^8 = 3,359,232

## Building and Deploying

I have provided scripts for Windows, OSX, and Linux.

Prerequisites:
 - Ensure you have Python 3 installed.
 - [Install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) (`rustc` and `cargo`)
 - [Install git](https://www.atlassian.com/git/tutorials/install-git)

### Windows: 

```cmd
git clone https://github.com/dclamage/SudokuClassicMinLex.git
cd SudokuClassicMinLex
build.bat
```

Copy `package\sudokux_minlex.pyd` to the same folder as your python project.

### Linux:

```sh
git clone https://github.com/dclamage/SudokuClassicMinLex.git
cd SudokuClassicMinLex
./build-linux.sh
```

Copy `package\sudokux_minlex.so` to the same folder as your python project.


### MacOS:

```sh
git clone https://github.com/dclamage/SudokuClassicMinLex.git
cd SudokuClassicMinLex
./build-osx.sh
```


Copy `package\sudoku_classic_minlex.so` to the same folder as your python project.

## Usage

See [test.py](package/test.py) for an example script.

Basics:

```py
# This import will work as long as sudoku_classic_minlex.pyd (Windows) / sudoku_classic_minlex.so (OSX/Linux) are in the same folder as the script.
import sudoku_classic_minlex

# The sudoku string must be exactly 81 characters long. Any non-numerical digit is treated as a non-given.
sudoku_string = '1..456...4......2...912...6.1...5.....5..729.8..6....43.....9.2....6......82...75'

# Get the exact number of solutions to the puzzle. The second parameter is a maximum number of solutions to return, or 0 for no limit.
count = sudoku_classic_minlex.solution_count(sudoku_string, 0)

# Get a solution to the puzzle. The second parameter is whether the solution should be random (different every time).
# When non-random, the solution is not guaranteed to be any specific solution, but it will be consistent every time
# it is called on the same input.
solved = sudoku_classic_minlex.solve(sudoku_string, False)

# The minlexed output will be a string with '.' for non-givens
minlexed = sudoku_classic_minlex.minlex(sudoku_string)

# minlexed now contains: '........1..2..3.4..5.16.2.....7...84..96.17..7..4.9.....8.9..3..3.....9..94..76..'

```
