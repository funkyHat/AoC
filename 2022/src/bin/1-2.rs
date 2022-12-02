use rust2022::read_lines;
use std::cmp::Reverse;

fn main() {
    let mut acc = 0;
    let mut totals: Vec<i32> = Vec::new();

    for line in read_lines("input/1").flatten() {
        if line.is_empty() {
            totals.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<i32>().unwrap();
        }
    }

    totals.sort_by_key(|w| Reverse(*w));
    let top_3: i32 = totals[..3].iter().sum();

    println!("{}", top_3);
}
