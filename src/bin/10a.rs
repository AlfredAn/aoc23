use std::{collections::VecDeque, ops::Range};

use aoc23::read_stdin_to_string;
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Start,
    Pipe([[isize; 2]; 2]),
}

const EAST: [isize; 2] = [0, 1];
const NORTH: [isize; 2] = [-1, 0];
const WEST: [isize; 2] = [0, -1];
const SOUTH: [isize; 2] = [1, 0];

impl Square {
    fn deltas(&self) -> impl Iterator<Item = [isize; 2]> + '_ {
        match self {
            Square::Empty => [].iter(),
            Square::Start => [[1, 0], [0, 1], [-1, 0], [0, -1]].iter(),
            Square::Pipe(dirs) => dirs.iter(),
        }
        .copied()
    }
}

fn parser() -> impl Parser<char, Array2<Square>, Error = Simple<char>> {
    let square = choice((
        just('.').to(Square::Empty),
        just('S').to(Square::Start),
        just('|').to(Square::Pipe([NORTH, SOUTH])),
        just('-').to(Square::Pipe([EAST, WEST])),
        just('L').to(Square::Pipe([NORTH, EAST])),
        just('J').to(Square::Pipe([NORTH, WEST])),
        just('7').to(Square::Pipe([SOUTH, WEST])),
        just('F').to(Square::Pipe([SOUTH, EAST])),
    ));

    let row = square.repeated().at_least(1).then_ignore(newline());
    row.repeated()
        .at_least(1)
        .then_ignore(end())
        .try_map(|vv, span: Range<_>| {
            let cols = vv.iter().map(|v| v.len()).all_equal_value().map_err(|e| {
                Simple::custom(
                    span.clone(),
                    format!("inconsistend number of columns: {:?}", e),
                )
            })?;
            let rows = vv.len();
            Array2::from_shape_vec((rows, cols), vv.into_iter().flatten().collect_vec())
                .map_err(|e| Simple::custom(span, format!("couldn't convert to array: {:?}", e)))
        })
}

fn main() {
    let grid = parser().parse(read_stdin_to_string()).unwrap();

    let start = grid
        .indexed_iter()
        .find_map(|(pos, &sq)| if sq == Square::Start { Some(pos) } else { None })
        .map(|(y, x)| [y, x])
        .unwrap();

    let mut visited = FxHashSet::default();
    let mut frontier = VecDeque::new();
    let mut dists = Array2::from_elem(<[usize; 2]>::try_from(grid.shape()).unwrap(), None);
    frontier.push_back((0, start));
    let mut farthest = 0;

    while let Some((depth, [y, x])) = frontier.pop_front() {
        visited.insert([y, x]);
        let sq = grid[[y, x]];

        if sq == Square::Empty {
            continue;
        }
        dists[[y, x]] = Some(depth);

        farthest = usize::max(farthest, depth);

        for [dy, dx] in sq.deltas() {
            let neighbor_pos = [(y as isize + dy) as usize, (x as isize + dx) as usize];

            if let Some(neighbor) = grid.get(neighbor_pos) {
                if sq == Square::Start && !neighbor.deltas().contains(&[-dy, -dx]) {
                    continue;
                }

                if !visited.contains(&neighbor_pos) {
                    frontier.push_back((depth + 1, neighbor_pos));
                }
            }
        }
    }

    println!("{farthest}");
}
