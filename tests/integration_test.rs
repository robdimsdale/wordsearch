use wordsearch::*;

#[test]
fn test_generate_square() {
    let words2 = ["if", "it", "to"];
    let valid_squares = vec![
        Square::new(&[vec!['i', 'f'], vec!['t', 'o']]),
        Square::new(&[vec!['i', 'f'], vec!['o', 't']]),
        Square::new(&[vec!['i', 't'], vec!['o', 'f']]),
        Square::new(&[vec!['i', 't'], vec!['f', 'o']]),
        Square::new(&[vec!['i', 'o'], vec!['t', 'f']]),
        Square::new(&[vec!['i', 'o'], vec!['f', 't']]),
        Square::new(&[vec!['f', 'i'], vec!['t', 'o']]),
        Square::new(&[vec!['f', 'i'], vec!['o', 't']]),
        Square::new(&[vec!['f', 'o'], vec!['t', 'i']]),
        Square::new(&[vec!['f', 'o'], vec!['i', 't']]),
        Square::new(&[vec!['f', 't'], vec!['o', 'i']]),
        Square::new(&[vec!['f', 't'], vec!['i', 'o']]),
        Square::new(&[vec!['t', 'i'], vec!['o', 'f']]),
        Square::new(&[vec!['t', 'i'], vec!['f', 'o']]),
        Square::new(&[vec!['t', 'o'], vec!['i', 'f']]),
        Square::new(&[vec!['t', 'o'], vec!['f', 'i']]),
        Square::new(&[vec!['t', 'f'], vec!['o', 'i']]),
        Square::new(&[vec!['t', 'f'], vec!['i', 'o']]),
        Square::new(&[vec!['o', 'i'], vec!['t', 'f']]),
        Square::new(&[vec!['o', 'i'], vec!['f', 't']]),
        Square::new(&[vec!['o', 't'], vec!['i', 'f']]),
        Square::new(&[vec!['o', 't'], vec!['f', 'i']]),
        Square::new(&[vec!['o', 'f'], vec!['t', 'i']]),
        Square::new(&[vec!['o', 'f'], vec!['i', 't']]),
    ];

    let square2 = generate_square(2, 2, &words2);
    assert!(valid_squares.contains(&square2));

    let found_words2 = solve_square_naive(&square2, &words2);
    assert_eq!(found_words2.len(), words2.len());

    for (w, fw) in words2.iter().zip(found_words2.iter()) {
        assert_eq!(fw.word, **w);
    }

    let words3 = ["now", "pen", "tan", "top"];
    let square3 = generate_square(3, 3, &words3);

    assert_eq!(square3.col_count(), 3);
    assert_eq!(square3.row_count(), 3);

    let found_words3 = solve_square_naive(&square3, &words3);

    assert_eq!(found_words3.len(), words3.len());

    for (w, fw) in words3.iter().zip(found_words3.iter()) {
        assert_eq!(fw.word, **w);
    }

    let words3x4 = ["pin", "post", "sit", "tent", "tie"];
    let square3x4 = generate_square(3, 4, &words3x4);

    assert_eq!(square3x4.col_count(), 4);
    assert_eq!(square3x4.row_count(), 3);

    let found_words3x4 = solve_square_naive(&square3x4, &words3x4);

    assert_eq!(found_words3x4.len(), words3x4.len());

    for (w, fw) in words3x4.iter().zip(found_words3x4.iter()) {
        assert_eq!(fw.word, **w);
    }
}

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
