use aoc23::{chumsky_err, parse_stdin, to_array2};
use chumsky::{prelude::*, text::newline};
use derive_more::Display;
use ndarray::Array2;

#[derive(Debug, Display, Clone, Copy)]
enum Tile {
    #[display(fmt = ".")]
    Empty,
    #[display(fmt = "#")]
    Square,
    #[display(fmt = "O")]
    Round,
}

fn parser() -> impl Parser<char, Array2<Tile>, Error = Simple<char>> {
    let tile = choice((
        just('.').to(Tile::Empty),
        just('#').to(Tile::Square),
        just('O').to(Tile::Round),
    ));
    let row = tile.repeated().at_least(1).then_ignore(newline());
    row.repeated()
        .at_least(1)
        .map(to_array2)
        .try_map(chumsky_err)
        .padded()
        .then_ignore(end())
}

fn tilt_north(grid: &mut Array2<Tile>) {
    for mut col in grid.columns_mut() {
        let mut tail = col.len();
        for head in (0..col.len()).rev() {
            match col[head] {
                Tile::Empty => {
                    tail -= 1;
                    col[head] = Tile::Round;
                    col[tail] = Tile::Empty;
                }
                Tile::Square => tail = head,
                Tile::Round => (),
            }
        }
    }
}

fn calc_load(grid: &Array2<Tile>) -> u64 {
    let mut load = 0;
    for col in grid.columns() {
        for (i, tile) in col.iter().enumerate() {
            match tile {
                Tile::Round => load += (col.len() - i) as u64,
                _ => (),
            }
        }
    }
    load
}

fn main() {
    let mut grid = parse_stdin(parser());
    tilt_north(&mut grid);
    let load = calc_load(&grid);
    println!("{load}");
}
