# This import will work as long as sudoku_classic_minlex.pyd (Windows) / sudoku_classic_minlex.so (OSX/Linux) are in the same folder as the script.
import sudoku_classic_minlex

# The sudoku string must be exactly 81 characters long. Any non-numerical digit is treated as a non-given.
sudoku_string = '1..456...4......2...912...6.1...5.....5..729.8..6....43.....9.2....6......82...75'

# Get the exact number of solutions to the puzzle. The second parameter is a maximum number of solutions to return, or 0 for no limit.
count = sudoku_classic_minlex.solution_count(sudoku_string, 0)
print("The puzzle has " + str(count) + " solutions.")

# Get a solution to the puzzle. The second parameter is whether the solution should be random (different every time).
# When non-random, the solution is not guaranteed to be any specific solution, but it will be consistent every time
# it is called on the same input.
solved = sudoku_classic_minlex.solve(sudoku_string, False)
print("The solution is: %s" % solved)

# The minlexed output will be a string with '.' for non-givens
minlexed = sudoku_classic_minlex.minlex(sudoku_string)
print ("The minlexed puzzle is: %s" % minlexed)

# minlexed now contains: '........1..2..3.4..5.16.2.....7...84..96.17..7..4.9.....8.9..3..3.....9..94..76..'