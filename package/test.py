import time
import sudoku_classic_minlex

num_passed = 0
num_failed = 0
num_better = 0
start_time = time.time()
with open("cb401-minlex.txt") as file:
	# Get the number of lines in the file
	num_lines = sum(1 for line in file)

	# Reset the file
	file.seek(0)

	cur_line = 0
	for line in file:
		cur_line += 1

		if cur_line % 100 == 0:
			time_elapsed = time.time() - start_time
			time_elapsed_ms = time_elapsed * 1000.0
			time_left = (num_lines - cur_line) * (time_elapsed / cur_line)
			print("%d/%d (%d%%) [BETTER %d/%d] [FAILED %d/%d] %d:%02d:%02d left (%.2fms per puzzle)" % (cur_line, num_lines, cur_line * 100 / num_lines, num_better, cur_line, num_failed, cur_line, time_left // 3600, (time_left % 3600) // 60, time_left % 60, time_elapsed_ms / cur_line))

		# Strip any whitespace
		line = line.strip()
		
		# Split line by tab character
		line = line.split("\t")
		
		input_puzzle = line[0].strip()
		expected_output = line[1].strip()
		minlexed = sudoku_classic_minlex.minlex(input_puzzle)

		if minlexed == expected_output:
			num_passed += 1
		elif minlexed.replace('.', '0') < expected_output.replace('.', '0'):
			num_better += 1
		else:
			num_failed += 1
			print("FAIL LINE %d: %s %s (%s)" % (cur_line, input_puzzle, minlexed, expected_output))
end_time = time.time()
elapsed_time_sec = end_time - start_time
elapsed_time_ms = elapsed_time_sec * 1000.0
num_tests = num_passed + num_better + num_failed
print("%d tests passed, %d tests better, %d tests failed, %fms per puzzle" % (num_passed, num_better, num_failed, elapsed_time_ms / num_tests))
