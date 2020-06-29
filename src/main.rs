use clap::{App, Arg};
use wordsearch::*;

const ROWS_SIZE_ARG: &str = "rows";
const COLS_SIZE_ARG: &str = "cols";

fn main() {
    let matches = App::new("Wordsearch")
        .version("0.1.0")
        .author("Rob Dimsdale-Zucker")
        .about("Generates and solves wordsearches")
        .arg(
            Arg::with_name(ROWS_SIZE_ARG)
                .long(ROWS_SIZE_ARG)
                .help("Number of rows in generated wordsearch.")
                .takes_value(true)
                .value_name("ROWS"),
        )
        .arg(
            Arg::with_name(COLS_SIZE_ARG)
                .long(COLS_SIZE_ARG)
                .help("Number of columns in generated wordsearch.")
                .takes_value(true)
                .value_name("COLS"),
        )
        .get_matches();

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

    let rows = match matches.value_of(ROWS_SIZE_ARG) {
        None => 15,
        Some(v) => v.parse().unwrap(),
    };

    let cols = match matches.value_of(COLS_SIZE_ARG) {
        None => 15,
        Some(v) => v.parse().unwrap(),
    };

    let grid = generate_grid(rows, cols, &words).unwrap();

    println!("words: {:?}", words);
    print!("{}", grid);

    let found_words = solve_grid_hash_first_letter(&grid, &words);

    for w in &found_words {
        println!("{:?}", w);
        println!("{}", grid.one_word_grid(&w));
    }

    assert_eq!(found_words.len(), words.len());
}
