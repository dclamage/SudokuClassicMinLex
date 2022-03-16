pub mod board;

use board::Board;
use itertools::Itertools;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn minlex(sudoku_str: &str) -> PyResult<String> {
    let sudoku = sudoku_str.chars().map(parse_digit).collect::<Vec<u8>>();
    if sudoku.len() != 81 {
        return Err(PyErr::new::<PyValueError, _>(
            "Invalid sudoku string. Must be exactly 81 characters long.",
        ));
    }

    // Determine whether rows, columns, or both need to be checked
    let min_row_count = (0..9).map(|row| count_row(&sudoku, row)).min().unwrap();
    let min_col_count = (0..9).map(|col| count_col(&sudoku, col)).min().unwrap();

    let order_perms: Vec<Vec<usize>> = (0..3).permutations(3).collect();
    let mut best_result: String = String::new();
    let mut best_result_row1_stacks_covered = 4;
    let mut best_result_row1_digit_count = 10;

    // Only two rotations are needed because the rest are handled by row/col swapping
    for rot in 0..2 {
        if rot == 0 && min_col_count < min_row_count {
            continue;
        }
        if rot == 1 && min_row_count < min_col_count {
            continue;
        }

        let cur_sudoku = match rot {
            0 => sudoku.clone(),
            1 => remap(&sudoku, |i, j| (j, i)),
            _ => unreachable!(),
        };

        // Swap bands
        for order in &order_perms {
            let mut cur_sudoku = cur_sudoku.clone();
            swap_band_order(&mut cur_sudoku, &order);

            let mut zero_row_index: Option<usize> = None;
            let mut one_row_index: Option<usize> = None;
            for row in 0..9 {
                let row_count = count_row(&cur_sudoku, row);
                if row_count == 0 {
                    zero_row_index = Some(row);
                    break;
                } else if row_count == 1 && one_row_index.is_none() {
                    one_row_index = Some(row);
                }
            }
            if let Some(zero_row_index) = zero_row_index {
                if zero_row_index >= 3 {
                    continue;
                }
            } else if let Some(one_row_index) = one_row_index {
                if one_row_index >= 3 {
                    continue;
                }
            }

            // Swap rows within the first band
            for order in &order_perms {
                let mut cur_sudoku = cur_sudoku.clone();
                swap_row_order(&mut cur_sudoku, &order, &[0, 1, 2]);

                let row123_counts = [
                    count_row(&cur_sudoku, 0),
                    count_row(&cur_sudoku, 1),
                    count_row(&cur_sudoku, 2),
                ];
                if row123_counts[0] > 0 && (row123_counts[1] == 0 || row123_counts[2] == 0) {
                    continue;
                }
                if row123_counts[0] > 1 && (row123_counts[1] == 1 || row123_counts[2] == 1) {
                    continue;
                }

                let row123_stacks_covered = [
                    count_stacks_covered(&cur_sudoku, 0),
                    count_stacks_covered(&cur_sudoku, 1),
                    count_stacks_covered(&cur_sudoku, 2),
                ];
                if row123_stacks_covered[0] > best_result_row1_stacks_covered {
                    continue;
                }
                if row123_stacks_covered[0] == best_result_row1_stacks_covered
                    && row123_counts[0] > best_result_row1_digit_count
                {
                    continue;
                }
                if row123_stacks_covered[1] < row123_stacks_covered[0]
                    || row123_stacks_covered[2] < row123_stacks_covered[0]
                {
                    continue;
                }

                // Swap stacks
                for order in &order_perms {
                    let mut cur_sudoku = cur_sudoku.clone();
                    swap_stack_order(&mut cur_sudoku, &order);

                    let mut skip = false;
                    for row in 0..3 {
                        if row123_stacks_covered[row] > 0 {
                            let row_offset = row * 9;
                            if row123_stacks_covered[row] == 1 {
                                let digit_index = (0..9)
                                    .filter(|col| cur_sudoku[row_offset + col] != 0)
                                    .next()
                                    .unwrap();
                                if digit_index <= 5 {
                                    skip = true;
                                }
                            } else {
                                let digit_count_box1 =
                                    (0..3).filter(|col| cur_sudoku[row_offset + col] != 0).count();
                                let digit_count_box2 =
                                    (3..6).filter(|col| cur_sudoku[row_offset + col] != 0).count();
                                let digit_count_box3 =
                                    (6..9).filter(|col| cur_sudoku[row_offset + col] != 0).count();
                                if digit_count_box1 > 0
                                    && (digit_count_box2 == 0 || digit_count_box3 == 0)
                                    || digit_count_box2 > 0 && digit_count_box3 == 0
                                {
                                    skip = true;
                                }
                            }
                            break;
                        }
                    }
                    if skip {
                        continue;
                    }

                    // Swap rows within the second band
                    for order in &order_perms {
                        let mut cur_sudoku = cur_sudoku.clone();
                        swap_row_order(&mut cur_sudoku, &order, &[3, 4, 5]);

                        let row456_counts = [
                            count_row(&cur_sudoku, 3),
                            count_row(&cur_sudoku, 4),
                            count_row(&cur_sudoku, 5),
                        ];
                        if row456_counts[0] > 0 && (row456_counts[1] == 0 || row456_counts[2] == 0)
                            || row456_counts[1] > 0 && row456_counts[2] == 0
                        {
                            continue;
                        }

                        // Swap rows within the third band
                        for order in &order_perms {
                            let mut cur_sudoku = cur_sudoku.clone();
                            swap_row_order(&mut cur_sudoku, &order, &[6, 7, 8]);

                            let row789_counts = [
                                count_row(&cur_sudoku, 6),
                                count_row(&cur_sudoku, 7),
                                count_row(&cur_sudoku, 8),
                            ];
                            if row789_counts[0] > 0
                                && (row789_counts[1] == 0 || row789_counts[2] == 0)
                                || row789_counts[1] > 0 && row789_counts[2] == 0
                            {
                                continue;
                            }

                            // Swap cols within the first stack
                            for order in &order_perms {
                                let mut cur_sudoku = cur_sudoku.clone();
                                swap_col_order(&mut cur_sudoku, &order, &[0, 1, 2]);

                                // Swap cols within the second stack
                                for order in &order_perms {
                                    let mut cur_sudoku = cur_sudoku.clone();
                                    swap_col_order(&mut cur_sudoku, &order, &[3, 4, 5]);

                                    // Swap cols within the third stack
                                    for order in &order_perms {
                                        let mut cur_sudoku = cur_sudoku.clone();
                                        swap_col_order(&mut cur_sudoku, &order, &[6, 7, 8]);

                                        // Renumber the grid to be in lexicographic order
                                        renumber(&mut cur_sudoku);

                                        let cur_result = to_string(&cur_sudoku);
                                        if best_result.is_empty() || cur_result < best_result {
                                            best_result = cur_result;
                                            best_result_row1_stacks_covered =
                                                row123_stacks_covered[0];
                                            best_result_row1_digit_count = row123_counts[0];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let best_result: String = best_result
        .chars()
        .map(|c| if c == '0' { '.' } else { c })
        .collect();
    Ok(best_result)
}

#[pyfunction]
fn solution_count(sudoku_str: &str, max_solutions: u64) -> u64 {
    let board = Board::from_given_str(sudoku_str);
    if board.is_none() {
        0
    } else {
        board.unwrap().count_solutions(max_solutions)
    }
}

#[pyfunction]
fn singles_depth(sudoku_str: &str) -> i64 {
    let board = Board::from_given_str(sudoku_str);
    if board.is_none() {
        -1
    } else {
        let board = board.unwrap();
        let count = board.count_solutions(2);
        if count != 1 {
            -1
        } else {
            let result = board.singles_depth_required();
            if result.is_none() {
                -1
            } else {
                result.unwrap() as i64
            }
        }
    }
}

#[pyfunction]
fn solve(sudoku_str: &str, random: bool) -> String {
    let board = Board::from_given_str(sudoku_str);
    if board.is_none() {
        String::new()
    } else {
        let board = board.unwrap();
        let solved = if random {
            board.solve_random()
        } else {
            board.solve()
        };

        if solved.is_none() {
            String::new()
        } else {
            solved.unwrap().to_string()
        }
    }
}

fn parse_digit(c: char) -> u8 {
    let c = c as u8;
    const ZERO: u8 = '0' as u8;
    const NINE: u8 = '9' as u8;
    if c >= ZERO && c <= NINE {
        c - ZERO
    } else {
        0
    }
}

fn to_string(sudoku: &Vec<u8>) -> String {
    let mut result = String::with_capacity(81);
    for i in 0..81 {
        result.push((sudoku[i] + '0' as u8) as char);
    }
    result
}

fn swap_rows(sudoku: &mut Vec<u8>, row1: usize, row2: usize) {
    let row1_off = row1 * 9;
    let row2_off = row2 * 9;
    for i in 0..9 {
        sudoku.swap(row1_off + i, row2_off + i);
    }
}

fn swap_cols(sudoku: &mut Vec<u8>, col1: usize, col2: usize) {
    for i in 0..9 {
        let row_off = i * 9;
        sudoku.swap(row_off + col1, row_off + col2);
    }
}

fn swap_row_order(sudoku: &mut Vec<u8>, order: &[usize], rows: &[usize]) {
    let mut order = order.to_vec();
    if order[0] > order[1] {
        swap_rows(sudoku, rows[0], rows[1]);
        order.swap(0, 1);
    }
    if order[1] > order[2] {
        swap_rows(sudoku, rows[1], rows[2]);
        order.swap(1, 2);
    }
    if order[0] > order[1] {
        swap_rows(sudoku, rows[0], rows[1]);
        order.swap(0, 1);
    }
}

fn swap_col_order(sudoku: &mut Vec<u8>, order: &[usize], cols: &[usize]) {
    let mut order = order.to_vec();
    if order[0] > order[1] {
        swap_cols(sudoku, cols[0], cols[1]);
        order.swap(0, 1);
    }
    if order[1] > order[2] {
        swap_cols(sudoku, cols[1], cols[2]);
        order.swap(1, 2);
    }
    if order[0] > order[1] {
        swap_cols(sudoku, cols[0], cols[1]);
        order.swap(0, 1);
    }
}

fn swap_band_order(sudoku: &mut Vec<u8>, order: &[usize]) {
    let mut order = order.to_vec();
    if order[0] > order[1] {
        swap_rows(sudoku, 0, 3);
        swap_rows(sudoku, 1, 4);
        swap_rows(sudoku, 2, 5);
        order.swap(0, 1);
    }
    if order[1] > order[2] {
        swap_rows(sudoku, 3, 6);
        swap_rows(sudoku, 4, 7);
        swap_rows(sudoku, 5, 8);
        order.swap(1, 2);
    }
    if order[0] > order[1] {
        swap_rows(sudoku, 0, 3);
        swap_rows(sudoku, 1, 4);
        swap_rows(sudoku, 2, 5);
        order.swap(0, 1);
    }
}

fn swap_stack_order(sudoku: &mut Vec<u8>, order: &[usize]) {
    let mut order = order.to_vec();
    if order[0] > order[1] {
        swap_cols(sudoku, 0, 3);
        swap_cols(sudoku, 1, 4);
        swap_cols(sudoku, 2, 5);
        order.swap(0, 1);
    }
    if order[1] > order[2] {
        swap_cols(sudoku, 3, 6);
        swap_cols(sudoku, 4, 7);
        swap_cols(sudoku, 5, 8);
        order.swap(1, 2);
    }
    if order[0] > order[1] {
        swap_cols(sudoku, 0, 3);
        swap_cols(sudoku, 1, 4);
        swap_cols(sudoku, 2, 5);
        order.swap(0, 1);
    }
}

fn remap(sudoku: &Vec<u8>, rf: fn(usize, usize) -> (usize, usize)) -> Vec<u8> {
    let mut new_sudoku = Vec::with_capacity(81);
    for i in 0..9 {
        for j in 0..9 {
            let (i1, j1) = rf(i, j);
            new_sudoku.push(sudoku[i1 * 9 + j1]);
        }
    }
    new_sudoku
}

fn renumber(sudoku: &mut Vec<u8>) {
    let mut number_map = [0u8; 9];
    let mut next_number = 1;
    for i in 0..81 {
        let cur_val = sudoku[i] as usize;
        if cur_val != 0 {
            if number_map[cur_val - 1] == 0 {
                number_map[cur_val - 1] = next_number;
                next_number += 1;
            }
            sudoku[i] = number_map[cur_val - 1];
        }
    }
}

fn count_row(sudoku: &Vec<u8>, row: usize) -> usize {
    let row_offset = row * 9;
    let mut count = 0;
    for i in 0..9 {
        if sudoku[row_offset + i] != 0 {
            count += 1;
        }
    }
    count
}

fn count_col(sudoku: &Vec<u8>, col: usize) -> usize {
    let mut count = 0;
    for i in 0..9 {
        if sudoku[i * 9 + col] != 0 {
            count += 1;
        }
    }
    count
}

fn count_stacks_covered(sudoku: &Vec<u8>, row: usize) -> usize {
    let mut have_stack = [false; 3];
    let mut num_stacks = 0;
    for i in 0..9 {
        let stack_index = i / 3;
        if sudoku[row * 9 + i] != 0 && !have_stack[stack_index] {
            have_stack[stack_index] = true;
            num_stacks += 1;
        }
    }
    num_stacks
}

/// Implements the Python module pip, registers the class Engine
#[pymodule]
fn sudoku_classic_minlex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(minlex, m)?)?;
    m.add_function(wrap_pyfunction!(solution_count, m)?)?;
    m.add_function(wrap_pyfunction!(solve, m)?)?;
    m.add_function(wrap_pyfunction!(singles_depth, m)?)?;

    Ok(())
}
