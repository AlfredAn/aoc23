use std::{
    collections::{BTreeMap, BTreeSet},
    iter,
};

use aoc23::read_stdin_to_string;
use derive_more::Display;
use itertools::Itertools;
use nalgebra::{DMatrix, Vector2};
use rustc_hash::FxHashSet;
use winnow::{
    ascii::{dec_uint, line_ending, space0, space1},
    combinator::{delimited, dispatch, fail, preceded, repeat, success},
    prelude::*,
    stream::AsChar,
    token::{any, take_while},
};

#[derive(Debug, Display, Clone, Copy)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    fn to_vec(self) -> Vector2<i64> {
        match self {
            Dir::Right => Vector2::new(1, 0),
            Dir::Left => Vector2::new(-1, 0),
            Dir::Down => Vector2::new(0, 1),
            Dir::Up => Vector2::new(0, -1),
        }
    }
}

#[derive(Debug, Display, Clone)]
#[display(fmt = "{dir} {len} ({color})")]
struct Instruction {
    dir: Dir,
    len: u64,
    color: String,
}

fn dir(input: &mut &str) -> PResult<Dir> {
    dispatch! {any;
        'D' => success(Dir::Down),
        'U' => success(Dir::Up),
        'R' => success(Dir::Right),
        'L' => success(Dir::Left),
        _ => fail
    }
    .parse_next(input)
}

fn color(input: &mut &str) -> PResult<String> {
    preceded('#', take_while(6, AsChar::is_hex_digit))
        .map(String::from)
        .parse_next(input)
}

fn line(input: &mut &str) -> PResult<Instruction> {
    (
        preceded(space0, dir),
        preceded(space1, dec_uint),
        delimited((space1, '('), color, (')', space0, line_ending)),
    )
        .map(|(dir, len, color)| Instruction { dir, len, color })
        .parse_next(input)
}

fn solve_a(lines: impl IntoIterator<Item = (Dir, u64)>) -> u64 {
    let mut dug = FxHashSet::default();
    let mut pos = Vector2::zeros();

    dug.insert(pos);

    for (dir, len) in lines {
        for _ in 0..len {
            pos += dir.to_vec();
            dug.insert(pos);
        }
    }

    let (xmin, xmax) = dug.iter().map(|v| v.x).minmax().into_option().unwrap();
    let (ymin, ymax) = dug.iter().map(|v| v.y).minmax().into_option().unwrap();

    let matrix = DMatrix::from_fn(
        (ymax - ymin + 1) as usize,
        (xmax - xmin + 1) as usize,
        |y, x| {
            if dug.contains(&Vector2::new(x as i64 + xmin, y as i64 + ymin)) {
                '#'
            } else {
                '.'
            }
        },
    );

    let mut total = 0u64;
    for y in ymin..=ymax {
        let mut above_inside = false;
        let mut below_inside = false;

        let mut row = 0u64;

        for x in xmin..=xmax {
            let [edge_here, edge_above, edge_below] =
                [(x, y), (x, y - 1), (x, y + 1)].map(|(x, y)| dug.contains(&Vector2::new(x, y)));

            if edge_here {
                above_inside ^= edge_above;
                below_inside ^= edge_below;
            }

            if edge_here || above_inside || below_inside {
                row += 1;
            }
        }
        total += row;
    }

    total
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hit {
    Corner(Corner),
    Flat,
}

fn solve_b(lines: &[(Dir, u64)]) -> u64 {
    let corners = lines
        .into_iter()
        .circular_tuple_windows()
        .scan(Vector2::zeros(), |pos, (&(dir1, len), &(dir2, _))| {
            *pos += dir1.to_vec() * (len as i64);

            let corner = match (dir1, dir2) {
                (Dir::Up, Dir::Right) | (Dir::Left, Dir::Down) => Corner::TopLeft,
                (Dir::Up, Dir::Left) | (Dir::Right, Dir::Down) => Corner::TopRight,
                (Dir::Down, Dir::Right) | (Dir::Left, Dir::Up) => Corner::BottomLeft,
                (Dir::Down, Dir::Left) | (Dir::Right, Dir::Up) => Corner::BottomRight,
                _ => unreachable!(),
            };

            Some(((pos.x, pos.y), corner))
        })
        .sorted_unstable_by_key(|&(pos, _)| pos);

    let mut h_edges = BTreeSet::new();
    let mut v_edges = BTreeMap::new();

    let group_by = corners.group_by(|&((x, _), _)| x);
    let sums = group_by.into_iter().map(|(x, column)| {
        column.for_each(|((_, y), corner)| {
            match corner {
                Corner::TopLeft | Corner::BottomLeft => h_edges.insert(y),
                Corner::TopRight | Corner::BottomRight => h_edges.remove(&y),
            };
            v_edges.insert(y, corner);
        });

        let mut left_inside = false;
        let mut right_inside = false;

        let mut left_sum = 0;
        let mut right_sum = 0;

        let mut y_prev = None;

        h_edges
            .iter()
            .map(|&y| (y, Hit::Flat))
            .merge(v_edges.iter().map(|(&y, &corner)| (y, Hit::Corner(corner))))
            .dedup_by(|(y1, _), (y2, _)| y1 == y2)
            .for_each(|(y, hit): (i64, Hit)| {
                let dy = y_prev.map_or(1, |y_prev| y - y_prev);
                let was_inside = left_inside || right_inside;
                let was_right_inside = right_inside;

                match hit {
                    Hit::Corner(Corner::TopLeft | Corner::BottomLeft) => {
                        right_inside = !right_inside;
                    }
                    Hit::Corner(Corner::TopRight | Corner::BottomRight) => {
                        left_inside = !left_inside;
                    }
                    Hit::Flat => {
                        left_inside = !left_inside;
                        right_inside = !right_inside;
                    }
                }

                let is_inside = left_inside || right_inside;

                if was_inside {
                    left_sum += dy;
                } else if is_inside {
                    left_sum += 1;
                }

                if was_right_inside {
                    right_sum += dy;
                } else if right_inside {
                    right_sum += 1;
                }

                y_prev = Some(y);
            });

        v_edges.clear();

        (x, (left_sum, right_sum))
    });

    itertools::chain(sums.map(Some), iter::once(None))
        .tuple_windows()
        .map(|(a, b)| {
            let (x1, (left_sum, right_sum)) = a.unwrap();
            let dx = if let Some((x2, _)) = b { x2 - x1 } else { 1 };

            let partial_sum = left_sum + (dx - 1) * right_sum;
            //dbg!(partial_sum);

            partial_sum as u64
        })
        .sum::<u64>()
}

fn main() {
    let input = read_stdin_to_string();
    let lines: Vec<_> = repeat(.., line).parse(input.as_str()).unwrap();

    let a_lines = || lines.iter().map(|&Instruction { dir, len, .. }| (dir, len));
    let b_lines = || {
        lines.iter().map(|Instruction { color, .. }| {
            let (a, b) = color.split_at(5);
            (
                match b {
                    "0" => Dir::Right,
                    "1" => Dir::Down,
                    "2" => Dir::Left,
                    "3" => Dir::Up,
                    _ => unreachable!(),
                },
                u64::from_str_radix(a, 16).unwrap(),
            )
        })
    };

    let a = solve_a(a_lines());
    println!("a: {a}");

    let b = solve_b(&b_lines().collect_vec());
    println!("b: {b}");
}
