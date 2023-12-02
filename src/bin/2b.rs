use anyhow::anyhow;
use enum_map::{Enum, EnumMap};
use std::{io::BufRead, str::FromStr};

#[derive(Enum, Debug, Clone, Copy)]
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
    let mut sum = 0;

    for line in std::io::stdin().lock().lines().map(Result::unwrap) {
        let (_, game) = line.split_once(':').unwrap();

        let mut min_counts = EnumMap::default();

        for set in game.split(';') {
            for part in set.split(',').map(str::trim) {
                let (number, color) = part.split_once(' ').unwrap();

                let number = number.parse::<u32>().unwrap();
                let color = color.parse::<Color>().unwrap();

                min_counts[color] = u32::max(min_counts[color], number);
            }
        }

        let power = min_counts
            .into_iter()
            .map(|(_, count)| count)
            .product::<u32>();
        sum += power;
    }

    println!("{sum}");
}
