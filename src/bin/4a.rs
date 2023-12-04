use aoc23::*;
use chumsky::prelude::*;
use vecmap::VecSet;

#[derive(Debug)]
struct Card {
    winning: VecSet<u8>,
    chosen: VecSet<u8>,
}

fn parser() -> impl Parser<char, Vec<Card>, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| s.parse::<u8>().unwrap())
        .padded();

    let head = just("Card ").then(int).then(just(':')).ignored();
    let numbers = int.repeated();
    let tail = numbers
        .then_ignore(just('|'))
        .then(numbers)
        .map(|(winning, chosen)| Card {
            winning: winning.into(),
            chosen: chosen.into(),
        });

    head.ignore_then(tail).repeated().then_ignore(end())
}

fn main() {
    let input = read_stdin_to_string();
    let cards = parser().parse(input).unwrap();

    let mut score = 0;
    for card in &cards {
        let mut card_score = 0;
        for x in &card.chosen {
            if card.winning.contains(x) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        score += card_score;
    }

    println!("{score}");
}
