use std::{fmt::Debug, iter::zip};

use aoc23::{chumsky_err, read_stdin_to_string, to_array2};
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Ash,
    Rock,
}

impl Square {
    fn swap(&mut self) {
        *self = match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

fn parser() -> impl Parser<char, Vec<Array2<Square>>, Error = Simple<char>> {
    let square = choice((just('.').to(Square::Ash), just('#').to(Square::Rock)));
    let row = square.repeated().at_least(1).then_ignore(newline());
    let grid = row
        .repeated()
        .at_least(1)
        .then_ignore(newline().or_not())
        .map(to_array2)
        .try_map(chumsky_err);
    grid.repeated().at_least(1).then_ignore(end())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reflection {
    MirrorX(usize),
    MirrorY(usize),
}

fn reflections(grid: &Array2<Square>) -> impl Iterator<Item = Reflection> + Debug + '_ {
    let (h, w) = grid.dim();

    let xx = (1..w)
        .filter(move |&x_piv| {
            zip((0..x_piv).rev(), x_piv..w).all(|(x1, x2)| grid.column(x1) == grid.column(x2))
        })
        .map(Reflection::MirrorX);

    let yy = (1..h)
        .filter(move |&y_piv| {
            zip((0..y_piv).rev(), y_piv..h).all(|(y1, y2)| (grid.row(y1) == grid.row(y2)))
        })
        .map(Reflection::MirrorY);

    itertools::chain(xx, yy)
}

fn main() {
    let grids = parser().parse(read_stdin_to_string()).unwrap();

    let mut sum = 0;
    for mut grid in grids.into_iter() {
        let (h, w) = grid.dim();
        let original = reflections(&grid).exactly_one().unwrap();

        let new = (0..h)
            .cartesian_product(0..w)
            .filter_map(|(y, x)| {
                grid[(y, x)].swap();
                let res = reflections(&grid)
                    .filter(|&r| r != original)
                    .exactly_one()
                    .ok();
                grid[(y, x)].swap();
                res
            })
            .dedup()
            .exactly_one()
            .unwrap();

        sum += match new {
            Reflection::MirrorX(x) => x,
            Reflection::MirrorY(y) => 100 * y,
        };
    }

    println!("{sum}");
}
