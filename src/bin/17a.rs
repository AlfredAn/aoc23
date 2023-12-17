#![feature(map_try_insert)]
#![feature(let_chains)]

use std::{cmp::Reverse, collections::BinaryHeap};

use aoc23::{bounded_offset, matrix, read_stdin_to_string};
use nalgebra::DMatrix;
use rustc_hash::FxHashMap;
use winnow::{prelude::*, token::any};

fn tile(input: &mut &str) -> PResult<u8> {
    any.verify_map(|c: char| {
        if c.is_ascii_digit() {
            Some(c as u8 - b'0')
        } else {
            None
        }
    })
    .parse_next(input)
}

type Pos = (usize, usize);
type Dir = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pos: Pos,
    chain: Option<(Dir, u8)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HeapNode {
    cost: Reverse<u32>,
    node: Node,
    prev: Option<Node>,
}

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn neg((dy, dx): Dir) -> Dir {
    (-dy, -dx)
}

fn dijkstra(start: Pos, end: Pos, grid: &DMatrix<u8>) -> Option<u32> {
    let mut visited = FxHashMap::<Node, (u32, Option<Node>)>::default();
    let mut frontier = BinaryHeap::<HeapNode>::default();

    frontier.push(HeapNode {
        cost: Reverse(0),
        node: Node {
            pos: start,
            chain: None,
        },
        prev: None,
    });

    let mut result = None;
    while let Some(HeapNode { cost, node, prev }) = frontier.pop() {
        let cost = cost.0;

        if visited.try_insert(node, (cost, prev)).is_err() {
            continue;
        }

        if node.pos == end {
            result = Some((cost, node, prev));
            break;
        }

        for dir in DIRS {
            let Some(n_pos) = bounded_offset(node.pos, dir, grid.shape()) else {
                continue;
            };
            if let Some((last_dir, _)) = node.chain
                && dir == neg(last_dir)
            {
                continue;
            }
            let n_node = Node {
                pos: n_pos,
                chain: if let Some((last_dir, last_consec)) = node.chain
                    && dir == last_dir
                {
                    if last_consec < 3 {
                        Some((dir, last_consec + 1))
                    } else {
                        continue;
                    }
                } else {
                    Some((dir, 1))
                },
            };
            if visited.contains_key(&n_node) {
                continue;
            };
            frontier.push(HeapNode {
                cost: Reverse(cost + grid[n_pos] as u32),
                node: n_node,
                prev: Some(node),
            });
        }
    }

    result.map(|(cost, _, _)| cost)
}

fn main() {
    let input = read_stdin_to_string();
    let grid = matrix(tile).parse(input.as_str()).unwrap();

    let result = dijkstra((0, 0), (grid.nrows() - 1, grid.ncols() - 1), &grid).unwrap();
    println!("{result}");
}
