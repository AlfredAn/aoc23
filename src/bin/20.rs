use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt,
    ops::Not,
};

use aoc23::read_stdin_to_string;
use graphviz_rust::{
    dot_generator::*,
    dot_structures::*,
    printer::{DotPrinter, PrinterContext},
};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use strum::EnumIs;
use winnow::{
    ascii::{line_ending, space0},
    combinator::{alt, preceded, repeat, separated},
    prelude::*,
    seq,
    token::take_while,
};

type Tag<'a> = &'a str;

#[derive(Debug, Clone, Copy, EnumIs, PartialEq, Eq, Hash)]
enum PulseValue {
    Low,
    High,
}

impl fmt::Display for PulseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PulseValue::Low => "low",
                PulseValue::High => "high",
            }
        )
    }
}

impl Not for PulseValue {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(PulseValue),
    Conjunction(BTreeMap<Tag<'a>, PulseValue>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pulse<'a> {
    source: Tag<'a>,
    target: Tag<'a>,
    value: PulseValue,
    depth: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ModuleWithOutputs<'a> {
    module: Module<'a>,
    outputs: Vec<Tag<'a>>,
}

const DEBUG: bool = false;

type ModGraph<'a> = BTreeMap<&'a str, ModuleWithOutputs<'a>>;

fn build_graph<'a>(input: &'a [(Tag, Module, Vec<Tag>)]) -> ModGraph<'a> {
    let mut graph = ModGraph::default();
    for (tag, module, outputs) in input {
        graph.insert(
            *tag,
            ModuleWithOutputs {
                module: module.clone(),
                outputs: outputs.clone(),
            },
        );
    }

    for (tag, _, outputs) in input {
        for output in outputs {
            if let Some(ModuleWithOutputs {
                module: Module::Conjunction(inputs),
                ..
            }) = graph.get_mut(output)
            {
                inputs.insert(tag, PulseValue::Low);
            }
        }
    }
    graph
}

fn resolve_pulse<'a>(
    source: Tag<'a>,
    target: Tag<'a>,
    value: PulseValue,
    graph: &mut ModGraph<'a>,
    queue: &mut VecDeque<Pulse<'a>>,
    mut on_pulse: impl FnMut(Pulse<'a>),
) {
    macro_rules! pulse {
        ($source:expr, $value:expr, $depth:expr) => {{
            let (source, value, depth) = ($source, $value, $depth);
            for target in &graph[&source].outputs {
                queue.push_back(Pulse {
                    source,
                    target,
                    value,
                    depth,
                });
            }
        }};
    }

    queue.push_back(Pulse {
        source,
        target,
        value,
        depth: 0,
    });

    while let Some(pulse) = queue.pop_front() {
        if DEBUG {
            println!("{} -{}-> {}", pulse.source, pulse.value, pulse.target);
        }

        on_pulse(pulse);

        if let Some(node) = graph.get_mut(&pulse.target) {
            match node.module {
                Module::Broadcaster => pulse!(pulse.target, pulse.value, pulse.depth + 1),
                Module::FlipFlop(ref mut state) => {
                    if pulse.value.is_low() {
                        *state = !*state;
                        pulse!(pulse.target, *state, pulse.depth + 1);
                    }
                }
                Module::Conjunction(ref mut inputs) => {
                    inputs.insert(pulse.source, pulse.value);
                    if inputs.iter().all(|(_, &input)| input.is_high()) {
                        pulse!(pulse.target, PulseValue::Low, pulse.depth + 1);
                    } else {
                        pulse!(pulse.target, PulseValue::High, pulse.depth + 1);
                    }
                }
            }
        }
    }
}

fn solve_a(input: &[(Tag, Module, Vec<Tag>)]) -> u64 {
    let mut graph = build_graph(input);

    if DEBUG {
        println!("{graph:#?}");
    }

    let mut queue = VecDeque::new();
    let (mut n_low, mut n_high) = (0, 0);

    for _ in 0..1000 {
        resolve_pulse(
            "button",
            "broadcaster",
            PulseValue::Low,
            &mut graph,
            &mut queue,
            |pulse| match pulse.value {
                PulseValue::Low => n_low += 1,
                PulseValue::High => n_high += 1,
            },
        );
    }

    if DEBUG {
        println!("({n_low}, {n_high})");
    }

    n_low * n_high
}

fn solve_b(input: &[(Tag, Module, Vec<Tag>)]) -> u64 {
    let graph = build_graph(input);
    let starts = graph["broadcaster"].outputs.iter().copied().collect_vec();
    let end = "hb";

    let mut periods = Vec::new();

    for start in starts {
        dbg!(start);

        let mut partition = BTreeSet::new();
        partition.insert(start);

        let mut new = Vec::<&str>::new();
        loop {
            new.extend(
                partition
                    .iter()
                    .flat_map(|&tag| graph[tag].outputs.iter())
                    .filter(|&&neighbor| !partition.contains(&neighbor) && neighbor != end),
            );
            if new.is_empty() {
                break;
            }
            partition.extend(new.drain(..));
        }

        dbg!(&partition);

        let mut graph = graph
            .iter()
            .filter(|(tag, _)| partition.contains(*tag))
            .map(|(tag, module)| (*tag, module.clone()))
            .collect::<ModGraph>();

        let mut seen = FxHashMap::default();
        let mut queue = VecDeque::new();
        let mut pattern = Vec::new();
        let period_start = loop {
            match seen.insert(graph.clone(), seen.len()) {
                None => (),
                Some(old) => break old,
            }

            let mut pulses = Vec::new();
            resolve_pulse(
                "broadcaster",
                start,
                PulseValue::Low,
                &mut graph,
                &mut queue,
                |pulse| {
                    if pulse.target == end {
                        pulses.push((pulse.value, pulse.depth));
                    }
                },
            );

            pattern.push(pulses);
        };

        let (high_i, high_v) = pattern
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v.iter().any(|(value, _)| value.is_high()))
            .exactly_one()
            .unwrap();

        assert!(high_v.len() > 1);

        let (_, high_depth) = high_v
            .into_iter()
            .filter(|(value, _)| value.is_high())
            .exactly_one()
            .unwrap();

        let period = seen.len() - period_start;

        dbg!(period_start);
        dbg!(period);
        dbg!(high_i);
        dbg!(high_depth);

        assert_eq!(period_start, 1);
        assert_eq!(high_depth, 3);
        assert_eq!(high_i, period - 1);

        periods.push(period);
    }

    dbg!(&periods);

    periods.into_iter().map(|p| p as u64).product::<u64>()
}

fn print_dot(input: &[(Tag, Module, Vec<Tag>)]) {
    let mut graph = graph!(strict di id!("g"));

    for (tag, module, outputs) in input {
        let &tag = tag;
        let label = match module {
            Module::Broadcaster => r#""\N""#,
            Module::FlipFlop(_) => r#""%\N""#,
            Module::Conjunction(_) => r#""&\N""#,
        };
        graph.add_stmt(stmt!(node!(tag; attr!("label", label))));
        for &output in outputs {
            graph.add_stmt(stmt!(edge!(node_id!(tag) => node_id!(output))));
        }
    }

    let dot = graph.print(&mut PrinterContext::default());
    println!("{}", dot);
}

fn solve(input: &str) {
    let args = std::env::args().collect_vec();
    let args = args.iter().map(|s| s.as_str()).collect_vec();

    let input = parser.parse(input).unwrap();

    match &args[1..] {
        ["a"] => {
            let a = solve_a(&input);
            println!("a: {a}");
        }
        ["dot"] => {
            print_dot(&input);
        }
        ["b"] => {
            let b = solve_b(&input);
            println!("b: {b}");
        }
        _ => panic!("invalid arguments"),
    }
}

fn main() {
    solve(read_stdin_to_string().as_str());
}

fn parser<'a>(input: &mut &'a str) -> PResult<Vec<(Tag<'a>, Module<'a>, Vec<Tag<'a>>)>> {
    let tag =
        |input: &mut &'a str| take_while(1.., |c: char| c.is_ascii_lowercase()).parse_next(input);

    let mut module = alt((
        "broadcaster".map(|tag| (tag, Module::Broadcaster)),
        preceded('%', tag).map(|tag| (tag, Module::FlipFlop(PulseValue::Low))),
        preceded('&', tag).map(|tag| (tag, Module::Conjunction(Default::default()))),
    ));

    let mut outputs = separated(1.., tag, (',', space0));

    repeat(
        1..,
        seq!(
            _: space0,
            module.by_ref(),
            _: (space0, "->", space0),
            outputs.by_ref(),
            _: (space0, line_ending),
        )
        .map(|((tag, mod_), out)| (tag, mod_, out)),
    )
    .parse_next(input)
}

#[cfg(test)]
#[test]
fn test() {
    let ex = parser.parse(include_str!("../../in/20/ex")).unwrap();
    let ex2 = parser.parse(include_str!("../../in/20/ex2")).unwrap();
    let input = parser.parse(include_str!("../../in/20/i")).unwrap();

    assert_eq!(solve_a(&ex), 32000000);
    assert_eq!(solve_a(&ex2), 11687500);
    assert_eq!(solve_a(&input), 684125385);

    assert_eq!(solve_b(&input), 225872806380073);
}
