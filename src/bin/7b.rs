use std::cmp::Reverse;

use aoc23::read_stdin_to_string;
use chumsky::prelude::*;
use itertools::Itertools;

type Card = u8;
type Hand = [Card; 5];
type Bid = u16;

const CARDS: &str = "J23456789TQKA";
const JOKER: Card = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: Hand) -> HandType {
    let counts = hand
        .into_iter()
        .sorted_unstable_by_key(|&c| (Reverse(hand.iter().filter(|&&c2| c == c2).count()), c))
        .dedup_with_count()
        .map(|(count, _)| count)
        .collect_vec();

    match counts.as_slice() {
        &[5, ..] => HandType::FiveOfAKind,
        &[4, ..] => HandType::FourOfAKind,
        &[3, 2, ..] => HandType::FullHouse,
        &[3, ..] => HandType::ThreeOfAKind,
        &[2, 2, ..] => HandType::TwoPair,
        &[2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn hand_type_ext(mut hand: Hand) -> HandType {
    if let Some((i, _)) = hand.into_iter().find_position(|&c| c == JOKER) {
        (1..CARDS.len() as Card)
            .map(|c| {
                hand[i] = c;
                hand_type_ext(hand)
            })
            .max()
            .unwrap()
    } else {
        hand_type(hand)
    }
}

fn parser() -> impl Parser<char, Vec<(Hand, Bid)>, Error = Simple<char>> {
    let card = one_of(CARDS).map(|c: char| {
        CARDS
            .chars()
            .position(|c2| c == c2)
            .map(|i| i as Card)
            .unwrap()
    });

    let hand = card
        .repeated()
        .exactly(5)
        .padded()
        .map(|hand| Hand::try_from(hand).unwrap());

    let bid = text::int(10).from_str::<Bid>().unwrapped().padded();
    let row = hand.then(bid);

    row.repeated()
}

fn main() {
    let hands = parser().parse(read_stdin_to_string()).unwrap();

    let result = hands
        .into_iter()
        .map(|(hand, bid)| ((hand_type_ext(hand), hand), bid))
        .sorted_unstable_by_key(|&(hand_with_type, _)| hand_with_type)
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid as u64)
        .sum::<u64>();

    dbg!(result);
}
