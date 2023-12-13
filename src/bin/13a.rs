use std::iter::zip;

use aoc23::{chumsky_err, read_stdin_to_string, to_array2};
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Ash,
    Rock,
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

enum Reflection {
    MirrorX(usize),
    MirrorY(usize),
}

fn reflections(grid: &Array2<Square>) -> impl Iterator<Item = Reflection> + '_ {
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
    for grid in grids.into_iter() {
        sum += match reflections(&grid).exactly_one() {
            Ok(Reflection::MirrorX(x)) => x,
            Ok(Reflection::MirrorY(y)) => 100 * y,
            Err(_) => unreachable!(),
        }
    }

    println!("{sum}");
}
