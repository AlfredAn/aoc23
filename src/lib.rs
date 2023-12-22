use std::{fmt, io::Read, ops::Range};

use chumsky::error::Simple;
use itertools::Itertools;
use ndarray::{Array2, ShapeError};
use num::traits::AsPrimitive;
use thiserror::Error;

pub fn read_stdin_to_bytes() -> Vec<u8> {
    let mut buf = Vec::new();
    std::io::stdin().lock().read_to_end(&mut buf).unwrap();
    buf
}

pub fn read_stdin_to_string() -> String {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    buf
}

#[derive(Clone)]
pub struct Display<F>(F);

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Debug for Display<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for Display<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

pub fn display<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>(f: F) -> Display<F> {
    Display(f)
}

pub fn arr_as<A: AsPrimitive<B>, B: 'static + Copy, const N: usize>(a: [A; N]) -> [B; N] {
    a.map(AsPrimitive::as_)
}

#[derive(Error, Debug)]
pub enum ToArrayError {
    #[error("Inconsistent number of columns: {0:?}")]
    ColumnCount(Option<(usize, usize)>),
    #[error("{0}")]
    ShapeError(ShapeError),
}

pub fn to_array2<T>(vv: Vec<Vec<T>>) -> Result<Array2<T>, ToArrayError> {
    let width = vv
        .iter()
        .map(Vec::len)
        .all_equal_value()
        .map_err(ToArrayError::ColumnCount)?;
    let height = vv.len();
    Array2::from_shape_vec((height, width), vv.into_iter().flatten().collect_vec())
        .map_err(ToArrayError::ShapeError)
}

pub fn chumsky_err<T, E: std::error::Error>(
    res: Result<T, E>,
    span: Range<usize>,
) -> Result<T, Simple<char>> {
    res.map_err(|err| Simple::custom(span, err.to_string()))
}

pub fn parse_stdin<T>(parser: impl chumsky::Parser<char, T, Error = Simple<char>>) -> T {
    parser.parse(read_stdin_to_string()).unwrap()
}

pub fn offset(pos: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        usize::try_from(pos.0 as isize + delta.0).ok()?,
        usize::try_from(pos.1 as isize + delta.1).ok()?,
    ))
}

pub fn bounded_offset(
    pos: (usize, usize),
    delta: (isize, isize),
    size: (usize, usize),
) -> Option<(usize, usize)> {
    offset(pos, delta).and_then(|pos| {
        if pos.0 < size.0 && pos.1 < size.1 {
            Some(pos)
        } else {
            None
        }
    })
}

pub fn wrapping_offset(
    pos: (usize, usize),
    delta: (isize, isize),
    size: (usize, usize),
) -> (usize, usize) {
    (
        (((pos.0 + size.0) as isize + delta.0) as usize) % size.0,
        (((pos.1 + size.1) as isize + delta.1) as usize) % size.1,
    )
}

pub use winnow_util::*;

mod winnow_util {
    use itertools::chain;
    use nalgebra::{DMatrix, Scalar};
    use winnow::{
        ascii::line_ending,
        combinator::{repeat, repeat_till0},
        error::ContextError,
        prelude::*,
    };

    pub fn matrix<'a, T>(
        mut element: impl Parser<&'a str, T, ContextError>,
    ) -> impl Parser<&'a str, DMatrix<T>, ContextError>
    where
        T: Scalar,
    {
        move |input: &mut &'a str| {
            let first_row: Vec<_> = repeat_till0(element.by_ref(), line_ending)
                .parse_next(input)?
                .0;
            let ncols = first_row.len();

            let rows: Vec<_> = repeat(
                ..,
                (repeat(ncols, element.by_ref()), line_ending).map(|(v, _): (Vec<_>, _)| v),
            )
            .parse_next(input)?;

            let nrows = rows.len() + 1;

            Ok(DMatrix::from_iterator(
                nrows,
                ncols,
                chain(std::iter::once(first_row), rows).flatten(),
            )
            .transpose())
        }
    }
}
