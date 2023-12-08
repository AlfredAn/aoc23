use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;
use regex::bytes::Regex;

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(String::into_bytes)
        .collect_vec();

    let re = Regex::new("\\d+").unwrap();

    let mut gears = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let ymin = y.saturating_sub(1);
        let ymax = (y + 1).min(lines.len() - 1);

        for m in re.find_iter(line) {
            let xmin = m.range().start.saturating_sub(1);
            let xmax = m.range().end.min(line.len() - 1);

            let mut is_part = false;
            'outer: for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let c = lines[y][x];
                    if !c.is_ascii_digit() && c != b'.' {
                        is_part = true;
                        break 'outer;
                    }
                }
            }

            if !is_part {
                continue;
            }

            let n = std::str::from_utf8(m.as_bytes())
                .unwrap()
                .parse::<u32>()
                .unwrap();

            println!("part: {n}");

            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let c = lines[y][x];
                    if c == b'*' {
                        let gear = gears.entry((x, y)).or_insert((1, 0));
                        gear.0 *= n; // gear ratio
                        gear.1 += 1; // number of adjacent parts
                    }
                }
            }
        }
    }

    for ((x, y), g) in &gears {
        println!("gear: ({x}, {y}): (ratio={}, count={})", g.0, g.1);
    }

    let sum = gears
        .into_values()
        .filter(|&(_, count)| count == 2)
        .map(|(ratio, _)| ratio)
        .sum::<u32>();

    println!("{sum}");
}
