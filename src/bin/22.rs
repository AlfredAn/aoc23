use std::fmt::Debug;

use aoc23::read_stdin_to_string;
use fixedbitset::FixedBitSet;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::repeat,
    prelude::*,
    seq,
};

type Pos = [u16; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick(Pos, Pos);

impl Brick {
    fn iter(self) -> impl Iterator<Item = Pos> {
        let Brick(a, b) = self;
        (a[0]..b[0] + 1).flat_map(move |i| {
            (a[1]..b[1] + 1).flat_map(move |j| (a[2]..b[2] + 1).map(move |k| [i, j, k]))
        })
    }

    fn lower(self) -> Option<Self> {
        // println!("trying to lower");

        let Brick(mut a, mut b) = self;
        if a[2] >= 2 {
            // println!("not at bottom");
            assert!(b[2] >= 2);
            a[2] -= 1;
            b[2] -= 1;
            // println!("new: {:?}", Brick(a, b));
            Some(Brick(a, b))
        } else {
            // println!("at bottom");
            None
        }
    }
}

struct Grid {
    size: [u16; 3],
    stride: [usize; 2],
    a: FixedBitSet,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            stride: self.stride,
            a: self.a.clone(),
        }
    }
}

impl Grid {
    fn new(size: [u16; 3]) -> Self {
        let s2 = size[2] as usize;
        let s1 = size[1] as usize * s2;
        let s0 = size[0] as usize * s1;

        Self {
            size,
            stride: [s1, s2],
            a: FixedBitSet::with_capacity(s0),
        }
    }

    fn i(&self, index: [u16; 3]) -> usize {
        assert!((0..3).all(|i| index[i] < self.size[i]));
        let index = index.map(|x| x as usize);
        let stride = self.stride;
        index[0] * stride[0] + index[1] * stride[1] + index[2]
    }

    fn get(&self, index: [u16; 3]) -> bool {
        self.a[self.i(index)]
    }

    fn set(&mut self, index: [u16; 3], val: bool) {
        self.a.set(self.i(index), val);
    }

    fn can_add_brick(&self, brick: Brick) -> bool {
        brick.iter().all(|pos| !self.get(pos))
    }

    fn add_brick(&mut self, brick: Brick) {
        brick.iter().for_each(|pos| {
            assert!(!self.get(pos));
            self.set(pos, true);
        });
    }

    fn remove_brick(&mut self, brick: Brick) {
        brick.iter().for_each(|pos| {
            assert!(self.get(pos));
            self.set(pos, false);
        });
    }

    fn settle_bricks(
        &mut self,
        bricks: &mut [Brick],
        skip: Option<usize>,
        mut on_move: impl FnMut(usize),
    ) {
        loop {
            let mut changed = false;

            for (i, brick) in bricks.iter_mut().enumerate() {
                if Some(i) == skip {
                    continue;
                }

                self.remove_brick(*brick);

                while let Some(new) = brick.lower() {
                    if self.can_add_brick(new) {
                        *brick = new;
                        changed = true;
                        on_move(i);
                    } else {
                        break;
                    }
                }

                self.add_brick(*brick);
            }

            if !changed {
                break;
            }
        }
    }

    fn is_settled<'a>(&mut self, bricks: impl IntoIterator<Item = &'a Brick>) -> bool {
        for &brick in bricks {
            self.remove_brick(brick);
            if let Some(new) = brick.lower() {
                if self.can_add_brick(new) {
                    self.add_brick(brick);
                    return false;
                }
            }
            self.add_brick(brick);
        }
        true
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in (1..self.size[2]).rev() {
            for y in 0..self.size[1] {
                for x in 0..self.size[0] {
                    write!(f, "{}", if self.get([x, y, z]) { '#' } else { '.' })?
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(mut bricks: Vec<Brick>) -> (u64, u64) {
    let mut max = [0; 3];
    for &Brick(a, b) in &bricks {
        for i in 0..3 {
            max[i] = max[i].max(a[i].max(b[i]));
        }
    }

    let shape = max.map(|x_i| x_i + 1);
    let mut grid = Grid::new(shape);

    for &brick in &bricks {
        grid.add_brick(brick);
    }

    grid.settle_bricks(&mut bricks, None, |_| ());

    let mut a_sum = 0;
    for &brick in &bricks {
        grid.remove_brick(brick);

        if grid.is_settled(bricks.iter().filter(|&&b| b != brick)) {
            a_sum += 1;
        }

        grid.add_brick(brick);
    }

    let mut b_sum = 0;
    let mut would_fall = vec![false; bricks.len()];

    for i in 0..bricks.len() {
        let mut grid = grid.clone();
        let mut bricks = bricks.clone();
        let brick = bricks[i];

        grid.remove_brick(brick);

        grid.settle_bricks(&mut bricks, Some(i), |i| would_fall[i] = true);
        b_sum += would_fall.iter().filter(|&&b| b).count() as u64;
        would_fall.fill(false);
    }

    (a_sum, b_sum)
}

fn main() {
    let input = parser.parse(read_stdin_to_string().as_str()).unwrap();

    let (a, b) = solve(input);
    println!("a: {a}");
    println!("b: {b}");
}

fn parser(input: &mut &str) -> PResult<Vec<Brick>> {
    let pos = || {
        seq!(
            dec_uint,
            _: ',',
            dec_uint,
            _: ',',
            dec_uint,
        )
        .map(|(x, y, z)| [x, y, z])
    };

    let brick = seq!(Brick(
        pos(),
        _: '~',
        pos(),
        _: line_ending
    ))
    .verify(|Brick(a, b)| a <= b);

    repeat(1.., brick).parse_next(input)
}

#[cfg(test)]
#[test]
fn test() {
    let ex = parser.parse(include_str!("../../in/22/ex")).unwrap();
    let i = parser.parse(include_str!("../../in/22/i")).unwrap();

    let (a_ex, b_ex) = solve(ex);
    assert_eq!(a_ex, 5);
    assert_eq!(b_ex, 7);

    let (a_i, b_i) = solve(i);
    assert_eq!(a_i, 482);
    assert_eq!(b_i, 103010);
}
