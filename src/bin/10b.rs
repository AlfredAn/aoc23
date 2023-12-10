use std::{collections::VecDeque, iter, ops::Range};

use aoc23::{arr_as, read_stdin_to_string};
use arrayvec::ArrayVec;
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;
use vecmath::{vec2_add, vec2_sub};

const NORTH: [isize; 2] = [-1, 0];
const SOUTH: [isize; 2] = [1, 0];
const EAST: [isize; 2] = [0, 1];
const WEST: [isize; 2] = [0, -1];

fn connections(pipe: u8) -> &'static [[isize; 2]] {
    match pipe {
        b'|' => &[NORTH, SOUTH],
        b'-' => &[EAST, WEST],
        b'L' => &[NORTH, EAST],
        b'J' => &[NORTH, WEST],
        b'7' => &[SOUTH, WEST],
        b'F' => &[SOUTH, EAST],
        b'S' => &[NORTH, SOUTH, EAST, WEST],
        b'.' => &[],
        _ => unreachable!(),
    }
}

fn parser() -> impl Parser<char, Array2<u8>, Error = Simple<char>> {
    let square = one_of("|-LJ7F.S").map(|c| c as u8);

    let row = square.repeated().at_least(1).then_ignore(newline());
    row.repeated()
        .at_least(1)
        .then_ignore(end())
        .try_map(|vv, span: Range<_>| {
            let cols = vv.iter().map(|v| v.len()).all_equal_value().map_err(|e| {
                Simple::custom(
                    span.clone(),
                    format!("inconsistent number of columns: {:?}", e),
                )
            })?;
            let rows = vv.len();
            Array2::from_shape_vec((rows, cols), vv.into_iter().flatten().collect_vec())
                .map_err(|e| Simple::custom(span, format!("couldn't convert to array: {:?}", e)))
        })
}

fn neighbors(grid: &Array2<u8>, pos: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    let (h, w) = grid.dim();
    let dim = [h, w];
    connections(grid[pos])
        .into_iter()
        .map(move |&delta| arr_as(vec2_add(arr_as(pos), delta)))
        .filter(move |neighbor| iter::zip(neighbor, dim).all(|(&n, d)| n < d))
}

fn main() {
    let mut grid = parser().parse(read_stdin_to_string()).unwrap();

    let start = grid
        .indexed_iter()
        .find_map(|(pos, &sq)| if sq == b'S' { Some(pos) } else { None })
        .map(|(y, x)| [y, x])
        .unwrap();

    let start_neighbors = neighbors(&grid, start)
        .filter(|&neighbor| grid[neighbor] != b'.' && neighbors(&grid, neighbor).contains(&start))
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    grid[start] = match start_neighbors.map(|n| vec2_sub(arr_as(n), arr_as(start))) {
        [NORTH, SOUTH] => b'|',
        [EAST, WEST] => b'-',
        [NORTH, EAST] => b'L',
        [NORTH, WEST] => b'J',
        [SOUTH, WEST] => b'7',
        [SOUTH, EAST] => b'F',
        _ => unreachable!(),
    };

    let grid = grid;

    for row in grid.rows() {
        for &x in row.iter() {
            print!("{}", x as char);
        }
        println!();
    }
    println!();

    let mut part_of_loop = Array2::from_elem(grid.dim(), false);
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    while let Some([y, x]) = frontier.pop_front() {
        part_of_loop[[y, x]] = true;

        if grid[[y, x]] != b'.' {
            for neighbor in neighbors(&grid, [y, x]) {
                if !part_of_loop[neighbor] {
                    frontier.push_back(neighbor);
                }
            }
        }
    }

    for (ra, rb) in iter::zip(grid.rows(), part_of_loop.rows()) {
        for (&sq, &visited) in iter::zip(ra.iter(), rb.iter()) {
            if visited {
                print!("X");
            } else {
                print!("{}", sq as char);
            }
        }
        println!();
    }
    println!();

    let mut n_inside = 0;

    let mut inside = Array2::from_elem(grid.dim(), false);

    for (x, col) in grid.columns().into_iter().enumerate() {
        let mut l_inside = false;
        let mut r_inside = false;
        for (y, &sq) in col.into_iter().enumerate() {
            if sq != b'.' && part_of_loop[[y, x]] {
                match sq {
                    b'|' => {
                        assert_ne!(l_inside, r_inside);
                    }
                    b'-' => {
                        assert_eq!(l_inside, r_inside);
                        l_inside = !l_inside;
                        r_inside = !r_inside;
                    }
                    b'L' => {
                        assert_ne!(l_inside, r_inside);
                        r_inside = !r_inside;
                    }
                    b'J' => {
                        assert_ne!(l_inside, r_inside);
                        l_inside = !l_inside;
                    }
                    b'7' => {
                        assert_eq!(l_inside, r_inside);
                        l_inside = !l_inside;
                    }
                    b'F' => {
                        assert_eq!(l_inside, r_inside);
                        r_inside = !r_inside;
                    }
                    _ => unreachable!(),
                }
            }

            if l_inside && r_inside && !part_of_loop[[y, x]] {
                n_inside += 1;
                inside[[y, x]] = true;
            }
        }
    }

    for (ra, rb) in iter::zip(grid.rows(), inside.rows()) {
        for (&sq, &inside) in iter::zip(ra.iter(), rb.iter()) {
            if inside {
                print!("X");
            } else {
                print!("{}", sq as char);
            }
        }
        println!();
    }
    println!();

    println!("{n_inside}");
}
