use anyhow::anyhow;
use enum_map::{enum_map, Enum};
use std::{io::BufRead, str::FromStr};

#[derive(Enum, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(anyhow!("invalid input")),
        }
    }
}

fn main() {
    let max_counts = enum_map! {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    };

    let mut sum = 0;

    'line: for (i, line) in std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .enumerate()
    {
        let (_, game) = line.split_once(':').unwrap();
        for set in game.split(';') {
            for part in set.split(',').map(str::trim) {
                let (number, color) = part.split_once(' ').unwrap();

                let number = number.parse::<u32>().unwrap();
                let color = color.parse().unwrap();

                if number > max_counts[color] {
                    continue 'line;
                }
            }
        }
        sum += i + 1;
    }

    println!("{sum}");
}
