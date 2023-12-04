use std::collections::VecDeque;

use aoc23::*;
use chumsky::prelude::*;
use vecmap::VecSet;

#[derive(Debug)]
struct Card {
    seq: usize,
    winning: VecSet<u8>,
    chosen: VecSet<u8>,
}

fn parser() -> impl Parser<char, Vec<Card>, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| s.parse::<u8>().unwrap())
        .padded();

    let head = just("Card ").ignore_then(int).then_ignore(just(':'));
    let numbers = int.repeated();
    let tail = numbers.then_ignore(just('|')).then(numbers);

    head.then(tail)
        .map(|(seq, (w, c))| Card {
            seq: (seq - 1).into(),
            winning: w.into(),
            chosen: c.into(),
        })
        .repeated()
        .then_ignore(end())
}

fn main() {
    let input = read_stdin_to_string();
    let cards = parser().parse(input).unwrap();
    let mut queue = VecDeque::new();
    queue.extend(cards.iter());

    let mut total_cards = 0;
    while let Some(card) = queue.pop_front() {
        total_cards += 1;

        let matches = card
            .chosen
            .iter()
            .filter(|x| card.winning.contains(x))
            .count();

        queue.extend((1..=matches).map(|i| &cards[i + card.seq]));
    }

    println!("{total_cards}");
}
