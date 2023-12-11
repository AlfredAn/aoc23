use aoc23::read_stdin_to_string;
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;

const EXPANSION: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Galaxy,
}

fn parser() -> impl Parser<char, Vec<Vec<Square>>, Error = Simple<char>> {
    let square = choice((just('.').to(Square::Empty), just('#').to(Square::Galaxy)));
    let row = square.repeated().then_ignore(newline());
    row.repeated().then_ignore(end())
}

fn main() {
    let grid = parser().parse(read_stdin_to_string()).unwrap();

    let (w, h) = (grid[0].len(), grid.len());

    let mut galaxies = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().filter_map(move |(x, sq)| {
                if sq == Square::Galaxy {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect_vec();

    let mut cols = vec![false; w];
    let mut rows = vec![false; h];

    for &(x, y) in &galaxies {
        cols[x] = true;
        rows[y] = true;
    }

    let mut col_map = Vec::new();
    let mut x = 0;
    for col_occupied in cols {
        col_map.push(x);
        if col_occupied {
            x += 1;
        } else {
            x += EXPANSION;
        }
    }

    let mut row_map = Vec::new();
    let mut y = 0;
    for row_occupied in rows {
        row_map.push(y);
        if row_occupied {
            y += 1;
        } else {
            y += EXPANSION;
        }
    }

    for (x, y) in &mut galaxies {
        *x = col_map[*x];
        *y = row_map[*y];
    }

    let mut sum = 0;
    for (i, &(x1, y1)) in galaxies.iter().enumerate() {
        for &(x2, y2) in &galaxies[i + 1..] {
            let dist = x1.abs_diff(x2) + y1.abs_diff(y2);
            sum += dist;
        }
    }

    println!("{sum}");
}
