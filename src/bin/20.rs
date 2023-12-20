use std::{collections::VecDeque, fmt, ops::Not};

use aoc23::read_stdin_to_string;
use rustc_hash::FxHashMap;
use strum::EnumIs;
use vecmap::VecMap;
use winnow::{
    ascii::{line_ending, space0},
    combinator::{alt, preceded, repeat, separated},
    prelude::*,
    seq,
    token::take_while,
};

type Tag<'a> = &'a str;

#[derive(Debug, Clone, Copy, EnumIs)]
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

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(PulseValue),
    Conjunction(VecMap<Tag<'a>, PulseValue>),
}

#[derive(Debug, Clone, Copy)]
struct Pulse<'a> {
    source: Tag<'a>,
    target: Tag<'a>,
    value: PulseValue,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    module: Module<'a>,
    outputs: Vec<Tag<'a>>,
}

const DEBUG: bool = false;

fn solve_a(input: &[(Tag, Module, Vec<Tag>)]) -> u64 {
    let mut graph = FxHashMap::default();

    for (tag, module, outputs) in input {
        graph.insert(
            tag,
            Node {
                module: module.clone(),
                outputs: outputs.clone(),
            },
        );
    }

    for (tag, _, outputs) in input {
        for output in outputs {
            if let Some(Node {
                module: Module::Conjunction(inputs),
                ..
            }) = graph.get_mut(output)
            {
                inputs.insert(tag, PulseValue::Low);
            }
        }
    }

    if DEBUG {
        println!("{graph:#?}");
    }

    let mut queue = VecDeque::new();
    let (mut n_low, mut n_high) = (0, 0);

    macro_rules! pulse {
        ($source:expr, $value:expr) => {{
            let source = $source;
            let value = $value;
            for target in &graph[&source].outputs {
                queue.push_back(Pulse {
                    source,
                    target,
                    value,
                });
            }
        }};
    }

    for _ in 0..1000 {
        if DEBUG {
            println!("button -low-> broadcaster");
        }
        n_low += 1;

        pulse!("broadcaster", PulseValue::Low);

        while let Some(pulse) = queue.pop_front() {
            if DEBUG {
                println!("{} -{}-> {}", pulse.source, pulse.value, pulse.target);
            }

            match pulse.value {
                PulseValue::Low => n_low += 1,
                PulseValue::High => n_high += 1,
            };

            if let Some(node) = graph.get_mut(&pulse.target) {
                match node.module {
                    Module::Broadcaster => pulse!(pulse.target, pulse.value),
                    Module::FlipFlop(ref mut state) => {
                        if pulse.value.is_low() {
                            *state = !*state;
                            pulse!(pulse.target, *state);
                        }
                    }
                    Module::Conjunction(ref mut inputs) => {
                        inputs[pulse.source] = pulse.value;
                        if inputs.iter().all(|(_, &input)| input.is_high()) {
                            pulse!(pulse.target, PulseValue::Low);
                        } else {
                            pulse!(pulse.target, PulseValue::High);
                        }
                    }
                }
            }
        }
    }

    if DEBUG {
        println!("({n_low}, {n_high})");
    }

    n_low * n_high
}

fn solve(input: &str) {
    let input = parser.parse(input).unwrap();

    let a = solve_a(&input);
    println!("a: {a}");
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
        preceded('&', tag).map(|tag| (tag, Module::Conjunction(VecMap::new()))),
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
