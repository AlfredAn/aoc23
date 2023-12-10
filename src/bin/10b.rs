use std::{collections::VecDeque, fmt, iter, ops::Range};

use aoc23::read_stdin_to_string;
use arrayvec::ArrayVec;
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Pipe {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeOrStart {
    Pipe(Pipe),
    Start,
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

const EAST: [isize; 2] = [0, 1];
const NORTH: [isize; 2] = [-1, 0];
const WEST: [isize; 2] = [0, -1];
const SOUTH: [isize; 2] = [1, 0];

impl Pipe {
    fn connections(self) -> [[isize; 2]; 2] {
        match self {
            Pipe::NS => [NORTH, SOUTH],
            Pipe::EW => [EAST, WEST],
            Pipe::NE => [NORTH, EAST],
            Pipe::NW => [NORTH, WEST],
            Pipe::SW => [SOUTH, WEST],
            Pipe::SE => [SOUTH, EAST],
        }
    }

    fn char(self) -> char {
        self as u8 as char
    }
}

fn parser() -> impl Parser<char, Array2<Option<PipeOrStart>>, Error = Simple<char>> {
    let square = choice((
        just('.').to(None),
        just('S').to(Some(PipeOrStart::Start)),
        just('|').to(Some(PipeOrStart::Pipe(Pipe::NS))),
        just('-').to(Some(PipeOrStart::Pipe(Pipe::EW))),
        just('L').to(Some(PipeOrStart::Pipe(Pipe::NE))),
        just('J').to(Some(PipeOrStart::Pipe(Pipe::NW))),
        just('7').to(Some(PipeOrStart::Pipe(Pipe::SW))),
        just('F').to(Some(PipeOrStart::Pipe(Pipe::SE))),
    ));

    let row = square.repeated().at_least(1).then_ignore(newline());
    row.repeated()
        .at_least(1)
        .then_ignore(end())
        .try_map(|vv, span: Range<_>| {
            let cols = vv.iter().map(|v| v.len()).all_equal_value().map_err(|e| {
                Simple::custom(
                    span.clone(),
                    format!("inconsistend number of columns: {:?}", e),
                )
            })?;
            let rows = vv.len();
            Array2::from_shape_vec((rows, cols), vv.into_iter().flatten().collect_vec())
                .map_err(|e| Simple::custom(span, format!("couldn't convert to array: {:?}", e)))
        })
}

fn neighbors_ext(
    dims: [usize; 2],
    pos: [usize; 2],
    deltas: impl IntoIterator<Item = [isize; 2]>,
) -> impl Iterator<Item = [usize; 2]> {
    deltas
        .into_iter()
        .map(move |[dy, dx]| {
            let [y, x] = pos;
            [(y as isize + dy) as usize, (x as isize + dx) as usize]
        })
        .filter(move |neighbor| iter::zip(neighbor, dims).all(|(&n, d)| n < d))
}

fn neighbors(grid: &Array2<Option<Pipe>>, pos: [usize; 2]) -> [[usize; 2]; 2] {
    let (h, w) = grid.dim();
    neighbors_ext([h, w], pos, grid[pos].unwrap().connections())
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap()
}

fn main() {
    let grid = parser().parse(read_stdin_to_string()).unwrap();
    let (h, w) = grid.dim();
    let dim = [h, w];

    let start = grid
        .indexed_iter()
        .find_map(|(pos, &sq)| {
            if sq == Some(PipeOrStart::Start) {
                Some(pos)
            } else {
                None
            }
        })
        .map(|(y, x)| [y, x])
        .unwrap();

    let mut new_grid = grid.map(|x| match x {
        Some(PipeOrStart::Pipe(pipe)) => Some(*pipe),
        Some(PipeOrStart::Start) | None => None,
    });

    let start_neighbors = neighbors_ext(dim, start, [NORTH, SOUTH, EAST, WEST])
        .filter(|&neighbor| {
            new_grid[neighbor].is_some() && neighbors(&new_grid, neighbor).contains(&start)
        })
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    new_grid[start] = Some(
        match start_neighbors.map(|n| [0, 1].map(|i| n[i] as isize - start[i] as isize)) {
            [NORTH, SOUTH] => Pipe::NS,
            [EAST, WEST] => Pipe::EW,
            [NORTH, EAST] => Pipe::NE,
            [NORTH, WEST] => Pipe::NW,
            [SOUTH, WEST] => Pipe::SW,
            [SOUTH, EAST] => Pipe::SE,
            _ => unreachable!(),
        },
    );

    let grid = new_grid;

    for row in grid.rows() {
        for x in row.iter() {
            if let Some(pipe) = x {
                print!("{pipe}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    let mut part_of_loop = Array2::from_elem(dim, false);
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    while let Some([y, x]) = frontier.pop_front() {
        part_of_loop[[y, x]] = true;

        if let Some(sq) = grid[[y, x]] {
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
            } else if let Some(pipe) = sq {
                print!("{pipe}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    let mut n_inside = 0;

    let mut inside = Array2::from_elem(dim, false);

    for (x, col) in grid.columns().into_iter().enumerate() {
        let mut l_inside = false;
        let mut r_inside = false;
        for (y, sq) in col.into_iter().enumerate() {
            if let Some(pipe) = sq {
                if part_of_loop[[y, x]] {
                    match pipe {
                        Pipe::NS => {
                            assert_ne!(l_inside, r_inside);
                        }
                        Pipe::EW => {
                            assert_eq!(l_inside, r_inside);
                            l_inside = !l_inside;
                            r_inside = !r_inside;
                        }
                        Pipe::NE => {
                            assert_ne!(l_inside, r_inside);
                            r_inside = !r_inside;
                        }
                        Pipe::NW => {
                            assert_ne!(l_inside, r_inside);
                            l_inside = !l_inside;
                        }
                        Pipe::SW => {
                            assert_eq!(l_inside, r_inside);
                            l_inside = !l_inside;
                        }
                        Pipe::SE => {
                            assert_eq!(l_inside, r_inside);
                            r_inside = !r_inside;
                        }
                    }
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
            } else if let Some(pipe) = sq {
                print!("{pipe}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    println!("{n_inside}");
}
