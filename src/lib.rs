use itertools::iproduct;
use std::slice::Iter;

pub type Cell = (usize, usize);

#[derive(Debug)]
pub struct Square {
    chars: Vec<Vec<char>>,
}

impl Square {
    pub fn new(chars: &Vec<Vec<char>>) -> Square {
        Square {
            chars: chars.clone(),
        }
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

    fn value_at_cell(&self, cell: &Cell) -> char {
        // TODO: bounds check
        self.chars[cell.0][cell.1]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FoundWord {
    pub word: String,
    pub start_cell: Cell,
    pub end_cell: Cell,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
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

pub fn solve_square(square: &Square, words: &[&str]) -> Vec<FoundWord> {
    let mut found_words = words
        .iter()
        .map(|w| FoundWord {
            word: w.to_owned().to_owned(),
            start_cell: (0, 0),
            end_cell: (0, 0),
        })
        .collect::<Vec<_>>();

    for (row, col, direction) in
        iproduct!(0..square.rows(), 0..square.cols(), Direction::iterator())
    {
        let cell = (row, col);
        if let Some(fw) = find_word_in_direction(vec![cell], &direction, square, words) {
            let mut w = &mut found_words.iter_mut().find(|w| w.word == fw.word).unwrap();
            w.start_cell = fw.start_cell;
            w.end_cell = fw.end_cell;
        }
    }

    found_words
}

fn find_word_in_direction(
    mut cells: Vec<Cell>,
    direction: &Direction,
    square: &Square,
    words: &[&str],
) -> Option<FoundWord> {
    let current_cell = cells[cells.len() - 1];

    let maybe_word = cells
        .iter()
        .map(|c| square.value_at_cell(c))
        .collect::<String>();

    if let Some(found_word) = words.iter().find(|&&w| w == maybe_word) {
        return Some(FoundWord {
            word: found_word.to_string(),
            start_cell: cells[0],
            end_cell: cells[cells.len() - 1],
        });
    }

    if let Some(next_cell) = next_cell_in_direction(&current_cell, direction, square) {
        cells.push(next_cell);
        return find_word_in_direction(cells, direction, square, words);
    }

    return None;
}

fn next_cell_in_direction(cell: &Cell, direction: &Direction, square: &Square) -> Option<Cell> {
    let (row, col) = match direction {
        Direction::Up => (cell.0 as isize - 1, cell.1 as isize),
        Direction::UpRight => (cell.0 as isize - 1, cell.1 as isize + 1),
        Direction::Right => (cell.0 as isize, cell.1 as isize + 1),
        Direction::DownRight => (cell.0 as isize + 1, cell.1 as isize + 1),
        Direction::Down => (cell.0 as isize + 1, cell.1 as isize),
        Direction::DownLeft => (cell.0 as isize + 1, cell.1 as isize - 1),
        Direction::Left => (cell.0 as isize, cell.1 as isize - 1),
        Direction::UpLeft => (cell.0 as isize - 1, cell.1 as isize - 1),
    };

    if row < 0 || col < 0 || row as usize >= square.rows() || col as usize >= square.cols() {
        return None;
    }

    return Some((row as usize, col as usize));
}
