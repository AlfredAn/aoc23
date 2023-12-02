use std::io::BufRead;

use regex::Regex;

fn to_digit(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    }
}

fn main() {
    let re = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let lines = std::io::stdin().lock().lines();

    let mut sum = 0;
    for line in lines.map(Result::unwrap) {
        let first = re.find_iter(&line).next().unwrap().as_str();

        let mut last = None;
        for (i, _) in line.char_indices().rev() {
            if let Some(m) = re.find(&line[i..]) {
                last = Some(m.as_str());
                break;
            }
        }
        let last = last.unwrap();

        let number = to_digit(first) * 10 + to_digit(last);
        sum += number;

        println!("{number} - {line}");
    }
    println!("{sum}");
}
