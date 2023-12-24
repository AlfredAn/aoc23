use std::collections::VecDeque;

use aoc23::{bounded_offset, matrix, read_stdin_to_string};
use itertools::Itertools;
use nalgebra::DMatrix;
use strum::{Display, EnumIter, IntoEnumIterator};
use winnow::{
    combinator::{dispatch, fail, success},
    prelude::*,
    token::any,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
enum Tile {
    Empty,    
    Blocked,
    Slope(Dir),
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
enum TileB {
    #[strum(serialize=".")] Empty,
    #[strum(serialize="#")] Blocked,
}

impl From<Tile> for TileB {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Blocked => Self::Blocked,
            _ => Self::Empty,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Blocked => write!(f, "#"),
            Tile::Slope(dir) => write!(f, "{dir}"),
        }
    }
}

#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
enum Dir {
    #[strum(serialize="v")] Down,
    #[strum(serialize="^")] Up,
    #[strum(serialize=">")] Right,
    #[strum(serialize="<")] Left,
}

impl Dir {
    fn delta(self) -> (isize, isize) {
        match self {
            Dir::Down => (1, 0),
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
        }
    }
    fn reverse(self) -> Self {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
        }
    }
}

fn find_start_and_end(map: &DMatrix<Tile>) -> ((usize, usize), (usize, usize)) {
    let [start, end] = [0, map.nrows() - 1].map(|row| {
        (
            row,
            map.row(row)
                .iter()
                .positions(|&tile| tile == Tile::Empty)
                .exactly_one()
                .unwrap(),
        )
    });
    (start, end)
}

fn solve_a(map: &DMatrix<Tile>) -> u32 {
    println!("{map}");

    let (start, end) = find_start_and_end(map);

    let mut frontier = VecDeque::new();
    let mut longest = map.map(|_| 0u32);

    frontier.push_back((start, 0, Dir::Down));

    while let Some((cur, longest_when_added, last_dir)) = frontier.pop_front() {
        if longest[cur] > longest_when_added {
            continue;
        }

        Dir::iter()
            .filter(|&dir| {
                dir != last_dir.reverse() && [Tile::Empty, Tile::Slope(dir)].contains(&map[cur])
            })
            .filter_map(|dir| {
                bounded_offset(cur, dir.delta(), map.shape()).map(|neighbor| (neighbor, dir))
            })
            .for_each(|(neighbor, dir)| {
                let dist = longest[cur] + 1;
                if dist > longest[neighbor] {
                    longest[neighbor] = dist;
                    frontier.push_back((neighbor, dist, dir));
                }
            });
    }

    longest[end]
}

#[derive(Debug)]
enum Node {
    Backtrack((usize, usize)),
    Visit((usize, usize)),
}

fn solve_b(map: &DMatrix<Tile>) -> u32 {
    let (start, end) = find_start_and_end(map);

    let map = map.map(TileB::from);
    println!("{map}");

    let mut visited = map.map(|_| false);
    let mut stack = vec![Node::Visit(start)];

    let mut dist = 0;
    let mut longest = 0;

    while let Some(node) = stack.pop() {
        match node {
            Node::Backtrack(cur) => {
                visited[cur] = false;
                dist -= 1;
            }
            Node::Visit(cur) => {
                visited[cur] = true;
                dist += 1;
                stack.push(Node::Backtrack(cur));

                if cur == end {
                    if dist > longest {
                        println!("{longest}");
                    }
                    longest = longest.max(dist);
                } else {
                    stack.extend(
                        Dir::iter()
                            .filter_map(|dir| bounded_offset(cur, dir.delta(), map.shape()))
                            .filter(|&neighbor| !visited[neighbor] && map[neighbor] == TileB::Empty)
                            .map(Node::Visit),
                    );
                }
            }
        }
    }

    longest - 1
}

fn main() {
    let map = parser.parse(read_stdin_to_string().as_str()).unwrap();
    let a = solve_a(&map);
    println!("a: {a}");

    let b = solve_b(&map);
    println!("b: {b}");
}

fn parser(input: &mut &str) -> PResult<DMatrix<Tile>> {
    let tile = dispatch!(any;
        '.' => success(Tile::Empty),
        '#' => success(Tile::Blocked),
        'v' => success(Tile::Slope(Dir::Down)),
        '^' => success(Tile::Slope(Dir::Up)),
        '>' => success(Tile::Slope(Dir::Right)),
        '<' => success(Tile::Slope(Dir::Left)),
        _ => fail,
    );
    matrix(tile).parse_next(input)
}

#[cfg(test)]
#[test]
fn test() {
    let ex = parser.parse(include_str!("../../in/23/ex")).unwrap();
    let i = parser.parse(include_str!("../../in/23/i")).unwrap();

    assert_eq!(solve_a(&ex), 94);
    assert_eq!(solve_a(&i), 2414);

    assert_eq!(solve_b(&ex), 154);
    assert_eq!(solve_b(&i), 6598);
}
