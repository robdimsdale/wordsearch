use wordsquare::*;

fn main() {
    let square = Square::new(&[
        vec!['t', 'o', 'p'],
        vec!['a', 'e', 'z'],
        vec!['n', 'o', 'w'],
    ]);
    let words = ["now", "pen", "tan", "top"];

    println!("words: {:?}", words);
    print!("{}", square);

    let found_words = solve_square(&square, &words);
    for w in found_words {
        println!("{:?}", w);
        println!("{}", square.one_word_square(&w));
    }
}
