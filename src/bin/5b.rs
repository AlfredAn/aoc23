use std::{mem, ops::Range};

use aoc23::read_stdin_to_string;
use chumsky::prelude::*;
use itertools::Itertools;

type Seeds = Vec<Range<u64>>;
type RangeMap = Vec<(Range<u64>, i64)>;

fn parser() -> impl Parser<char, (Seeds, Vec<RangeMap>), Error = Simple<char>> {
    let int = text::int(10).from_str::<u64>().unwrapped().padded();

    let seed_range = int.then(int).map(|(start, len)| start..start + len);

    let seeds = just("seeds: ").ignore_then(seed_range.repeated());

    let map_label = filter(|&c: &char| c.is_ascii_alphabetic() || c == '-')
        .ignored()
        .repeated()
        .then_ignore(just(" map:"));

    let map_range = int
        .then(int)
        .then(int)
        .map(|((dst_start, src_start), len)| {
            (
                src_start..src_start + len,
                dst_start as i64 - src_start as i64,
            )
        });

    let map = map_label.ignore_then(map_range.repeated().map(|mut ranges| {
        ranges.sort_by_key(|(range, _)| range.start);
        itertools::chain!(
            ranges.first().map(|(first, _)| (0..first.start, 0)),
            ranges
                .iter()
                .cloned()
                .tuple_windows()
                .flat_map(|((r0, o0), (r1, _))| [(r0.clone(), o0), (r0.end..r1.start, 0)]),
            ranges
                .last()
                .into_iter()
                .flat_map(|(last, off)| [(last.clone(), *off), (last.end..(1 << 62), 0)]),
        )
        .filter(|(range, _)| !range.is_empty())
        .collect()
    }));

    seeds.then(map.repeated())
}

fn intersection(mut a: Range<u64>, mut b: Range<u64>) -> Range<u64> {
    if a.start > b.start {
        mem::swap(&mut a, &mut b);
    }

    if b.start < a.end {
        b.start..u64::min(a.end, b.end)
    } else {
        b.start..b.start
    }
}

fn lowest_loc(src: Range<u64>, maps: &[RangeMap]) -> u64 {
    if let [map, tail @ ..] = maps {
        let mut best = u64::MAX;

        for (range, offset) in map {
            let x = intersection(src.clone(), range.clone());
            if !x.is_empty() {
                best = best.min(lowest_loc(
                    (x.start as i64 + offset) as u64..(x.end as i64 + offset) as u64,
                    tail,
                ));
            }
        }

        best
    } else {
        src.start
    }
}

fn main() {
    let (seeds, maps) = parser().parse(read_stdin_to_string()).unwrap();

    let result = seeds
        .into_iter()
        .map(|seed| lowest_loc(seed, &maps))
        .min()
        .unwrap();

    println!("{result}");
}
