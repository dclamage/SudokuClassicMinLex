use rand::Rng;

const VALUE_SET: u32 = 1u32 << 31;
const ALL_VALUES: u32 = 0x1ff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicResult {
    None,
    Changed,
    Invalid,
    Solved,
}

pub type Givens = [u8; 81];

fn value_mask(v: u32) -> u32 {
    1u32 << (v - 1)
}

fn value_count(v: u32) -> usize {
    (v & ALL_VALUES).count_ones() as usize
}

fn get_value(v: u32) -> u32 {
    (v & ALL_VALUES).trailing_zeros() + 1
}

fn is_value_set(v: u32) -> bool {
    (v & VALUE_SET) != 0
}

pub fn given_string(givens: &Givens) -> String {
    let mut s = String::new();
    s.reserve(81);
    for i in 0..81 {
        if givens[i] == 0 {
            s.push('.');
        } else {
            assert!(givens[i] <= 9);
            s.push((givens[i] + b'0') as char);
        }
    }
    s
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<u32>,
    singles: Vec<usize>,
    num_set_cells: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![ALL_VALUES; 81],
            singles: Vec::new(),
            num_set_cells: 0,
        }
    }

    pub fn from_givens(givens: &[u8]) -> Option<Board> {
        let mut board = Self::new();
        for i in 0..givens.len() {
            let value = givens[i];
            if value != 0 && !board.set_value(i, value as u32) {
                return None;
            }
        }
        board.recreate_singles();

        Some(board)
    }

    pub fn from_given_str(given_str: &str) -> Option<Board> {
        let mut board = Self::new();
        for (i, c) in given_str.chars().enumerate() {
            if c <= '0' || c > '9' {
                continue;
            }

            let value = c as u8 - b'0';
            if !board.set_value(i, value as u32) {
                return None;
            }
        }
        board.recreate_singles();

        Some(board)
    }

    pub fn count_solutions(&self, max_solutions: u64) -> u64 {
        let mut solutions = 0;

        let mut board_stack: Vec<Self> = Vec::with_capacity(81);
        board_stack.push(self.clone());

        while let Some(mut board) = board_stack.pop() {
            

            let result = board.set_singles();
            if result == LogicResult::Solved {
                solutions += 1;
                if max_solutions > 0 && solutions >= max_solutions {
                    return solutions;
                }
                continue;
            }
            if result == LogicResult::Invalid {
                continue;
            }

            let i = board.best_index();
            if i.is_none() {
                solutions += 1;
                if max_solutions > 0 && solutions >= max_solutions {
                    return solutions;
                }
                continue;
            }

            let i = i.unwrap();
            if board.cells[i] == 0 {
                continue;
            }

            let value = get_value(board.cells[i]);
            assert!(value > 0 && value <= 9);

            let mut backup = board.clone();
            backup.cells[i] &= !value_mask(value);
            if backup.cells[i] != 0 {
                if value_count(backup.cells[i]) == 1 {
                    backup.singles.push(i);
                }
                board_stack.push(backup);
            }

            if board.set_value(i, value) {
                board_stack.push(board);
            }
        }

        solutions
    }


    pub fn solve(&self) -> Option<Board> {
        let mut board_stack: Vec<Self> = Vec::with_capacity(81);
        board_stack.push(self.clone());

        while let Some(mut board) = board_stack.pop() {
            

            let result = board.set_singles();
            if result == LogicResult::Solved {
                return Some(board);
            }
            if result == LogicResult::Invalid {
                continue;
            }

            let i = board.best_index();
            if i.is_none() {
                return Some(board);
            }

            let i = i.unwrap();
            if board.cells[i] == 0 {
                continue;
            }

            let value = get_value(board.cells[i]);
            assert!(value > 0 && value <= 9);

            let mut backup = board.clone();
            backup.cells[i] &= !value_mask(value);
            if backup.cells[i] != 0 {
                if value_count(backup.cells[i]) == 1 {
                    backup.singles.push(i);
                }
                board_stack.push(backup);
            }

            if board.set_value(i, value) {
                board_stack.push(board);
            }
        }

        None
    }

    pub fn solve_random(&self) -> Option<Board> {
        let mut board_stack: Vec<Self> = Vec::with_capacity(81);
        board_stack.push(self.clone());

        while let Some(mut board) = board_stack.pop() {
            

            let result = board.set_singles();
            if result == LogicResult::Solved {
                return Some(board);
            }
            if result == LogicResult::Invalid {
                continue;
            }

            let i = board.best_index();
            if i.is_none() {
                return Some(board);
            }

            let i = i.unwrap();
            if board.cells[i] == 0 {
                continue;
            }

            let value = board.random_value(i);
            assert!(value > 0 && value <= 9);

            let mut backup = board.clone();
            backup.cells[i] &= !value_mask(value);
            if backup.cells[i] != 0 {
                if value_count(backup.cells[i]) == 1 {
                    backup.singles.push(i);
                }
                board_stack.push(backup);
            }

            if board.set_value(i, value) {
                board_stack.push(board);
            }
        }

        None
    }

    fn eliminate_contradiction(&mut self, max_depth: usize) -> LogicResult {
        if max_depth == 0 {
            return LogicResult::None;
        }

        for i in 0..self.cells.len() {
            let cell_mask = self.cells[i];
            if cell_mask == 0 {
                return LogicResult::Invalid;
            }

            if is_value_set(cell_mask) {
                continue;
            }
            
            for value in 1..10 {
                let value_mask = value_mask(value);
                if (cell_mask & value_mask) == 0 {
                    continue;
                }

                let mut test_board = self.clone();
                if !test_board.set_value(i, value) {
                    if !self.clear_value(i, value_mask) {
                        return LogicResult::Invalid;
                    }
                    return LogicResult::Changed;
                }

                let test_singles_result = test_board.eliminate_singles_and_contradictions(max_depth - 1);
                if test_singles_result == LogicResult::Invalid {
                    if !self.clear_value(i, value_mask) {
                        return LogicResult::Invalid;
                    }
                    return LogicResult::Changed;
                }
            }
        }

        LogicResult::None
    }

    fn eliminate_singles_and_contradictions(&mut self, max_depth: usize) -> LogicResult {
        let mut result = LogicResult::None;
        loop {
            let singles_result = self.set_singles();
            if singles_result == LogicResult::Solved {
                return singles_result;
            }
            if singles_result == LogicResult::Invalid {
                return singles_result;
            }
            if singles_result == LogicResult::Changed {
                result = LogicResult::Changed;
            }

            let con_result = self.eliminate_contradiction(max_depth);
            match con_result {
                LogicResult::Solved => return con_result,
                LogicResult::Invalid => return con_result,
                LogicResult::Changed => result = LogicResult::Changed,
                LogicResult::None => return result,
            }
        }
    }

    pub fn singles_depth_required(&self) -> Option<usize> {
        let mut board = self.clone();
        let mut cur_depth = 0;
        loop {
            let result = board.eliminate_singles_and_contradictions(cur_depth);
            if result == LogicResult::Solved {
                return Some(cur_depth);
            }
            if result == LogicResult::Invalid {
                return None;
            }
            cur_depth += 1;
        }
    }

    fn random_value(&self, i: usize) -> u32 {
        let mask = self.cells[i] & ALL_VALUES;
        if mask == 0 {
            panic!("Cell {} has no possible values", i);
        }
        let count = value_count(mask);
        if count == 1 {
            return get_value(mask);
        }

        let mut value_index: usize = rand::rng().random_range(0..count);
        for value in 1..=9 {
            if mask & value_mask(value) != 0 {
                if value_index == 0 {
                    return value;
                }
                value_index -= 1;
            }
        }

        panic!("Cell {} could not get random value", i);
    }

    pub fn num_givens(&self) -> usize {
        self.cells.iter().filter(|&v| v & VALUE_SET != 0).count()
    }

    pub fn givens(&self) -> Givens {
        let mut givens: Givens = [0; 81];
        for i in 0..81 {
            if self.cells[i] & VALUE_SET != 0 {
                givens[i] = get_value(self.cells[i]) as u8;
                assert!(givens[i] <= 9);
            }
        }

        givens
    }

    pub fn given_str(&self) -> String {
        given_string(&self.givens())
    }

    fn update_house(&mut self, house_index: usize, value_mask: u32) -> bool {
        for i in 0..9 {
            let cell_index = HOUSES[house_index][i];
            if !self.clear_value(cell_index, value_mask) {
                return false;
            }
        }

        true
    }

    fn clear_value(&mut self, cell_index: usize, value_mask: u32) -> bool {
        let mut cell_value = self.cells[cell_index];
        if cell_value & VALUE_SET == 0 && cell_value & value_mask != 0 {
            cell_value &= !value_mask;
            if cell_value == 0 {
                return false;
            }

            if value_count(cell_value) == 1 {
                self.singles.push(cell_index);
            }

            self.cells[cell_index] = cell_value;
        }

        true
    }

    pub fn set_value(&mut self, index: usize, value: u32) -> bool {
        assert!(self.cells[index] & VALUE_SET == 0);
        assert!(value > 0 && value <= 9);

        let value_mask = value_mask(value);
        if (self.cells[index] & value_mask) == 0 {
            return false;
        }

        self.cells[index] = VALUE_SET | value_mask;
        self.num_set_cells += 1;

        let (r, c, b) = HOUSE_LOOKUP[index];
        if !self.update_house(r, value_mask) {
            return false;
        }
        if !self.update_house(c, value_mask) {
            return false;
        }
        if !self.update_house(b, value_mask) {
            return false;
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..81 {
            if self.cells[i] & VALUE_SET == 0 {
                return false;
            }
        }

        true
    }

    pub fn set_singles(&mut self) -> LogicResult {
        let mut changed = false;
        loop {
            let naked_result = self.set_naked_singles();
            if naked_result == LogicResult::Invalid || naked_result == LogicResult::Solved {
                return naked_result;
            }
            changed |= naked_result == LogicResult::Changed;

            let hidden_result = self.set_hidden_singles();
            if hidden_result == LogicResult::Invalid {
                return hidden_result;
            }
            changed |= hidden_result == LogicResult::Changed;
            if hidden_result == LogicResult::None {
                break;
            }
        }

        if changed {
            LogicResult::Changed
        } else {
            LogicResult::None
        }
    }

    pub fn set_naked_singles(&mut self) -> LogicResult {
        let mut changed = false;
        while let Some(index) = self.singles.pop() {
            

            assert!(self.cells[index] & VALUE_SET == 0);
            assert!(self.cells[index] != 0);
            assert!(value_count(self.cells[index]) == 1);

            let value = get_value(self.cells[index]);
            if !self.set_value(index, value) {
                return LogicResult::Invalid;
            }
            changed = true;
        }

        if self.num_set_cells == 81 {
            return LogicResult::Solved;
        }

        if changed {
            LogicResult::Changed
        } else {
            LogicResult::None
        }
    }

    pub fn set_hidden_singles(&mut self) -> LogicResult {
        let mut changed = false;
        for house in HOUSES {
            let mut at_least_once: u32 = 0;
            let mut more_than_once: u32 = 0;
            let mut set_mask = 0;
            for i in house {
                let mask = self.cells[i];
                if is_value_set(mask) || value_count(mask) == 1 {
                    set_mask |= mask;
                } else {
                    more_than_once |= at_least_once & mask;
                    at_least_once |= mask;
                }
            }
            more_than_once &= !set_mask;
            at_least_once &= !set_mask;
            set_mask &= !VALUE_SET;

            if at_least_once | set_mask != ALL_VALUES {
                return LogicResult::Invalid;
            }

            let exactly_once = at_least_once & !more_than_once;
            if exactly_once != 0 {
                for i in house {
                    let cell_mask = self.cells[i];
                    if is_value_set(cell_mask) || value_count(cell_mask) == 1 {
                        continue;
                    }

                    let mask = exactly_once & cell_mask;
                    if value_count(mask) > 1 {
                        return LogicResult::Invalid;
                    }

                    if mask != 0 {
                        if !self.set_value(i, get_value(mask)) {
                            return LogicResult::Invalid;
                        }
                        changed = true;
                    }
                }
            }
        }

        if changed {
            LogicResult::Changed
        } else {
            LogicResult::None
        }
    }

    fn best_index(&self) -> Option<usize> {
        let mut best_index = None;
        let mut best_count = 10;
        for i in 0..81 {
            if self.cells[i] & VALUE_SET != 0 {
                continue;
            }
            if self.cells[i] == 0 {
                return Some(i);
            }

            let count = value_count(self.cells[i]);
            if count <= 2 {
                return Some(i);
            }

            if count < best_count {
                best_index = Some(i);
                best_count = count;
            }
        }

        best_index
    }

    fn recreate_singles(&mut self) {
        self.singles.clear();
        for i in 0..81 {
            if self.cells[i] & VALUE_SET != 0 {
                continue;
            }
            if value_count(self.cells[i]) == 1 {
                self.singles.push(i);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut chars = ['.'; 81];
        for i in 0..81 {
            if is_value_set(self.cells[i]) {
                chars[i] = (get_value(self.cells[i]) as u8 + b'0') as char;
            }
        }

        chars.iter().collect()
    }
}

const HOUSES: [[usize; 9]; 27] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

const HOUSE_LOOKUP: [(usize, usize, usize); 81] = construct_house_lookup();

const fn construct_house_lookup() -> [(usize, usize, usize); 81] {
    let lookup = [(0, 0, 0); 81];
    fill_house(0, lookup)
}

const fn fill_house(
    i: usize,
    mut lookup: [(usize, usize, usize); 81],
) -> [(usize, usize, usize); 81] {
    let r = i / 9;
    let c = i % 9;
    let b = (r / 3) * 3 + c / 3;
    lookup[i] = (r, 9 + c, 18 + b);

    if i < 80 {
        fill_house(i + 1, lookup)
    } else {
        lookup
    }
}
