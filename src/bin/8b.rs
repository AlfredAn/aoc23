#![feature(map_try_insert)]

use std::iter::repeat;

use aoc23::read_stdin_to_string;
use chumsky::prelude::*;
use num::Integer;
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

    let tag = filter(|c: &char| c.is_ascii_uppercase() || c.is_ascii_digit())
        .repeated()
        .collect();

    let node = tag
        .then_ignore(just(" = ("))
        .then(tag.then_ignore(just(", ")).then(tag).then_ignore(just(")")))
        .padded();

    directions.then(node.repeated()).then_ignore(end())
}

fn main() {
    let (dirs, nodes) = parser().parse(read_stdin_to_string()).unwrap();

    let tag_to_index: FxHashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, (tag, _))| (&**tag, i))
        .collect();

    let start_nodes = nodes.iter().filter_map(|(tag, _)| {
        if tag.ends_with('A') {
            Some(&**tag)
        } else {
            None
        }
    });

    let mut cycles = Vec::new();

    for start in start_nodes {
        let mut dirs = repeat(()).flat_map(|_| dirs.iter().copied().enumerate());
        let mut pos = start;
        let mut step = 0;
        let mut visited = FxHashMap::default();
        let mut end_indices = Vec::new();

        let (offset, period) = loop {
            let (j, dir) = dirs.next().unwrap();

            if let Err(e) = visited.try_insert((j, pos), step) {
                let &old_step = e.entry.get();
                break (old_step, step - old_step);
            }

            if pos.ends_with('Z') {
                end_indices.push(step);
            }

            let i = tag_to_index[pos];
            pos = match dir {
                Dir::Left => &*nodes[i].1 .0,
                Dir::Right => &*nodes[i].1 .1,
            };

            step += 1;
        };

        end_indices.iter_mut().for_each(|i| *i -= offset);

        let [end_index] = *end_indices else {
            unimplemented!();
        };

        if offset + end_index != period {
            unimplemented!();
        }

        cycles.push(period);
    }

    dbg!(&cycles);

    // x === 0 (mod 15871)
    // x === 0 (mod 21251)
    // x === 0 (mod 16409)
    // x === 0 (mod 11567)
    // x === 0 (mod 18023)
    // x === 0 (mod 14257)

    let lcm = cycles.into_iter().fold(1, |acc, a| acc.lcm(&(a as u64)));

    println!("{lcm}");
}
