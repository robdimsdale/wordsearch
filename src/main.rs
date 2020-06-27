use wordsearch::*;

fn main() {
    let words = [
        "anxious",
        "blossom",
        "border",
        "coordinated",
        "follow",
        "guess",
        "hope",
        "impede",
        "initiate",
        "instrument",
        "mind",
        "nose",
        "plausible",
        "prescribe",
        "produce",
        "recite",
        "robin",
        "stress",
        "vivacious",
    ];
    let grid = generate_grid(14, 13, &words);

    println!("words: {:?}", words);
    print!("{}", grid);

    let found_words = solve_grid_hash_first_letter(&grid, &words);

    for w in &found_words {
        println!("{:?}", w);
        println!("{}", grid.one_word_grid(&w));
    }

    assert_eq!(found_words.len(), words.len());
}
