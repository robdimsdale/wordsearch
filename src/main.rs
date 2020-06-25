use wordsquare::*;

fn main() {
    let square = Square::new(&[
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
    ]);

    let words = [
        "anxious",
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
        "recite",
        "robin",
        "vivacious",
    ];

    println!("words: {:?}", words);
    print!("{}", square);

    let found_words = solve_square_naive(&square, &words);

    for w in &found_words {
        println!("{:?}", w);
        println!("{}", square.one_word_square(&w));
    }

    assert_eq!(found_words.len(), words.len());
}
