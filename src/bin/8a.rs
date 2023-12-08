use std::iter::repeat;

use aoc23::read_stdin_to_string;
use chumsky::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

fn parser() -> impl Parser<char, (Vec<Dir>, Vec<(String, (String, String))>), Error = Simple<char>>
{
    let directions = choice((just('L').to(Dir::Left), just('R').to(Dir::Right)))
        .repeated()
        .collect()
        .padded();

    let tag = filter(|&c: &char| c.is_ascii_uppercase())
        .repeated()
        .collect();

    let node = tag
        .then_ignore(just(" = ("))
        .then(tag.then_ignore(just(", ")).then(tag).then_ignore(just(")")))
        .padded();

    directions.then(node.repeated()).then_ignore(end())
}

fn main() {
    let (directions, nodes) = parser().parse(read_stdin_to_string()).unwrap();

    let tag_to_index: FxHashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, (tag, _))| (&**tag, i))
        .collect();

    let mut pos = "AAA";
    let goal = "ZZZ";

    let mut directions = repeat(()).flat_map(|_| directions.iter().copied());
    let mut steps = 0;

    while pos != goal {
        let i = tag_to_index[pos];
        pos = match directions.next().unwrap() {
            Dir::Left => &*nodes[i].1 .0,
            Dir::Right => &*nodes[i].1 .1,
        };
        steps += 1;
    }

    println!("{steps}");
}
