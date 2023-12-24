use std::ops::RangeBounds;

use aoc23::read_stdin_to_string;
use nalgebra::{vector, Matrix2, Vector2, Vector3};
use num::{rational::Ratio, Zero};
use winnow::{
    ascii::{dec_int, line_ending, space0},
    combinator::{delimited, repeat},
    prelude::*,
    seq,
};

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Vector3<i128>,
    vel: Vector3<i128>,
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
    t_a: Ratio<i128>,
    t_b: Ratio<i128>,
    pos: Vector2<Ratio<i128>>,
}

impl Hailstone {
    fn intersect(&self, other: &Self) -> Option<Intersection> {
        let a = Matrix2::from_columns(&[self.vel.xy(), -other.vel.xy()]);
        let b = other.pos.xy() - self.pos.xy();

        let det = a[(0, 0)] * a[(1, 1)] - a[(0, 1)] * a[(1, 0)];

        if det == 0 {
            if b.is_zero() {
                return Some(Intersection {
                    t_a: 0.into(),
                    t_b: 0.into(),
                    pos: self.pos.xy().map(Ratio::from),
                });
            } else {
                return None;
            }
        }

        #[rustfmt::skip]
        let a_inv = Matrix2::new(
             a[(1, 1)], -a[(0, 1)],
            -a[(1, 0)],  a[(0, 0)],
        ).map(|a_ij| Ratio::new(a_ij, det));

        let b = b.map(Ratio::from);
        let x = a_inv * b;

        let pos = self.pos.xy().map(Ratio::from) + self.vel.xy().map(Ratio::from) * x[0];

        Some(Intersection {
            t_a: x[0],
            t_b: x[1],
            pos,
        })
    }
}

fn solve_a(stones: &[Hailstone], bounds: impl RangeBounds<Ratio<i128>>) -> usize {
    (0..stones.len())
        .flat_map(|i| (i + 1..stones.len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| stones[i].intersect(&stones[j]))
        .filter(|&Intersection { t_a, t_b, pos }| {
            t_a >= Ratio::zero()
                && t_b >= Ratio::zero()
                && bounds.contains(&pos[0])
                && bounds.contains(&pos[1])
        })
        .count()
}

fn main() {
    let stones = parser.parse(read_stdin_to_string().as_str()).unwrap();

    let a_small = solve_a(&stones, Ratio::from(7)..=Ratio::from(27));
    println!("a_small: {a_small}");

    let a_large = solve_a(
        &stones,
        Ratio::from(200000000000000)..=Ratio::from(400000000000000),
    );
    println!("a_large: {a_large}");
}

fn parser(input: &mut &str) -> PResult<Vec<Hailstone>> {
    let int = || delimited(space0, dec_int, space0);
    let vec3 = || {
        {
            seq!(
                int(),
                _: ',',
                int(),
                _: ',',
                int(),
            )
        }
        .map(|(x, y, z)| vector![x, y, z])
    };
    let hailstone = seq!(Hailstone {
        pos: vec3(),
        _: '@',
        vel: vec3(),
        _: line_ending,
    });
    let mut parser = repeat(1.., hailstone);
    parser.parse_next(input)
}

#[cfg(test)]
#[test]
fn test() {
    let ex = parser.parse(include_str!("../../in/24/ex")).unwrap();
    let i = parser.parse(include_str!("../../in/24/i")).unwrap();

    assert_eq!(solve_a(&ex, Ratio::from(7)..=Ratio::from(27)), 2);
    assert_eq!(
        solve_a(
            &i,
            Ratio::from(200000000000000)..=Ratio::from(400000000000000)
        ),
        11098
    );
}
