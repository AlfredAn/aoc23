#![feature(map_try_insert)]

use std::{
    convert::identity,
    iter,
    sync::atomic::{AtomicUsize, Ordering},
};

use aoc23::read_stdin_to_string;
use itertools::{chain, Itertools};
use outils::prelude::*;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use winnow::{
    ascii::{line_ending, space0},
    combinator::{preceded, repeat},
    prelude::*,
    seq,
    token::take_while,
};

type Input<'a> = Vec<(&'a str, Vec<&'a str>)>;

fn solve_a(input: &Input) -> u64 {
    let mut vertices = FxHashMap::default();
    let mut labels = Vec::new();
    input
        .iter()
        .flat_map(|(a, bs)| chain!(iter::once(*a), bs.iter().copied()))
        .for_each(|label| {
            if vertices
                .try_insert(label, VertexIndex(vertices.len()))
                .is_ok()
            {
                labels.push(label);
            }
        });

    let mut edges = Vec::new();
    let mut graph = DynamicGraph::<EmptyWeight>::new(vertices.len(), 4);
    for (a, bs) in input {
        let a = vertices[a];
        for b in bs {
            let b = vertices[b];
            let edge = graph.insert_edge(a, b).unwrap();
            edges.push(edge);
        }
    }

    let print_edge = |e: Edge| println!("{}-{}", labels[e.src().0], labels[e.dst().0]);
    let finished = AtomicUsize::new(0);

    let result = (0..edges.len())
        .into_par_iter()
        .map_with((graph, edges), |(graph, edges), i| {
            graph.delete_edge(edges[i]);
            assert!(graph.is_connected(edges[i].src(), edges[i].dst()));

            for j in i + 1..edges.len() {
                graph.delete_edge(edges[j]);
                assert!(graph.is_connected(edges[j].src(), edges[j].dst()));

                for k in j + 1..edges.len() {
                    graph.delete_edge(edges[k]);
                    if !graph.is_connected(edges[k].src(), edges[k].dst()) {
                        print_edge(edges[i]);
                        print_edge(edges[j]);
                        print_edge(edges[k]);

                        let components = graph.components().collect_vec();
                        assert_eq!(components.len(), 2);

                        println!("success: {}", finished.fetch_add(1, Ordering::Relaxed));

                        return Some(
                            components
                                .into_iter()
                                .map(|c| graph.component_vertices(c).count() as u64)
                                .inspect(|n| println!("{n}"))
                                .product::<u64>(),
                        );
                    }

                    edges[k] = graph.insert_edge(edges[k].src(), edges[k].dst()).unwrap();
                }
                edges[j] = graph.insert_edge(edges[j].src(), edges[j].dst()).unwrap();
            }
            edges[i] = graph.insert_edge(edges[i].src(), edges[i].dst()).unwrap();

            println!("finish: {}", finished.fetch_add(1, Ordering::Relaxed));

            None
        })
        .find_map_any(identity);

    result.unwrap()
}

fn main() {
    let input = read_stdin_to_string();
    let input = parser.parse(input.as_str()).unwrap();
    let a = solve_a(&input);
    println!("a: {a}");
}

fn parser<'a>(input: &mut &'a str) -> PResult<Input<'a>> {
    let tag = || take_while(3, |c: char| c.is_ascii_lowercase());
    let line = seq!(
        tag(),
        _: ':',
        repeat(1.., preceded(space0, tag())),
        _: (space0, line_ending)
    );
    repeat(1.., line).parse_next(input)
}
