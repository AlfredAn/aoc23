use aoc23::read_stdin_to_string;
use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    record: u64,
}

fn parser() -> impl Parser<char, Race, Error = Simple<char>> {
    let int = text::int(10).from_str::<u64>().unwrapped().padded();

    let long_int = int.then(int.repeated()).foldl(|a, b| {
        a * 10u64.pow(b.ilog10() + 1) + b
    });

    let time = just("Time:").ignore_then(long_int);
    let record = just("Distance:").ignore_then(long_int);

    time.then(record)
        .map(|(time, record)| Race { time, record })
}

fn main() {
    let race = parser().parse(read_stdin_to_string()).unwrap();
    dbg!(&race);

    let mut ways = 0;
    for hold_time in 0..=race.time {
        let speed = hold_time;
        let travel_time = race.time - hold_time;
        let dist = speed * travel_time;

        if dist > race.record {
            ways += 1;
        }
    }
    println!("{ways}");
}
