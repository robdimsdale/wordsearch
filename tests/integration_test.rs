use wordsearch::*;

#[test]
fn test_generate_grid() {
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

    let words3 = ["now", "pen", "tan", "top"];
    let grid3 = generate_grid(3, 3, &words3).unwrap();

    assert_eq!(grid3.col_count(), 3);
    assert_eq!(grid3.row_count(), 3);

    let found_words3 = solve_grid_naive(&grid3, &words3);

    assert_eq!(found_words3.len(), words3.len());

    for (w, fw) in words3.iter().zip(found_words3.iter()) {
        assert_eq!(fw.word, **w);
    }

    let words3x4 = ["pin", "post", "sit", "tent", "tie"];
    let grid3x4 = generate_grid(3, 4, &words3x4).unwrap();

    assert_eq!(grid3x4.col_count(), 4);
    assert_eq!(grid3x4.row_count(), 3);

    let found_words3x4 = solve_grid_naive(&grid3x4, &words3x4);

    assert_eq!(found_words3x4.len(), words3x4.len());

    for (w, fw) in words3x4.iter().zip(found_words3x4.iter()) {
        assert_eq!(fw.word, **w);
    }
}

#[test]
fn test_solve_grid() {
    let grid = Grid::new(&vec![
        vec!['t', 'o', 'p'],
        vec!['a', 'e', 'z'],
        vec!['n', 'o', 'w'],
    ]);

    let words = ["now", "pen", "tan", "top"];

    let found_words_naive = solve_grid_naive(&grid, &words);
    let found_words_hash_first_letter = solve_grid_hash_first_letter(&grid, &words);
    let found_words_reverse = solve_grid_reverse_words(&grid, &words);
    let found_words_reverse_hash_first_letter = solve_grid_reverse_hash_first_letter(&grid, &words);
    let found_words_reverse_hash_first_two_letter =
        solve_grid_reverse_hash_first_two_letters(&grid, &words);

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
