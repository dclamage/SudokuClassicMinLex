# SudokuX MinLexer

*Created by David Clamage (Rangsk)*

This Rust library is intended to hook into Python 3 and allow for converting a Classic Sudoku puzzle into its minimal lexicographical form. This is useful for keeping a database of known puzzles in canonical form without repeating symmetrically identical ones.

Solver functionality is also provided.

## Minimal Lexicographical Form

This code is independently written based on the following symmetries of a classic sudoku. These symmetries are described in the [Mathematics of Sudoku](https://en.wikipedia.org/wiki/Mathematics_of_Sudoku) wikipedia article without citation.

 - Rotate 90 degrees [2]
 - Swap bands [3!]
 - Swap stacks [3!]
 - Swap rows within bands [(3!)^3]
 - Swap cols within stacks [(3!)^3]

Total symmetries: 2 * (3!)^8 = 3,359,232

Some optimizations are performed, but the goal is to produce a true minimal lexicograhical form of the Sudoku. As such, this process is significantly slower than other popular tools.

Glenn S. Fowler's [sudoku solver/generator](http://gsf.cococlyde.org/download/sudoku) includes a much faster "minlex" tool based on contributions by Michael Deverin ("holdout"), as described at this [forum post](http://forum.enjoysudoku.com/minlex-form-min-and-max-lists-chaining-t30325.html). Experimentation with this tool has found that though it produces consistent results that are suitable for classifying two puzzles as the same, about 20% of the time these results are not actually the minimally lexicographical version of the puzzle.

## Singles Depth

The `singles_depth` functionality in this codebase was also independently written and was a natural extension to the "simple contradiction" functionality in my main [C# Sudoku Solver](https://github.com/dclamage/SudokuSolver).

This function only works on puzzles with unique solutions, and limits the recursion depth of the brute-force solver to 0, then 1, then 2, and so on until the puzzle is solved. Only "naked" and "hidden" singles are checked inbetween recursive contradiction steps. The result of this process is the minimal brute force recursive depth of the puzzle.

This code reproduces the results of the T&E(singles,n) process as defined formally by [Denis Berthier](https://www.researchgate.net/project/Pattern-Based-Constraint-Satisfaction) in Constraint Resolution Theories (2011). The T&E depth metric has been found to be useful in searching for the "hardest" puzzles. Until 2022, it was believed that the maximum depth required for a 9x9 classic sudoku was 2, as conjectured by Berthier; however, counterexamples have now been found, with the first example discovered by [Philip Newman](https://github.com/newman-iknewit) and [identified as not solvable by T&E(singles,2) by Berthier](http://forum.enjoysudoku.com/the-hardest-sudokus-new-thread-t6539-1035.html#p317678).

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
