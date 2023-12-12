use std::iter;

use inpt::Inpt;
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Inpt, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    #[inpt(regex = r"\.")]
    Operational,
    #[inpt(regex = r"#")]
    Damaged,
    #[inpt(regex = r"\?")]
    Unknown,
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

#[derive(Inpt, Debug)]
#[inpt(regex = r"([\.#\?]+) ([\d,]+)")]
struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

fn print_springs(springs: &[Spring]) {
    for spring in springs {
        print!("{:?}", spring);
    }
}

fn combinations(
    springs: &[Spring],
    groups: &[usize],
    consec: usize,
    cache: &mut FxHashMap<(usize, usize, usize), u64>,
) -> u64 {
    let key = (springs.len(), groups.len(), consec);

    if let Some(&res) = cache.get(&key) {
        return res;
    }

    let res = match (springs, groups) {
        ([], []) => 1,
        ([], [g]) if *g == consec => 1,
        ([], _) => 0,
        ([s, ss @ ..], groups) => {
            let operational = |cache| {
                if consec == 0 {
                    combinations(ss, groups, 0, cache)
                } else if groups.first() == Some(&consec) {
                    combinations(ss, &groups[1..], 0, cache)
                } else {
                    0
                }
            };
            let damaged = |cache| {
                if groups.len() > 0 && consec < groups[0] {
                    combinations(ss, groups, consec + 1, cache)
                } else {
                    0
                }
            };

            match s {
                Spring::Operational => operational(cache),
                Spring::Damaged => damaged(cache),
                Spring::Unknown => operational(cache) + damaged(cache),
            }
        }
    };

    cache.insert(key, res);

    res
}

#[inpt::main]
fn main(rows: Vec<Row>) {
    let mut sum = 0;

    for row in rows {
        let springs = iter::repeat(
            row.springs
                .iter()
                .copied()
                .chain(iter::once(Spring::Unknown)),
        )
        .take(4)
        .flatten()
        .chain(row.springs.iter().copied())
        .collect_vec();

        let groups = iter::repeat(row.groups.iter().copied())
            .take(5)
            .flatten()
            .collect_vec();

        print_springs(&springs);
        println!("{:?}", &groups);

        let mut cache = Default::default();
        let n = combinations(&springs, &groups, 0, &mut cache);

        println!("{n}");

        sum += n;

        println!();
    }

    println!("sum: {sum}");
}
