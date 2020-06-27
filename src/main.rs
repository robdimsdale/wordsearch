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
    let square = generate_square(14, 13, &words);

    println!("words: {:?}", words);
    print!("{}", square);

    let found_words = solve_square_hash_first_letter(&square, &words);

    for w in &found_words {
        println!("{:?}", w);
        println!("{}", square.one_word_square(&w));
    }

    assert_eq!(found_words.len(), words.len());
}
