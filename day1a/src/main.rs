use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines();

    let mut sum = 0;
    for line in lines.map(Result::unwrap) {
        let first = line.chars().filter(char::is_ascii_digit).next().unwrap();
        let last = line
            .chars()
            .rev()
            .filter(char::is_ascii_digit)
            .next()
            .unwrap();

        let number = (first as u32 - '0' as u32) * 10 + last as u32 - '0' as u32;
        //println!("{number}");
        sum += number;
    }
    println!("{sum}");
}
