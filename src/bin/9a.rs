use aoc23::read_stdin_to_string;
use chumsky::prelude::*;
use itertools::Itertools;

fn parser() -> impl Parser<char, Vec<Vec<i64>>, Error = Simple<char>> {
    let int = just('-')
        .or_not()
        .chain::<char, _, _>(filter(|c: &char| c.is_ascii_digit()).repeated().at_least(1))
        .collect::<String>()
        .from_str::<i64>()
        .unwrapped()
        .padded_by(just(' ').ignored().repeated());
    let seq = int.repeated().at_least(1).padded();
    seq.repeated().then_ignore(end())
}

fn extrapolate(mut seq: Vec<i64>) -> i64 {
    let mut stack = Vec::new();
    while !seq.iter().all(|&x| x == 0) {
        let new = seq.iter().tuple_windows().map(|(x, y)| y - x).collect_vec();
        stack.push(seq);
        seq = new;
    }

    stack
        .into_iter()
        .rev()
        .fold(0, |below, seq| below + seq.last().unwrap())
}

fn main() {
    let seqs = parser().parse(read_stdin_to_string()).unwrap();
    let result = seqs.into_iter().map(extrapolate).sum::<i64>();
    println!("{result}");
}
