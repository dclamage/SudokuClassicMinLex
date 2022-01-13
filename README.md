# SudokuX MinLexer

*Created by David Clamage (Rangsk)*

This Rust library is intended to hook into Python 3 and allow for converting a Sudoku X puzzle into its minimal lexicographical form. This is useful for keeping a database of known puzzles in canonical form without repeating symmetrically identical ones.

Solver functionality is also provided.

The algorithm used is [described here](http://www.sudocue.net/minx.php):  
1. All symmetrically identical versions of the puzzle are generated
2. Puzzles are renumbered such that the smallest numbers appear as early as possible.
3. These puzzles are converted to string.
4. The lexicographically smallest string is chosen.

Sudoku-X symmetries:   
 - Rotations/reflections (8)
 - Rows 4 & 6 and Cols 4 & 6 can be swapped (2)
 - R19C19, R28C28, and R37C37 can be permuted (3! = 6).

## Building and Deploying

I have provided scripts for Windows, OSX, and Linux.

Prerequisites:
 - Ensure you have Python 3 installed.
 - [Install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) (`rustc` and `cargo`)
 - [Install git](https://www.atlassian.com/git/tutorials/install-git)

### Windows: 

```cmd
git clone https://github.com/dclamage/SudokuXMinLex.git
cd SudokuXMinLex
build.bat
```

Copy `package\sudokux_minlex.pyd` to the same folder as your python project.

### Linux:

```sh
git clone https://github.com/dclamage/SudokuXMinLex.git
cd SudokuXMinLex
./build-linux.sh
```

Copy `package\sudokux_minlex.so` to the same folder as your python project.


### MacOS:

```sh
git clone https://github.com/dclamage/SudokuXMinLex.git
cd SudokuXMinLex
./build-osx.sh
```


Copy `package\sudokux_minlex.so` to the same folder as your python project.

## Usage

See [test.py](package/test.py) for an example script.

Basics:

```py
# This import will work as long as sudokux_minlex.pyd (Windows) / sudokux_minlex.so (OSX/Linux) are in the same folder as the script.
import sudokux_minlex

# The sudoku string must be exactly 81 characters long. Any non-numerical digit is treated as a non-given.
sudoku_string = '..............1....3....4...2.....................5......34....1.6....7....8.....'

# Get the exact number of solutions to the puzzle. The second parameter is a maximum number of solutions to return, or 0 for no limit.
count = sudokux_minlex.solution_count(sudoku_string, 0)

# Get a solution to the puzzle. The second parameter is whether the solution should be random (different every time).
# When non-random, the solution is not guaranteed to be any specific solution, but it will be consistent every time
# it is called on the same input.
solved = sudokux_minlex.solve(sudoku_string, False)

# The minlexed output will be a string with '.' for non-givens
minlexed = sudokux_minlex.minlex(sudoku_string)

# minlexed now contains: '................1...2.........3..4...1.......56............7.6...8........4......'

```
