use std::collections::HashSet;
use itertools::Itertools;

use rust2022::read_lines;


fn main() {
    let mut total = 0;

    for chunk in &read_lines("input/3").flatten().into_iter().chunks(3) {
        let mut lines = chunk.into_iter();
        let mut common: HashSet<char> = HashSet::from_iter(lines.next().unwrap().chars());
        for bag in lines {
            let next_bag = HashSet::from_iter(bag.chars());
            common = common.intersection(&next_bag).map(|a| *a).collect();
        }
        let pri = priority(common.into_iter().next().unwrap());

        total += pri;
    }

    println!("{}", total);
}


fn priority(c: char) -> i32 {
    if c.is_uppercase() {
        return (c as i32) - 38;
    }
    (c as i32) - 96
}
