use std::ops::Not;

use aoc23::{bounded_offset, matrix, read_stdin_to_string};
use nalgebra::DMatrix;
use strum::Display;
use winnow::{
    combinator::{dispatch, fail, success},
    prelude::*,
    token::any,
};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[strum(to_string = ".")]
    Empty,

    #[strum(to_string = "#")]
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileOrStart {
    Tile(Tile),
    Start,
}

type Map = DMatrix<Tile>;
type Pos = (usize, usize);

fn solve_a(map: &Map, start: Pos, steps: u64) -> usize {
    let (rows, cols) = map.shape();

    let mut depth: DMatrix<Option<u64>> = DMatrix::repeat(rows, cols, None);
    depth[start] = Some(0);

    for d in 1..=steps {
        if d & 1023 == 0 {
            println!("{d}");
        }

        for i in 0..rows {
            for j in 0..cols {
                match depth[(i, j)] {
                    Some(d2) if d2 < d && d2 & 1 != d & 1 => {
                        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            if let Some(neighbor) = bounded_offset((i, j), dir, (rows, cols)) {
                                if depth[neighbor].is_none() && map[neighbor] == Tile::Empty {
                                    depth[neighbor] = Some(d);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // println!(
    //     "{}",
    //     depth.map(|d| d.map_or(format!("."), |d| format!("{d}")))
    // );

    depth
        .iter()
        .filter(|&&d| d.map_or(false, |d| d & 1 == steps & 1))
        .count()
}

fn solve_b(map: &Map, start: Pos, steps: u64) -> u64 {
    todo!()
}

fn main() {
    let input = read_stdin_to_string();
    let (map, start) = parser.parse(input.as_str()).unwrap();

    // println!("{map}");
    // println!("{start:?}");

    for n in [6, 64] {
        println!("a({n}): {}", solve_a(&map, start, n));
    }

    // for n in [6, 10, 50] {
    //     println!("b({n}): {}", solve_b(&map, start, n));
    // }
}

fn parser(input: &mut &str) -> PResult<(Map, Pos)> {
    let tile = dispatch! {any;
        '.' => success(TileOrStart::Tile(Tile::Empty)),
        '#' => success(TileOrStart::Tile(Tile::Rock)),
        'S' => success(TileOrStart::Start),
        _ => fail,
    };
    let map = matrix(tile).parse_next(input)?;

    let mut start = None;
    let map = map.map_with_location(|i, j, tile_or_start| match tile_or_start {
        TileOrStart::Tile(tile) => tile,
        TileOrStart::Start => {
            assert!(start.is_none(), "multiple start locations");
            start = Some((i, j));
            Tile::Empty
        }
    });

    Ok((map, start.expect("no start location")))
}

#[cfg(test)]
#[test]
fn test() {
    let (ex_map, ex_start) = parser.parse(include_str!("../../in/21/ex")).unwrap();
    let (i_map, i_start) = parser.parse(include_str!("../../in/21/i")).unwrap();

    assert_eq!(solve_a(&ex_map, ex_start, 6), 16);
    assert_eq!(solve_a(&i_map, i_start, 64), 3768);
}
