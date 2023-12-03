use std::io::BufRead;

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

    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let ymin = y.saturating_sub(1);
        let ymax = (y + 1).min(lines.len() - 1);

        for m in re.find_iter(&line) {
            let xmin = m.range().start.saturating_sub(1);
            let xmax = m.range().end.min(line.len() - 1);

            'outer: for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let c = lines[y][x];
                    if !c.is_ascii_digit() && c != b'.' {
                        let n = std::str::from_utf8(m.as_bytes())
                            .unwrap()
                            .parse::<u32>()
                            .unwrap();

                        sum += n;
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{sum}");
}
