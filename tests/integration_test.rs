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
        FoundWord {
            word: "now".to_string(),
            start_cell: (2, 0),
            end_cell: (2, 2),
        }
    )
}
