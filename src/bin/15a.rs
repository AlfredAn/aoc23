use std::num::Wrapping;

use aoc23::parse_stdin;
use chumsky::prelude::*;

fn parser() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    let string = none_of(",\r\n").repeated().at_least(1).collect();
    string
        .separated_by(just(","))
        .at_least(1)
        .padded()
        .then_ignore(end())
}

fn hash(s: &str) -> u8 {
    let mut x = Wrapping(0);
    for c in s.bytes() {
        x += c;
        x *= 17;
    }
    x.0
}

fn solve() -> u64 {
    parse_stdin(parser())
        .into_iter()
        .map(|s| hash(&s) as u64)
        .sum()
}

fn main() {
    println!("{:?}", solve());
}
