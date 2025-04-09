use std::fs;

use markdownit::{parser, tokenizer};


fn main() {
    let input = fs::read_to_string("./simple.txt").unwrap();
    let tokens = tokenizer(&input);
    let html = parser(&tokens);
    // println!("{}", html);
}
