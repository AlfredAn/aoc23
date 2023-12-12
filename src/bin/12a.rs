use std::iter;

use inpt::Inpt;
use itertools::Itertools;

#[derive(Inpt, Clone, Copy, PartialEq, Eq)]
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

#[inpt::main]
fn main(rows: Vec<Row>) {
    let mut sum = 0;

    for Row {
        mut springs,
        groups,
    } in rows
    {
        let unknowns = springs
            .iter()
            .enumerate()
            .filter_map(|(i, &s)| if s == Spring::Unknown { Some(i) } else { None })
            .collect_vec();

        let mut valid_count = 0;

        for n in 0..1 << unknowns.len() {
            for (j, &i) in unknowns.iter().enumerate() {
                if n & (1 << j) == 0 {
                    springs[i] = Spring::Damaged;
                } else {
                    springs[i] = Spring::Operational;
                }
            }

            let mut consecutive = 0;
            let actual_groups = springs
                .iter()
                .copied()
                .chain(iter::once(Spring::Operational))
                .filter_map(|s| match s {
                    Spring::Operational => {
                        let res = consecutive;
                        consecutive = 0;
                        if res > 0 {
                            Some(res)
                        } else {
                            None
                        }
                    }
                    Spring::Damaged => {
                        consecutive += 1;
                        None
                    }
                    Spring::Unknown => unreachable!(),
                });

            if itertools::equal(groups.iter().copied(), actual_groups) {
                valid_count += 1;
            }
        }

        sum += valid_count;
    }

    println!("sum: {sum}");
}
