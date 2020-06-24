use wordsquare::*;

fn main() {
    let square = Square::new(&vec![
        vec!['t', 'o', 'p'],
        vec!['a', 'e', 'z'],
        vec!['n', 'o', 'w'],
    ]);
    let words = ["now", "pen", "tan", "top"];

    println!("words: {:?}", words);
    println!("{:?}", square);

    let found_words = solve_square(&square, &words);
    for w in found_words {
        println!("{:?}", w)
    }
}
