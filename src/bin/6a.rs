use aoc23::read_stdin_to_string;
use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    record_dist: u64,
}

fn parser() -> impl Parser<char, Vec<Race>, Error = Simple<char>> {
    let int = text::int(10).from_str::<u64>().unwrapped().padded();

    let times = just("Time:").ignore_then(int.repeated());
    let distances = just("Distance:").ignore_then(int.repeated());

    times.then(distances).map(|(ts, ds)| {
        itertools::zip_eq(ts, ds)
            .map(|(time, record_dist)| Race { time, record_dist })
            .collect()
    })
}

fn main() {
    let races = parser().parse(read_stdin_to_string()).unwrap();
    dbg!(&races);

    let mut product: u64 = 1;

    for Race { time, record_dist } in races {
        let mut ways = 0;
        for hold_time in 0..=time {
            let speed = hold_time;
            let travel_time = time - hold_time;
            let dist = speed * travel_time;

            if dist > record_dist {
                ways += 1;
            }
        }

        product *= ways;
    }

    println!("{product}");
}
