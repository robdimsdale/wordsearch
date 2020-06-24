use itertools::iproduct;
use itertools::Itertools;
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Square {
    chars: Vec<Vec<char>>,
}

fn direction_between_cells(start_cell: &Cell, end_cell: &Cell) -> Direction {
    let up = start_cell.row > end_cell.row;
    let down = start_cell.row < end_cell.row;

    let left = start_cell.col > end_cell.col;
    let right = start_cell.col < end_cell.col;

    if up {
        if left {
            return Direction::UpLeft;
        } else if right {
            return Direction::UpRight;
        } else {
            return Direction::Up;
        }
    } else if down {
        if left {
            return Direction::DownLeft;
        } else if right {
            return Direction::DownRight;
        } else {
            return Direction::Down;
        }
    } else {
        if left {
            return Direction::Left;
        } else if right {
            return Direction::Right;
        } else {
            panic!();
        }
    }
}

impl Square {
    pub fn new(chars: &[Vec<char>]) -> Square {
        Square {
            chars: chars.to_owned(),
        }
    }

    pub fn one_word_square(&self, start_cell: &Cell, end_cell: &Cell) -> Square {
        let direction = direction_between_cells(start_cell, end_cell);

        let mut square = Square::new(&vec![vec!['_'; self.cols()]; self.rows()]);
        let mut cell = start_cell.to_owned();

        let val = self.value_at_cell(&cell);
        square.set_value_at_cell(&cell, val);

        while let Some(next_cell) = self.next_cell_in_direction(&cell, &direction) {
            let val = self.value_at_cell(&cell);
            square.set_value_at_cell(&cell, val);

            if next_cell == *end_cell {
                break;
            }
            cell = next_cell
        }

        let last_val = self.value_at_cell(&end_cell);
        square.set_value_at_cell(&end_cell, last_val);

        square
    }

    fn rows(&self) -> usize {
        self.chars.len()
    }

    fn cols(&self) -> usize {
        if self.chars.is_empty() {
            0
        } else {
            self.chars[0].len()
        }
    }

    fn set_value_at_cell(&mut self, cell: &Cell, val: char) {
        self.chars[cell.row][cell.col] = val;
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

        if row < 0 || col < 0 || row as usize >= self.rows() || col as usize >= self.cols() {
            return None;
        }

        Some(Cell {
            row: row as usize,
            col: col as usize,
        })
    }
}

impl fmt::Display for Square {
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

#[derive(Debug, Eq, PartialEq)]
pub struct WordLocation {
    pub word: String,
    pub start_cell: Cell,
    pub end_cell: Cell,
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub fn solve_square(square: &Square, words: &[&str]) -> Vec<WordLocation> {
    let mut word_locations = words
        .iter()
        .map(|w| WordLocation {
            word: w.to_owned().to_owned(),
            start_cell: Cell { row: 0, col: 0 },
            end_cell: Cell { row: 0, col: 0 },
            direction: Direction::Up,
        })
        .collect::<Vec<_>>();

    for (row, col, direction) in
        iproduct!(0..square.rows(), 0..square.cols(), Direction::iterator())
    {
        let cell = Cell { row, col };
        if let Some(found) = find_word_in_direction(vec![cell], &direction, square, words) {
            let mut w = &mut word_locations
                .iter_mut()
                .find(|w| w.word == found.word)
                .unwrap();
            w.start_cell = found.start_cell;
            w.end_cell = found.end_cell;
            w.direction = found.direction;
        }
    }

    word_locations
}

fn find_word_in_direction(
    mut cells: Vec<Cell>,
    direction: &Direction,
    square: &Square,
    words: &[&str],
) -> Option<WordLocation> {
    let current_cell = cells[cells.len() - 1];

    let maybe_word = cells
        .iter()
        .map(|c| square.value_at_cell(c))
        .collect::<String>();

    if let Some(found_word) = words.iter().find(|&&w| w == maybe_word) {
        return Some(WordLocation {
            word: found_word.to_string(),
            start_cell: cells[0],
            end_cell: cells[cells.len() - 1],
            direction: direction.to_owned(),
        });
    }

    if let Some(next_cell) = square.next_cell_in_direction(&current_cell, direction) {
        cells.push(next_cell);
        return find_word_in_direction(cells, direction, square, words);
    }

    None
}
