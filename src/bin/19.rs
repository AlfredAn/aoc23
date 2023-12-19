use std::ops::Range;

use aoc23::read_stdin_to_string;
use enum_map::{enum_map, Enum, EnumMap};
use itertools::Itertools;
use rangetools::Rangetools;
use rustc_hash::FxHashMap;
use winnow::{
    ascii::{dec_uint, multispace0},
    combinator::{alt, dispatch, fail, preceded, repeat, seq, success, terminated},
    prelude::*,
    token::{any, take_while},
    trace::trace,
};

#[derive(Debug, Clone, Copy, Enum)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    GreaterThan(u16),
    LessThan(u16),
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug, Clone)]
struct Rule {
    category: Category,
    condition: Condition,
    action: Action,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    default: Action,
}

type Part = EnumMap<Category, u16>;

fn combinations(
    workflows: &FxHashMap<String, Workflow>,
    tag: &str,
    index: usize,
    ranges: EnumMap<Category, Range<u16>>,
) -> (u64, u64) {
    assert!(!ranges.values().any(Range::is_empty));

    let workflow = &workflows[tag];

    let count = |rs: &EnumMap<_, Range<_>>| rs.values().map(|r| r.len() as u64).product();

    let combs_with_action = |a: &Action, rs: EnumMap<_, Range<_>>| match a {
        Action::Accept => {
            let count = count(&rs);
            (count, 0)
        }
        Action::Reject => {
            let count = count(&rs);
            (0, count)
        }
        Action::Goto(tag) => combinations(workflows, tag, 0, rs),
    };

    match workflow.rules.get(index) {
        None => combs_with_action(&workflow.default, ranges),
        Some(rule) => {
            let range = ranges[rule.category].clone();
            let [pass, fail] = match rule.condition {
                Condition::GreaterThan(n) => [
                    range.clone().intersection(n + 1..),
                    range.intersection(..n + 1),
                ],
                Condition::LessThan(n) => {
                    [range.clone().intersection(..n), range.intersection(n..)]
                }
            }
            .map(|r| {
                if r.is_empty() {
                    None
                } else {
                    Some({
                        let mut rs = ranges.clone();
                        rs[rule.category] = r.into();
                        rs
                    })
                }
            });

            let pass_val = pass.map_or((0, 0), |pass| combs_with_action(&rule.action, pass));
            let fail_val =
                fail.map_or((0, 0), |fail| combinations(workflows, tag, index + 1, fail));

            (pass_val.0 + fail_val.0, pass_val.1 + fail_val.1)
        }
    }
}

fn solve_a(workflows: &FxHashMap<String, Workflow>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .map(|part| {
            match combinations(
                workflows,
                "in",
                0,
                EnumMap::from_fn(|c| part[c]..part[c] + 1),
            ) {
                (0, _) => 0,
                (1, _) => part.values().copied().map_into::<u64>().sum(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn solve_b(workflows: &FxHashMap<String, Workflow>) -> (u64, u64) {
    combinations(workflows, "in", 0, EnumMap::from_fn(|_| 1..4001))
}

fn solve(input: &str) -> (u64, u64) {
    let (workflows, parts) = input_parser.parse(input).unwrap();
    let workflows = workflows.into_iter().collect::<FxHashMap<_, _>>();

    let a = solve_a(&workflows, &parts);
    let (b, b2) = solve_b(&workflows);
    assert_eq!(b + b2, 4000u64.pow(4));

    (a, b)
}

fn main() {
    let input = read_stdin_to_string();
    let (a, b) = solve(input.as_str());
    println!("a: {a}\nb: {b}");
}

fn category(input: &mut &str) -> PResult<Category> {
    trace(
        "category",
        dispatch!(any;
            'x' => success(Category::X),
            'm' => success(Category::M),
            'a' => success(Category::A),
            's' => success(Category::S),
            _ => fail
        ),
    )
    .parse_next(input)
}

fn condition(input: &mut &str) -> PResult<Condition> {
    trace(
        "condition",
        dispatch!((any, dec_uint);
            ('>', n) => success(Condition::GreaterThan(n)),
            ('<', n) => success(Condition::LessThan(n)),
            _ => fail
        ),
    )
    .parse_next(input)
}

fn tag(input: &mut &str) -> PResult<String> {
    trace(
        "tag",
        take_while(1.., |c: char| c.is_ascii_lowercase()).map(Into::into),
    )
    .parse_next(input)
}

fn action(input: &mut &str) -> PResult<Action> {
    trace(
        "action",
        alt((
            'A'.map(|_| Action::Accept),
            'R'.map(|_| Action::Reject),
            tag.map(Action::Goto),
        )),
    )
    .parse_next(input)
}

fn rule(input: &mut &str) -> PResult<Rule> {
    trace(
        "rule",
        seq! {Rule {
            category: category,
            condition: condition,
            _: ':',
            action: action,
        }},
    )
    .parse_next(input)
}

fn workflow(input: &mut &str) -> PResult<(String, Workflow)> {
    trace(
        "workflow",
        seq!(
            tag,
            _: '{',
            repeat(.., terminated(rule, ',')),
            action,
            _: '}',
        )
        .map(|(tag, rules, default)| (tag, Workflow { rules, default })),
    )
    .parse_next(input)
}

fn part(input: &mut &str) -> PResult<Part> {
    trace(
        "part",
        seq!(
            _: "{x=",
            dec_uint,
            _: ",m=",
            dec_uint,
            _: ",a=",
            dec_uint,
            _: ",s=",
            dec_uint,
            _: '}',
        )
        .map(|(x, m, a, s)| {
            enum_map! {
                Category::X => x,
                Category::M => m,
                Category::A => a,
                Category::S => s
            }
        }),
    )
    .parse_next(input)
}

#[allow(clippy::type_complexity)]
fn input_parser(input: &mut &str) -> PResult<(Vec<(String, Workflow)>, Vec<Part>)> {
    trace(
        "input_parser",
        seq!(
            repeat(.., preceded(multispace0, workflow)),
            repeat(.., preceded(multispace0, part)),
            _: multispace0,
        ),
    )
    .parse_next(input)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        solve(include_str!("../../in/19/ex")),
        (19114, 167409079868000)
    );
    assert_eq!(
        solve(include_str!("../../in/19/i")),
        (446517, 130090458884662)
    );
}
