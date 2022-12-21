use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/20").unwrap();

    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("{}", calculate(numbers, &1, 1));

    let numbers: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("{}", calculate(numbers, &7395, 10));
}

fn calculate(input: Vec<i64>, key: &i64, rounds: usize) -> i64 {
    let working: Vec<_> = input.iter().enumerate().collect();

    // a number being at the end or at the start
    // is equivalent because it is still between
    // the same 2 numbers, so our modulus should
    // be one less than we would expect it to be
    let len: i64 = working.len() as i64 - 1;

    let mut working: Vec<_> = working.into_iter().map(|(i, v)| (i, v * key)).collect();

    // println!("{:?}", &working.iter().map(|i| i.1).collect::<Vec<i64>>());

    for _ in 0..rounds {
        for i in 0..working.len() {
            let (pos, (_, mv)) = working
                .iter()
                .enumerate()
                .find(|(_, (j, _))| i == *j)
                .unwrap();

            let pos_i64: i64 = i64::try_from(pos).unwrap();
            let pre_mod = pos_i64 + *mv;
            let new_pos_i = ((pre_mod % len) + len) % len;
            let new_pos: usize = new_pos_i.try_into().unwrap();

            match pos.cmp(&new_pos) {
                Less => {
                    working[pos..=new_pos].rotate_left(1);
                }
                Greater => {
                    working[new_pos..=pos].rotate_right(1);
                }
                Equal => (),
            }

            // println!("{:?}", &working.iter().map(|i| *i.1).collect::<Vec<i64>>());
        }
    }

    let pos_of_0 = working
        .iter()
        .enumerate()
        .find(|(_, (_, n))| *n == 0)
        .unwrap()
        .0;
    dbg!(pos_of_0);

    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let m = working[(pos_of_0 + i) % working.len()].1;
            dbg!(m);
            m
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        for (input, key, rounds, result) in [
            ("input/20-sample", 1, 1, 3),
            ("input/20", 1, 1, 7395),
            ("input/20-sample", 811589153, 10, 1623178306),
            ("input/20", 811589153, 10, 1640221678213),
        ] {
            let numbers: Vec<i64> = read_to_string(input)
                .unwrap()
                .lines()
                .map(|l| l.parse().unwrap())
                .collect();
            assert_eq!(calculate(numbers, &key, rounds), result);
        }
    }
}
