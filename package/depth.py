import sys
import time
# This import will work as long as sudoku_classic_minlex.pyd (Windows) / sudoku_classic_minlex.so (OSX/Linux) are in the same folder as the script.
import sudoku_classic_minlex

if len(sys.argv) < 2:
	print("Usage: python3 depth.py sudoku_string")
	exit(1)

for sudoku_string in sys.argv[1:]:
	# Time how long this takes
	start_time = time.time()
	depth = sudoku_classic_minlex.singles_depth(sudoku_string)
	time_elapsed = time.time() - start_time
	time_elapsed_ms = time_elapsed * 1000.0
	print("%s: %d (%.2f ms)" % (sudoku_string, depth, time_elapsed_ms))
