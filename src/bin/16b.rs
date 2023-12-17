use aoc23::{matrix, read_stdin_to_string};
use nalgebra::DMatrix;
use rustc_hash::FxHashSet;
use std::fmt;
use strum::FromRepr;
use winnow::{token::any, PResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(u8)]
enum Tile {
    Empty = b'.',
    Vertical = b'|',
    Horizontal = b'-',
    Diag1 = b'/',
    Diag2 = b'\\',
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8 as char)
    }
}

fn tile(input: &mut &str) -> PResult<Tile> {
    any.verify_map(|c| u8::try_from(c).ok().and_then(Tile::from_repr))
        .parse_next(input)
}

fn offset(pos: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        usize::try_from(pos.0 as isize + delta.0).ok()?,
        usize::try_from(pos.1 as isize + delta.1).ok()?,
    ))
}

fn shoot_beam(
    grid: &DMatrix<Tile>,
    start_pos: (usize, usize),
    start_dir: (isize, isize),
) -> DMatrix<bool> {
    let mut beams = vec![(start_pos, start_dir)];
    let mut next = Vec::new();
    let mut seen = FxHashSet::default();
    let mut energized = DMatrix::repeat(grid.nrows(), grid.ncols(), false);

    while !beams.is_empty() {
        for (pos, dir) in beams.drain(..) {
            if !seen.insert((pos, dir)) {
                continue;
            }

            energized[pos] = true;

            let mut push = |dir| {
                if let Some(pos) = offset(pos, dir) {
                    if pos.0 < grid.nrows() && pos.1 < grid.ncols() {
                        next.push((pos, dir));
                    }
                }
            };

            assert_eq!(dir.0.abs() + dir.1.abs(), 1);
            match (grid[pos], dir) {
                (Tile::Diag1, (dy, dx)) => push((-dx, -dy)),
                (Tile::Diag2, (dy, dx)) => push((dx, dy)),
                (Tile::Vertical, (0, _)) => {
                    push((1, 0));
                    push((-1, 0));
                }
                (Tile::Horizontal, (_, 0)) => {
                    push((0, 1));
                    push((0, -1));
                }
                (_, (dy, dx)) => push((dy, dx)),
            }
        }

        std::mem::swap(&mut beams, &mut next);
    }

    energized
}

fn count_energized(
    grid: &DMatrix<Tile>,
    start_pos: (usize, usize),
    start_dir: (isize, isize),
) -> usize {
    shoot_beam(grid, start_pos, start_dir)
        .into_iter()
        .filter(|&&b| b)
        .count()
}

fn starting_states(grid: &DMatrix<Tile>) -> impl Iterator<Item = ((usize, usize), (isize, isize))> {
    let (m, n) = grid.shape();
    itertools::chain!(
        (0..m).map(move |y| ((y, 0), (0, 1))),
        (0..m).map(move |y| ((y, n - 1), (0, -1))),
        (0..n).map(move |x| ((0, x), (1, 0))),
        (0..n).map(move |x| ((m - 1, x), (-1, 0))),
    )
}

fn main() {
    let input = read_stdin_to_string();
    let grid = matrix(tile).parse(input.as_str()).unwrap();

    let result = starting_states(&grid)
        .map(|(pos, dir)| count_energized(&grid, pos, dir))
        .max()
        .unwrap();

    println!("{result}");
}
