use itertools::iproduct;
use itertools::Itertools;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt;
use std::slice::Iter;

const EMPTY_CHAR: char = '_';
const LOWERCASE_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid {
    chars: Vec<Vec<char>>,
}

impl Grid {
    pub fn empty(row_count: usize, col_count: usize) -> Grid {
        Grid::new(&vec![vec![EMPTY_CHAR; col_count]; row_count])
    }

    pub fn new(chars: &[Vec<char>]) -> Grid {
        Grid {
            chars: chars.to_owned(),
        }
    }

    pub fn one_word_grid(&self, word: &WordLocation) -> Grid {
        let mut grid = Grid::new(&vec![vec![EMPTY_CHAR; self.col_count()]; self.row_count()]);
        let mut cell = word.start_cell.to_owned();

        let val = self.value_at_cell(&cell);
        grid.set_value_at_cell(&cell, val);

        while let Some(next_cell) = self.next_cell_in_direction(&cell, &word.direction) {
            let val = self.value_at_cell(&cell);
            grid.set_value_at_cell(&cell, val);

            if next_cell == word.end_cell {
                break;
            }
            cell = next_cell
        }

        let last_val = self.value_at_cell(&word.end_cell);
        grid.set_value_at_cell(&word.end_cell, last_val);

        grid
    }

    pub fn rows(&self) -> &Vec<Vec<char>> {
        &self.chars
    }

    pub fn row_count(&self) -> usize {
        self.chars.len()
    }

    pub fn col_count(&self) -> usize {
        if self.chars.is_empty() {
            0
        } else {
            self.chars[0].len()
        }
    }

    // Will clobber any existing chars
    // Panics if word does not fit
    fn add_word_at_location(&mut self, wl: &WordLocation) {
        if self.cells_remaining_in_direction(&wl.start_cell, &wl.direction) + 1 < wl.word.len() {
            panic!("Not enough cells remain to place word: {}", wl.word);
        }

        let mut cell = wl.start_cell;
        for (i, c) in wl.word.chars().enumerate() {
            self.set_value_at_cell(&cell, c);

            if i == wl.word.len() - 1 {
                return;
            }

            cell = self.next_cell_in_direction(&cell, &wl.direction).unwrap(); // We have already validated that there are enough cells to place the word.
        }
    }

    fn set_value_at_cell(&mut self, cell: &Cell, val: char) {
        self.chars[cell.row][cell.col] = val;
    }

    fn fill_empty_cells_with_chars(&mut self, rng: &mut ThreadRng) {
        for r in 0..self.chars.len() {
            for c in 0..self.chars[r].len() {
                if self.chars[r][c] == EMPTY_CHAR {
                    self.chars[r][c] = random_char(rng);
                }
            }
        }
    }

    fn value_at_cell(&self, cell: &Cell) -> char {
        // TODO: bounds check
        self.chars[cell.row][cell.col]
    }

    fn next_cell_in_direction(&self, cell: &Cell, direction: &Direction) -> Option<Cell> {
        let (row, col) = match direction {
            Direction::Up => (cell.row as isize - 1, cell.col as isize),
            Direction::UpRight => (cell.row as isize - 1, cell.col as isize + 1),
            Direction::Right => (cell.row as isize, cell.col as isize + 1),
            Direction::DownRight => (cell.row as isize + 1, cell.col as isize + 1),
            Direction::Down => (cell.row as isize + 1, cell.col as isize),
            Direction::DownLeft => (cell.row as isize + 1, cell.col as isize - 1),
            Direction::Left => (cell.row as isize, cell.col as isize - 1),
            Direction::UpLeft => (cell.row as isize - 1, cell.col as isize - 1),
        };

        if row < 0
            || col < 0
            || row as usize >= self.row_count()
            || col as usize >= self.col_count()
        {
            return None;
        }

        Some(Cell {
            row: row as usize,
            col: col as usize,
        })
    }

    fn cells_remaining_in_direction(&self, cell: &Cell, direction: &Direction) -> usize {
        let rows_remaining = match direction {
            Direction::UpLeft | Direction::Up | Direction::UpRight => cell.row,
            Direction::DownLeft | Direction::Down | Direction::DownRight => {
                self.row_count() - cell.row - 1
            }
            Direction::Left | Direction::Right => usize::MAX,
        };

        let cols_remaining = match direction {
            Direction::UpLeft | Direction::Left | Direction::DownLeft => cell.col,
            Direction::UpRight | Direction::Right | Direction::DownRight => {
                self.col_count() - cell.col - 1
            }
            Direction::Up | Direction::Down => usize::MAX,
        };

        min(rows_remaining, cols_remaining)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.chars {
            writeln!(
                f,
                "{}",
                row.clone().into_iter().intersperse(' ').collect::<String>()
            )?
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WordLocation {
    pub word: String,
    pub start_cell: Cell,
    pub end_cell: Cell,
    pub direction: Direction,
}

impl WordLocation {
    fn into_reversed_location(self) -> WordLocation {
        WordLocation {
            word: self.word.chars().rev().collect::<String>(),
            start_cell: self.end_cell,
            end_cell: self.start_cell,
            direction: opposite_direction(&self.direction),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ];
        DIRECTIONS.iter()
    }
}

pub fn generate_grid(rows: usize, cols: usize, words: &[&str]) -> Option<Grid> {
    let mut rng = rand::thread_rng();

    if words.is_empty() {
        return None;
    }

    let mut word_list = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();

    // sort word list by longest words first to fit faster.
    word_list.sort_by_key(|a| a.len());

    // reverse word list so we can push/pop easily and yet still preserve initial ordering
    word_list.reverse();

    let cells = iproduct!(0..rows, 0..cols)
        .map(|(r, c)| Cell { row: r, col: c })
        .collect::<Vec<_>>();

    let directions = Direction::iterator()
        .map(|d| d.to_owned())
        .collect::<Vec<_>>();

    struct StackEntry {
        grid: Grid,
        word: String,
        remaining_possible_directions: Vec<Direction>,
        remaining_possible_cells: Vec<Cell>,
    }

    impl StackEntry {
        /// Shuffles cells and directions with provided ThreadRng
        fn new_with_shuffle(
            grid: Grid,
            word: &str,
            cells: &[Cell],
            directions: &[Direction],
            rng: &mut ThreadRng,
        ) -> Self {
            let mut ps = cells.to_vec();
            ps.shuffle(rng);
            let mut ds = directions.to_vec();
            ds.shuffle(rng);

            StackEntry {
                grid,
                word: word.to_owned(),
                remaining_possible_directions: ds,
                remaining_possible_cells: ps,
            }
        }
    }

    let mut stack = vec![StackEntry::new_with_shuffle(
        Grid::empty(rows, cols),
        &word_list.pop().unwrap(), // We know there is at least one word.
        cells.as_slice(),
        directions.as_slice(),
        &mut rng,
    )];

    loop {
        let mut current = match stack.last_mut() {
            Some(c) => c,
            None => return None,
        };

        // Get the next direction to try
        let direction = match current.remaining_possible_directions.pop() {
            Some(d) => d,
            None => {
                // If we've tried all the possible directions at this position,
                // pop the current position off and reset the list of directions,
                current.remaining_possible_cells.pop();

                let mut ds = directions.clone();
                ds.shuffle(&mut rng);
                current.remaining_possible_directions = ds;
                current.remaining_possible_directions.pop().unwrap() // we just refreshed the list so we know we can pop.
            }
        };

        // Get the position in the grid that we're trying the word against
        match current.remaining_possible_cells.last() {
            Some(p) => {
                if let Some(mut grid) =
                    place_word_at_cell(&current.grid, &p, &direction, &current.word)
                {
                    if let Some(w) = word_list.pop() {
                        stack.push(StackEntry::new_with_shuffle(
                            grid,
                            &w,
                            cells.as_slice(),
                            directions.as_slice(),
                            &mut rng,
                        ));
                    } else {
                        grid.fill_empty_cells_with_chars(&mut rng);
                        return Some(grid);
                    }
                } else {
                    // TODO: explain why we don't care about this
                }
            }
            None => {
                // If there are no more available positions,
                // put the current word back in the vocab list and backtrack by popping the stack.
                word_list.push(current.word.clone());
                stack.pop();
            }
        };
    }
}

fn place_word_at_cell(
    grid: &Grid,
    start_cell: &Cell,
    direction: &Direction,
    word: &str,
) -> Option<Grid> {
    // +1 to account for the current cell.
    if grid.cells_remaining_in_direction(&start_cell, direction) + 1 < word.len() {
        return None;
    }

    let mut cell: Cell = start_cell.to_owned();

    for (i, c) in word.chars().enumerate() {
        let char_at_cell = grid.value_at_cell(&cell);

        if char_at_cell != EMPTY_CHAR && char_at_cell != c {
            return None;
        }

        let remaining_chars = word.len() - i;
        if remaining_chars > 1 {
            cell = grid.next_cell_in_direction(&cell, direction)?;
        }
    }

    let mut g = grid.clone();

    g.add_word_at_location(&WordLocation {
        word: word.to_owned(),
        start_cell: *start_cell,
        end_cell: cell,
        direction: *direction,
    });

    Some(g)
}

fn random_char(rng: &mut ThreadRng) -> char {
    let idx = rng.gen_range(0, LOWERCASE_CHARSET.len());
    LOWERCASE_CHARSET[idx] as char
}

pub fn solve_grid_reverse_hash_first_two_letters(grid: &Grid, words: &[&str]) -> Vec<WordLocation> {
    let mut all_words = words.to_vec();
    let reverse_words = words
        .iter()
        .map(|w| w.chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    all_words.extend(
        &reverse_words
            .iter()
            .map(|w| w.as_str())
            .collect::<Vec<&str>>(),
    );

    let directions = [
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
    ];

    let mut hashed: HashSet<Vec<char>> = HashSet::new();
    for w in all_words.iter() {
        let mut chars = w.chars();
        hashed.insert(vec![chars.next().unwrap(), chars.next().unwrap()]); // We expect that all words are at least two chars long.
    }

    let mut word_locations = Vec::new();

    for (row, col, direction) in
        iproduct!(0..grid.row_count(), 0..grid.col_count(), directions.iter())
    {
        let cell = Cell { row, col };
        if let Some(found) =
            find_word_in_direction_hash(vec![cell], &direction, grid, &all_words, &hashed, 2)
        {
            let mut w = WordLocation {
                word: found.word,
                start_cell: found.start_cell,
                end_cell: found.end_cell,
                direction: found.direction,
            };

            // If the originally-provided list of words does not contain the found word,
            // the found word must be reversed.
            if !words.contains(&w.word.as_str()) {
                w = w.into_reversed_location();
            }

            word_locations.push(w);
        }
    }

    word_locations.sort();
    word_locations
}

pub fn solve_grid_reverse_hash_first_letter(grid: &Grid, words: &[&str]) -> Vec<WordLocation> {
    let mut all_words = words.to_vec();
    let reverse_words = words
        .iter()
        .map(|w| w.chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    all_words.extend(
        &reverse_words
            .iter()
            .map(|w| w.as_str())
            .collect::<Vec<&str>>(),
    );

    let directions = [
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
    ];

    let mut hashed: HashSet<char> = HashSet::new();
    for w in all_words.iter() {
        hashed.insert(w.chars().next().unwrap()); // We expect that all words are at least one char long.
    }

    let mut word_locations = Vec::new();

    for (row, col, direction) in
        iproduct!(0..grid.row_count(), 0..grid.col_count(), directions.iter())
    {
        let cell = Cell { row, col };
        if hashed.contains(&grid.value_at_cell(&cell)) {
            let cell = Cell { row, col };
            if let Some(found) = find_word_in_direction(vec![cell], &direction, grid, &all_words) {
                let mut w = WordLocation {
                    word: found.word,
                    start_cell: found.start_cell,
                    end_cell: found.end_cell,
                    direction: found.direction,
                };

                // If the originally-provided list of words does not contain the found word,
                // the found word must be reversed.
                if !words.contains(&w.word.as_str()) {
                    w = w.into_reversed_location();
                }

                word_locations.push(w);
            }
        }
    }

    word_locations.sort();
    word_locations
}

pub fn solve_grid_hash_first_letter(grid: &Grid, words: &[&str]) -> Vec<WordLocation> {
    let mut hashed: HashSet<char> = HashSet::new();
    for w in words.iter() {
        hashed.insert(w.chars().next().unwrap()); // We expect that all words are at least one char long.
    }

    let mut word_locations = Vec::new();

    for (row, col, direction) in iproduct!(
        0..grid.row_count(),
        0..grid.col_count(),
        Direction::iterator()
    ) {
        let cell = Cell { row, col };
        if hashed.contains(&grid.value_at_cell(&cell)) {
            if let Some(found) = find_word_in_direction(vec![cell], &direction, grid, words) {
                word_locations.push(WordLocation {
                    word: found.word,
                    start_cell: found.start_cell,
                    end_cell: found.end_cell,
                    direction: found.direction,
                });
            }
        }
    }

    word_locations.sort();
    word_locations
}

pub fn solve_grid_naive(grid: &Grid, words: &[&str]) -> Vec<WordLocation> {
    let mut word_locations = Vec::new();

    for (row, col, direction) in iproduct!(
        0..grid.row_count(),
        0..grid.col_count(),
        Direction::iterator()
    ) {
        let cell = Cell { row, col };
        if let Some(found) = find_word_in_direction(vec![cell], &direction, grid, words) {
            word_locations.push(WordLocation {
                word: found.word,
                start_cell: found.start_cell,
                end_cell: found.end_cell,
                direction: found.direction,
            });
        }
    }

    word_locations.sort();
    word_locations
}

pub fn solve_grid_reverse_words(grid: &Grid, words: &[&str]) -> Vec<WordLocation> {
    let mut all_words = words.to_vec();
    let reverse_words = words
        .iter()
        .map(|w| w.chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    all_words.extend(
        &reverse_words
            .iter()
            .map(|w| w.as_str())
            .collect::<Vec<&str>>(),
    );

    let mut word_locations = Vec::new();

    let directions = [
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
    ];

    for (row, col, direction) in
        iproduct!(0..grid.row_count(), 0..grid.col_count(), directions.iter())
    {
        let cell = Cell { row, col };
        if let Some(found) = find_word_in_direction(vec![cell], &direction, grid, &all_words) {
            let mut w = WordLocation {
                word: found.word,
                start_cell: found.start_cell,
                end_cell: found.end_cell,
                direction: found.direction,
            };

            // If the originally-provided list of words does not contain the found word,
            // the found word must be reversed.
            if !words.contains(&w.word.as_str()) {
                w = w.into_reversed_location();
            }

            word_locations.push(w);
        }
    }

    word_locations.sort();
    word_locations
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::UpRight => Direction::DownLeft,
        Direction::Right => Direction::Left,
        Direction::DownRight => Direction::UpLeft,
        Direction::Down => Direction::Up,
        Direction::DownLeft => Direction::UpRight,
        Direction::Left => Direction::Right,
        Direction::UpLeft => Direction::DownRight,
    }
}

fn find_word_in_direction_hash(
    mut cells: Vec<Cell>,
    direction: &Direction,
    grid: &Grid,
    words: &[&str],
    hashed: &HashSet<Vec<char>>,
    hashed_length: usize,
) -> Option<WordLocation> {
    let current_cell = cells[cells.len() - 1];

    if cells.len() == hashed_length
        && !hashed.contains(
            &cells
                .iter()
                .map(|c| grid.value_at_cell(&c))
                .collect::<Vec<char>>(),
        )
    {
        return None;
    }

    let maybe_word = cells
        .iter()
        .map(|c| grid.value_at_cell(c))
        .collect::<String>();

    if words.contains(&maybe_word.as_str()) {
        return Some(WordLocation {
            word: maybe_word,
            start_cell: cells[0],
            end_cell: cells[cells.len() - 1],
            direction: direction.to_owned(),
        });
    }

    let next_cell = grid.next_cell_in_direction(&current_cell, direction)?;
    cells.push(next_cell);
    find_word_in_direction(cells, direction, grid, words)
}

fn find_word_in_direction(
    mut cells: Vec<Cell>,
    direction: &Direction,
    grid: &Grid,
    words: &[&str],
) -> Option<WordLocation> {
    let current_cell = cells[cells.len() - 1];

    let maybe_word = cells
        .iter()
        .map(|c| grid.value_at_cell(c))
        .collect::<String>();

    if words.contains(&maybe_word.as_str()) {
        return Some(WordLocation {
            word: maybe_word,
            start_cell: cells[0],
            end_cell: cells[cells.len() - 1],
            direction: direction.to_owned(),
        });
    }

    let next_cell = grid.next_cell_in_direction(&current_cell, direction)?;
    cells.push(next_cell);
    find_word_in_direction(cells, direction, grid, words)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Grid {
        Grid::new(&vec![
            vec![
                'h', 'b', 'b', 'q', 'd', 'v', 'p', 'n', 'r', 'e', 'w', 'z', 's', 'i', 'h',
            ],
            vec![
                'j', 'g', 'w', 'w', 'e', 'd', 'c', 'r', 'd', 'e', 'm', 'm', 'n', 'x', 'g',
            ],
            vec![
                'k', 'u', 'u', 'o', 't', 'g', 'n', 'e', 'e', 's', 'd', 'i', 'l', 'l', 'r',
            ],
            vec![
                'o', 's', 'q', 'l', 'a', 'm', 'p', 'i', 'i', 's', 't', 'r', 't', 'c', 'i',
            ],
            vec![
                'c', 'p', 'z', 'l', 'n', 'm', 'z', 'l', 'm', 'i', 'c', 'n', 'o', 'y', 'h',
            ],
            vec![
                'k', 'l', 'l', 'o', 'i', 'n', 'p', 'k', 'a', 'h', 'e', 'r', 'v', 'b', 'c',
            ],
            vec![
                'y', 'j', 'l', 'f', 'd', 's', 'r', 't', 'v', 'm', 'q', 'y', 'i', 'h', 'd',
            ],
            vec![
                'e', 'p', 'o', 'h', 'r', 'x', 'e', 'o', 'u', 'a', 'w', 'h', 'v', 'b', 'b',
            ],
            vec![
                'u', 'x', 'm', 'y', 'o', 'b', 't', 'r', 'b', 'd', 'q', 'k', 'a', 'g', 'e',
            ],
            vec![
                'o', 'l', 'r', 'n', 'o', 'm', 't', 'c', 'q', 'i', 'x', 'r', 'c', 'u', 'e',
            ],
            vec![
                'w', 'c', 'o', 't', 'c', 's', 'u', 'o', 'i', 'x', 'n', 'a', 'i', 'e', 'c',
            ],
            vec![
                'v', 's', 'm', 'f', 'n', 'r', 'e', 'c', 'i', 't', 'e', 'j', 'o', 's', 'k',
            ],
            vec![
                'e', 'p', 'i', 'i', 'w', 't', 'o', 'x', 'x', 'g', 'i', 'd', 'u', 's', 'z',
            ],
            vec![
                'p', 'l', 'a', 'u', 's', 'i', 'b', 'l', 'e', 'e', 'k', 'm', 's', 'i', 'i',
            ],
            vec![
                't', 'f', 'a', 'y', 'x', 'v', 'g', 'b', 'h', 'a', 'y', 'q', 'z', 'q', 'e',
            ],
        ])
    }

    fn words() -> Vec<String> {
        [
            "anxious".to_string(),
            "border".to_string(),
            "coordinated".to_string(),
            "follow".to_string(),
            "guess".to_string(),
            "hope".to_string(),
            "impede".to_string(),
            "initiate".to_string(),
            "instrument".to_string(),
            "mind".to_string(),
            "nose".to_string(),
            "plausible".to_string(),
            "prescribe".to_string(),
            "recite".to_string(),
            "robin".to_string(),
            "vivacious".to_string(),
        ]
        .to_vec()
    }

    fn assert_found_words(found_words: &[WordLocation]) {
        assert_eq!(found_words.len(), words().len());

        assert_eq!(
            found_words[0],
            WordLocation {
                word: "anxious".to_string(),
                start_cell: Cell { row: 10, col: 11 },
                end_cell: Cell { row: 10, col: 5 },
                direction: Direction::Left,
            }
        );

        assert_eq!(
            found_words[1],
            WordLocation {
                word: "border".to_string(),
                start_cell: Cell { row: 5, col: 13 },
                end_cell: Cell { row: 0, col: 8 },
                direction: Direction::UpLeft,
            }
        );

        assert_eq!(
            found_words[2],
            WordLocation {
                word: "coordinated".to_string(),
                start_cell: Cell { row: 10, col: 4 },
                end_cell: Cell { row: 0, col: 4 },
                direction: Direction::Up,
            }
        );

        assert_eq!(
            found_words[3],
            WordLocation {
                word: "follow".to_string(),
                start_cell: Cell { row: 6, col: 3 },
                end_cell: Cell { row: 1, col: 3 },
                direction: Direction::Up,
            }
        );

        assert_eq!(
            found_words[4],
            WordLocation {
                word: "guess".to_string(),
                start_cell: Cell { row: 8, col: 13 },
                end_cell: Cell { row: 12, col: 13 },
                direction: Direction::Down,
            }
        );

        assert_eq!(
            found_words[5],
            WordLocation {
                word: "hope".to_string(),
                start_cell: Cell { row: 7, col: 3 },
                end_cell: Cell { row: 7, col: 0 },
                direction: Direction::Left,
            }
        );

        assert_eq!(
            found_words[6],
            WordLocation {
                word: "impede".to_string(),
                start_cell: Cell { row: 5, col: 4 },
                end_cell: Cell { row: 0, col: 9 },
                direction: Direction::UpRight,
            }
        );

        assert_eq!(
            found_words[7],
            WordLocation {
                word: "initiate".to_string(),
                start_cell: Cell { row: 0, col: 13 },
                end_cell: Cell { row: 7, col: 6 },
                direction: Direction::DownLeft,
            }
        );

        assert_eq!(
            found_words[8],
            WordLocation {
                word: "instrument".to_string(),
                start_cell: Cell { row: 12, col: 3 },
                end_cell: Cell { row: 3, col: 12 },
                direction: Direction::UpRight,
            }
        );

        assert_eq!(
            found_words[9],
            WordLocation {
                word: "mind".to_string(),
                start_cell: Cell { row: 4, col: 8 },
                end_cell: Cell { row: 1, col: 5 },
                direction: Direction::UpLeft,
            }
        );

        assert_eq!(
            found_words[10],
            WordLocation {
                word: "nose".to_string(),
                start_cell: Cell { row: 9, col: 3 },
                end_cell: Cell { row: 12, col: 0 },
                direction: Direction::DownLeft,
            }
        );

        assert_eq!(
            found_words[11],
            WordLocation {
                word: "plausible".to_string(),
                start_cell: Cell { row: 13, col: 0 },
                end_cell: Cell { row: 13, col: 8 },
                direction: Direction::Right,
            }
        );

        assert_eq!(
            found_words[12],
            WordLocation {
                word: "prescribe".to_string(),
                start_cell: Cell { row: 0, col: 6 },
                end_cell: Cell { row: 8, col: 14 },
                direction: Direction::DownRight,
            }
        );

        assert_eq!(
            found_words[13],
            WordLocation {
                word: "recite".to_string(),
                start_cell: Cell { row: 11, col: 5 },
                end_cell: Cell { row: 11, col: 10 },
                direction: Direction::Right,
            }
        );

        assert_eq!(
            found_words[14],
            WordLocation {
                word: "robin".to_string(),
                start_cell: Cell { row: 6, col: 6 },
                end_cell: Cell { row: 10, col: 10 },
                direction: Direction::DownRight,
            }
        );

        assert_eq!(
            found_words[15],
            WordLocation {
                word: "vivacious".to_string(),
                start_cell: Cell { row: 5, col: 12 },
                end_cell: Cell { row: 13, col: 12 },
                direction: Direction::Down,
            }
        );
    }

    #[test]
    fn test_solve_grid_reverse() {
        assert_found_words(&solve_grid_reverse_words(
            &grid(),
            &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
        ));
    }

    #[test]
    fn test_solve_grid_naive() {
        assert_found_words(&solve_grid_naive(
            &grid(),
            &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
        ));
    }

    #[test]
    fn test_solve_grid_naive_hash_first_letter() {
        assert_found_words(&solve_grid_hash_first_letter(
            &grid(),
            &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
        ));
    }

    #[test]
    fn test_solve_grid_reverse_hash_first_letter() {
        assert_found_words(&solve_grid_reverse_hash_first_letter(
            &grid(),
            &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
        ));
    }

    #[test]
    fn test_solve_grid_reverse_hash_first_two_letters() {
        assert_found_words(&solve_grid_reverse_hash_first_two_letters(
            &grid(),
            &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
        ));
    }

    #[test]
    fn test_create_2x2() {
        let words2 = ["if", "it", "to"];

        let valid_grids = vec![
            Grid::new(&[vec!['i', 'f'], vec!['t', 'o']]),
            Grid::new(&[vec!['i', 'f'], vec!['o', 't']]),
            Grid::new(&[vec!['i', 't'], vec!['o', 'f']]),
            Grid::new(&[vec!['i', 't'], vec!['f', 'o']]),
            Grid::new(&[vec!['i', 'o'], vec!['t', 'f']]),
            Grid::new(&[vec!['i', 'o'], vec!['f', 't']]),
            Grid::new(&[vec!['f', 'i'], vec!['t', 'o']]),
            Grid::new(&[vec!['f', 'i'], vec!['o', 't']]),
            Grid::new(&[vec!['f', 'o'], vec!['t', 'i']]),
            Grid::new(&[vec!['f', 'o'], vec!['i', 't']]),
            Grid::new(&[vec!['f', 't'], vec!['o', 'i']]),
            Grid::new(&[vec!['f', 't'], vec!['i', 'o']]),
            Grid::new(&[vec!['t', 'i'], vec!['o', 'f']]),
            Grid::new(&[vec!['t', 'i'], vec!['f', 'o']]),
            Grid::new(&[vec!['t', 'o'], vec!['i', 'f']]),
            Grid::new(&[vec!['t', 'o'], vec!['f', 'i']]),
            Grid::new(&[vec!['t', 'f'], vec!['o', 'i']]),
            Grid::new(&[vec!['t', 'f'], vec!['i', 'o']]),
            Grid::new(&[vec!['o', 'i'], vec!['t', 'f']]),
            Grid::new(&[vec!['o', 'i'], vec!['f', 't']]),
            Grid::new(&[vec!['o', 't'], vec!['i', 'f']]),
            Grid::new(&[vec!['o', 't'], vec!['f', 'i']]),
            Grid::new(&[vec!['o', 'f'], vec!['t', 'i']]),
            Grid::new(&[vec!['o', 'f'], vec!['i', 't']]),
        ];

        let grid2 = generate_grid(2, 2, &words2).unwrap();
        assert!(valid_grids.contains(&grid2));

        let found_words2 = solve_grid_naive(&grid2, &words2);
        assert_eq!(found_words2.len(), words2.len());

        for (w, fw) in words2.iter().zip(found_words2.iter()) {
            assert_eq!(fw.word, **w);
        }
    }

    #[test]
    fn test_grid_cells_remaining_in_direction() {
        let g1 = Grid::empty(1, 1);

        for direction in Direction::iterator() {
            assert_eq!(
                g1.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, direction),
                0
            );
        }

        let g2 = Grid::empty(2, 2);

        // (0,0)
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::Up),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::UpRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::Right),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::DownRight),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::Down),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::DownLeft),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::Left),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::UpLeft),
            0
        );

        // (1,0)
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::Up),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::UpRight),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::Right),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::DownRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::Down),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::DownLeft),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::Left),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 0 }, &Direction::UpLeft),
            0
        );

        // (0,1)
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::Up),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::UpRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::Right),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::DownRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::Down),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::DownLeft),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::Left),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 0, col: 1 }, &Direction::UpLeft),
            0
        );

        // (1,1)
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::Up),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::UpRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::Right),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::DownRight),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::Down),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::DownLeft),
            0
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::Left),
            1
        );
        assert_eq!(
            g2.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::UpLeft),
            1
        );

        let g3 = Grid::empty(3, 3);
        assert_eq!(
            g3.cells_remaining_in_direction(&Cell { row: 0, col: 0 }, &Direction::DownRight),
            2
        );
        assert_eq!(
            g3.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::DownRight),
            1
        );
        assert_eq!(
            g3.cells_remaining_in_direction(&Cell { row: 1, col: 1 }, &Direction::UpLeft),
            1
        );
        assert_eq!(
            g3.cells_remaining_in_direction(&Cell { row: 2, col: 2 }, &Direction::Left),
            2
        );
    }
}
