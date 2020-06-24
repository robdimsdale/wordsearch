use wordsquare::*;

#[test]
fn test_square() {
    let square = Square::new(&vec![
        vec!['t', 'o', 'p'],
        vec!['a', 'e', 'z'],
        vec!['n', 'o', 'w'],
    ]);
    let words = ["now", "pen", "tan", "top"];

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
}
