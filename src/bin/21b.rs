#![feature(let_chains)]

use std::collections::VecDeque;

use aoc23::{bounded_offset, matrix, read_stdin_to_string};
use enum_map::Enum;
use nalgebra::DMatrix;
use rustc_hash::FxHashMap;
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

#[derive(Debug, Clone, Copy, Enum)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

impl Edge {
    fn is_on(self, pos: (usize, usize), size: (usize, usize)) -> bool {
        match self {
            Edge::Top => pos.0 == 0,
            Edge::Bottom => pos.0 == size.0 - 1,
            Edge::Left => pos.1 == 0,
            Edge::Right => pos.1 == size.1 - 1,
        }
    }
}

type Map = DMatrix<Tile>;

#[derive(Debug, Clone, Copy)]
struct Pos {
    chunk: (i32, i32),
    tile: (i32, i32),
}

impl Pos {
    fn offset(self, delta: (i32, i32), size: (usize, usize)) -> Pos {
        Pos {
            chunk: (
                self.chunk.0 + (self.tile.0 + delta.0).div_euclid(size.0 as i32),
                self.chunk.1 + (self.tile.1 + delta.1).div_euclid(size.1 as i32),
            ),
            tile: (
                (self.tile.0 + delta.0).rem_euclid(size.0 as i32),
                (self.tile.1 + delta.1).rem_euclid(size.1 as i32),
            ),
        }
    }
}

enum Chunk {
    Partial(Box<(DMatrix<u32>, u32)>),
    Finished,
}

struct MapState {
    map: Map,
    chunks: FxHashMap<(i32, i32), Chunk>,
    frontier: Vec<Pos>,
    buf: Vec<Pos>,
    depth: u32,
    finished: DMatrix<bool>,
    finished_n: u32,
    n_odd: u64,
    n_even: u64,
}

impl MapState {
    fn get(&self, pos: Pos) -> (Tile, bool) {
        let tile_pos = (pos.tile.0 as usize, pos.tile.1 as usize);
        let tile = self.map[tile_pos];

        let visited = match self.chunks.get(&pos.chunk) {
            Some(Chunk::Partial(chunk)) => chunk.0[tile_pos] != 0,
            Some(Chunk::Finished) => self.finished[tile_pos],
            None => false,
        };

        (tile, visited)
    }

    fn set(&mut self, pos: Pos, val: u32) {
        //println!("set({pos:?}, {val}");

        let chunk = self.chunks.entry(pos.chunk).or_insert_with(|| {
            Chunk::Partial(Box::new((
                DMatrix::zeros(self.map.nrows(), self.map.ncols()),
                0,
            )))
        });
        match chunk {
            Chunk::Partial(p_chunk) => {
                p_chunk.1 += 1;
                if p_chunk.1 == self.finished_n {
                    *chunk = Chunk::Finished;
                } else {
                    p_chunk.0[(pos.tile.0 as usize, pos.tile.1 as usize)] = val;
                }
            }
            Chunk::Finished => unreachable!(),
        }
    }

    fn step(&mut self) {
        self.depth += 1;

        let mut frontier = std::mem::take(&mut self.frontier);
        for from in frontier.drain(..) {
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let neighbor = from.offset(dir, self.map.shape());
                let (tile, visited) = self.get(neighbor);
                if !visited && tile == Tile::Empty {
                    self.set(neighbor, self.depth + 1);
                    self.buf.push(neighbor);
                    if self.depth % 2 == 0 {
                        self.n_even += 1;
                    } else {
                        self.n_odd += 1;
                    }
                }
            }
        }

        self.frontier = std::mem::take(&mut self.buf);
        self.buf = frontier;
    }
}

fn find_reachable(map: &Map) -> (u32, DMatrix<bool>) {
    let mut visited = map.map(|_| false);
    let mut frontier = VecDeque::new();
    let mut count = 1;

    frontier.push_back((0, 0));
    visited[(0, 0)] = true;

    while let Some(pos) = frontier.pop_front() {
        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(neighbor) = bounded_offset(pos, dir, map.shape())
                && !visited[neighbor]
                && map[neighbor] == Tile::Empty
            {
                frontier.push_back(neighbor);
                visited[neighbor] = true;
                count += 1;
            }
        }
    }

    (count, visited)
}

fn solve_b(map: &Map, start: Pos, steps: u32) -> u64 {
    let (finished_n, finished) = find_reachable(map);

    //dbg!(finished_n);

    let mut state = MapState {
        map: map.clone(),
        chunks: FxHashMap::default(),
        frontier: vec![start],
        buf: vec![],
        depth: 0,
        finished,
        finished_n,
        n_odd: 0,
        n_even: 0,
    };

    state.set(start, 1);
    state.n_even += 1;

    for _ in 0..steps {
        //dbg!(depth);
        state.step();
        //dbg!(state.n_even, state.n_odd);
    }

    if steps % 2 == 0 {
        state.n_even
    } else {
        state.n_odd
    }
}

fn main() {
    let input = read_stdin_to_string();
    let (map, start) = parser.parse(input.as_str()).unwrap();
    let start = Pos {
        chunk: (0, 0),
        tile: (start.0 as i32, start.1 as i32),
    };

    // println!("{map}");
    // println!("{start:?}");

    // for n in [6, 64] {
    //     println!("a({n}): {}", solve_a(&map, start, n));
    // }

    for n in [6, 10, 50, 100, 500, 1000, 5000] {
        println!("b({n}): {}", solve_b(&map, start, n));
    }
}

fn parser(input: &mut &str) -> PResult<(Map, (usize, usize))> {
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
    let (map, start) = parser.parse(include_str!("../../in/21/ex")).unwrap();
    let ex_start = Pos {
        chunk: (0, 0),
        tile: (start.0 as i32, start.1 as i32),
    };

    assert_eq!(solve_b(&map, ex_start, 6), 16);
    assert_eq!(solve_b(&map, ex_start, 10), 50);
    assert_eq!(solve_b(&map, ex_start, 50), 1594);
    assert_eq!(solve_b(&map, ex_start, 100), 6536);
    assert_eq!(solve_b(&map, ex_start, 500), 167004);
    assert_eq!(solve_b(&map, ex_start, 1000), 668697);
    assert_eq!(solve_b(&map, ex_start, 5000), 16733044);
}
