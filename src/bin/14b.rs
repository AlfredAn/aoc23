use aoc23::{chumsky_err, parse_stdin, to_array2};
use chumsky::{prelude::*, text::newline};
use derive_more::Display;
use ndarray::{Array2, ArrayBase, Axis, Dim, ViewRepr};
use rustc_hash::FxHashMap;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
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

fn tilt_north(grid: &mut ArrayBase<ViewRepr<&mut Tile>, Dim<[usize; 2]>>) {
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

fn rot90(view: &mut ArrayBase<ViewRepr<&mut Tile>, Dim<[usize; 2]>>) {
    view.swap_axes(0, 1);
    view.invert_axis(Axis(1));
}

fn run_cycle(grid: &mut Array2<Tile>) {
    let mut view = grid.view_mut();
    for _ in 0..4 {
        tilt_north(&mut view);
        rot90(&mut view);
    }
}

const ITERS: usize = 1_000_000_000;

fn main() {
    let mut grid = parse_stdin(parser());
    println!("{grid}\n");

    let mut found = FxHashMap::default();
    let mut loads = Vec::new();

    let mut period = None;
    for i in 0..ITERS {
        if let Some(old_i) = found.insert(grid.clone(), i) {
            period = Some(i - old_i);
            break;
        }
        loads.push(calc_load(&grid));

        if i % (1 << 20) == 0 {
            println!("i={} * 2^20", i >> 20);
        }

        run_cycle(&mut grid);
    }

    let result = if let Some(period) = period {
        let start = loads.len();
        println!("period={period}");
        println!("start={start}");

        let cycle = &loads[start-period..start];
        println!("all: {loads:?}");
        println!("cycle: {cycle:?}");

        let cycle_index = (ITERS - start) % period;
        cycle[cycle_index]
    } else {
        *loads.last().unwrap()
    };

    println!("{result}");
}
