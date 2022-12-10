use std::collections::HashSet;

use rust2022::read_lines;

fn main() {
    let mut total = 0;
    for line in read_lines("input/3").flatten() {
        let mut comp_a: HashSet<char> = HashSet::new();
        let mut comp_b: HashSet<char> = HashSet::new();
        'line: for (a, b) in std::iter::zip(line.chars(), line.chars().rev()) {
            comp_a.insert(a);
            comp_b.insert(b);

            for (c, other) in [(a, &comp_b), (b, &comp_a)] {
                if other.contains(&c) {
                    let score = priority(c);
                    total += score;
                    break 'line;
                }
            }
        }
    }

    println!("{}", total);
}

fn priority(c: char) -> i32 {
    if c.is_uppercase() {
        return (c as i32) - 38;
    }
    (c as i32) - 96
}
