#![feature(test)]

extern crate test;

use itertools::iproduct;
use itertools::Itertools;
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WordLocation {
    pub word: String,
    pub start_cell: Cell,
    pub end_cell: Cell,
    pub direction: Direction,
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

pub fn solve_square_naive(square: &Square, words: &[&str]) -> Vec<WordLocation> {
    let mut word_locations = Vec::new();

    for (row, col, direction) in
        iproduct!(0..square.rows(), 0..square.cols(), Direction::iterator())
    {
        let cell = Cell { row, col };
        if let Some(found) = find_word_in_direction(vec![cell], &direction, square, words) {
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

pub fn solve_square_reverse_words(square: &Square, words: &[&str]) -> Vec<WordLocation> {
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

    for (row, col, direction) in iproduct!(0..square.rows(), 0..square.cols(), directions.iter()) {
        let cell = Cell { row, col };
        if let Some(found) = find_word_in_direction(vec![cell], &direction, square, &all_words) {
            let w: WordLocation;
            if words.contains(&found.word.as_str()) {
                w = WordLocation {
                    word: found.word,
                    start_cell: found.start_cell,
                    end_cell: found.end_cell,
                    direction: found.direction,
                };
            } else {
                w = WordLocation {
                    word: found.word.to_string().chars().rev().collect::<String>(),
                    start_cell: found.end_cell,
                    end_cell: found.start_cell,
                    direction: opposite_direction(&found.direction),
                };
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

    fn square() -> Square {
        Square::new(&vec![
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

    #[bench]
    fn bench_solve_square_naive(b: &mut Bencher) {
        b.iter(|| {
            let found_words = solve_square_naive(
                &square(),
                &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
            );

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
        });
    }

    #[bench]
    fn bench_solve_square_reverse(b: &mut Bencher) {
        b.iter(|| {
            let found_words = solve_square_reverse_words(
                &square(),
                &words().iter().map(|w| w.as_str()).collect::<Vec<&str>>(),
            );

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
        });
    }
}
