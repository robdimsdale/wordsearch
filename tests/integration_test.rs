use wordsquare::*;

#[test]
fn test_solve_square() {
    let square = Square::new(&vec![
        vec!['t', 'o', 'p'],
        vec!['a', 'e', 'z'],
        vec!['n', 'o', 'w'],
    ]);

    let words = ["now", "pen", "tan", "top"];

    let found_words_naive = solve_square_naive(&square, &words);
    let found_words_hash_first_letter = solve_square_hash_first_letter(&square, &words);
    let found_words_reverse = solve_square_reverse_words(&square, &words);
    let found_words_reverse_hash_first_letter =
        solve_square_reverse_hash_first_letter(&square, &words);
    let found_words_reverse_hash_first_two_letter =
        solve_square_reverse_hash_first_two_letters(&square, &words);

    assert_eq!(found_words_naive, found_words_hash_first_letter);
    assert_eq!(found_words_naive, found_words_reverse);
    assert_eq!(found_words_naive, found_words_hash_first_letter);
    assert_eq!(found_words_naive, found_words_reverse_hash_first_letter);
    assert_eq!(found_words_naive, found_words_reverse_hash_first_two_letter);

    assert_eq!(found_words_naive.len(), 4);

    assert_eq!(
        found_words_naive[0],
        WordLocation {
            word: "now".to_string(),
            start_cell: Cell { row: 2, col: 0 },
            end_cell: Cell { row: 2, col: 2 },
            direction: Direction::Right,
        }
    );

    assert_eq!(
        found_words_naive[1],
        WordLocation {
            word: "pen".to_string(),
            start_cell: Cell { row: 0, col: 2 },
            end_cell: Cell { row: 2, col: 0 },
            direction: Direction::DownLeft,
        }
    );

    assert_eq!(
        found_words_naive[2],
        WordLocation {
            word: "tan".to_string(),
            start_cell: Cell { row: 0, col: 0 },
            end_cell: Cell { row: 2, col: 0 },
            direction: Direction::Down,
        }
    );

    assert_eq!(
        found_words_naive[3],
        WordLocation {
            word: "top".to_string(),
            start_cell: Cell { row: 0, col: 0 },
            end_cell: Cell { row: 0, col: 2 },
            direction: Direction::Right,
        }
    );
}
