#![feature(test)]

extern crate test;

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

impl Square {
    pub fn new(chars: &[Vec<char>]) -> Square {
        Square {
            chars: chars.to_owned(),
        }
    }

    pub fn one_word_square(&self, word: &WordLocation) -> Square {
        let mut square = Square::new(&vec![vec!['_'; self.cols()]; self.rows()]);
        let mut cell = word.start_cell.to_owned();

        let val = self.value_at_cell(&cell);
        square.set_value_at_cell(&cell, val);

        while let Some(next_cell) = self.next_cell_in_direction(&cell, &word.direction) {
            let val = self.value_at_cell(&cell);
            square.set_value_at_cell(&cell, val);

            if next_cell == word.end_cell {
                break;
            }
            cell = next_cell
        }

        let last_val = self.value_at_cell(&word.end_cell);
        square.set_value_at_cell(&word.end_cell, last_val);

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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solve_square(b: &mut Bencher) {
        let square = Square::new(&vec![
            vec!['t', 'o', 'p'],
            vec!['a', 'e', 'z'],
            vec!['n', 'o', 'w'],
        ]);
        let words = ["now", "pen", "tan", "top"];

        b.iter(|| {
            let found_words = solve_square(&square, &words);

            assert_eq!(found_words.len(), 4);

            assert_eq!(
                found_words[0],
                WordLocation {
                    word: "now".to_string(),
                    start_cell: Cell { row: 2, col: 0 },
                    end_cell: Cell { row: 2, col: 2 },
                    direction: Direction::Right,
                }
            );

            assert_eq!(
                found_words[1],
                WordLocation {
                    word: "pen".to_string(),
                    start_cell: Cell { row: 0, col: 2 },
                    end_cell: Cell { row: 2, col: 0 },
                    direction: Direction::DownLeft,
                }
            );

            assert_eq!(
                found_words[2],
                WordLocation {
                    word: "tan".to_string(),
                    start_cell: Cell { row: 0, col: 0 },
                    end_cell: Cell { row: 2, col: 0 },
                    direction: Direction::Down,
                }
            );

            assert_eq!(
                found_words[3],
                WordLocation {
                    word: "top".to_string(),
                    start_cell: Cell { row: 0, col: 0 },
                    end_cell: Cell { row: 0, col: 2 },
                    direction: Direction::Right,
                }
            );
        });
    }
}
