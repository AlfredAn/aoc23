use std::ops::Range;

use aoc23::read_stdin_to_string;
use chumsky::prelude::*;

type Seeds = Vec<u64>;
type RangeMap = Vec<(Range<u64>, i64)>;

fn parser() -> impl Parser<char, (Seeds, Vec<RangeMap>), Error = Simple<char>> {
    let int = text::int(10).from_str::<u64>().unwrapped().padded();

    let seeds = just("seeds: ").ignore_then(int.repeated());

    let map_label = filter(|&c: &char| c.is_ascii_alphabetic() || c == '-')
        .ignored()
        .repeated()
        .then_ignore(just(" map:"));

    let range = int
        .then(int)
        .then(int)
        .map(|((dst_start, src_start), len)| {
            (
                src_start..src_start + len,
                dst_start as i64 - src_start as i64,
            )
        });

    let map = map_label.ignore_then(range.repeated());

    seeds.then(map.repeated())
}

fn main() {
    let (seeds, maps) = parser().parse(read_stdin_to_string()).unwrap();
    let mut seed_locs = Vec::new();

    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            for (range, offset) in map {
                if range.contains(&val) {
                    val = (val as i64 + offset) as u64;
                    break;
                }
            }
        }
        seed_locs.push(val);
    }

    let best_loc = seed_locs.into_iter().min().unwrap();
    println!("{best_loc}");
}
