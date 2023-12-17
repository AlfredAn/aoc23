use std::num::Wrapping;

use aoc23::parse_stdin;
use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Op {
    Remove,
    Add(u8),
}

fn parser() -> impl Parser<char, Vec<(String, Op)>, Error = Simple<char>> {
    let label = filter(char::is_ascii_lowercase)
        .repeated()
        .at_least(1)
        .collect();

    let op = choice((
        just('-').to(Op::Remove),
        just('=')
            .ignore_then(filter(|c| ('1'..='9').contains(c)))
            .map(|c| Op::Add(c.to_digit(10).unwrap() as u8)),
    ));

    let entry = label.then(op);
    entry
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
    let mut boxes = [(); 256].map(|_| Vec::new());

    for (label, op) in parse_stdin(parser()) {
        println!("{:?}", (&label, op));
        let b = &mut boxes[hash(&label) as usize];
        let i = b.iter().position(|(l2, _)| *l2 == label);

        match (op, i) {
            (Op::Remove, Some(i)) => {
                b.remove(i);
            }
            (Op::Remove, None) => (),
            (Op::Add(f), Some(i)) => {
                b[i].1 = f;
            }
            (Op::Add(f), None) => {
                b.push((label.clone(), f));
            }
        }
    }

    let power = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, (_, f))| (i as u64 + 1) * (j as u64 + 1) * (*f as u64))
        })
        .sum::<u64>();

    power
}

fn main() {
    println!("{:?}", solve());
}
