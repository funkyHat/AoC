use itertools::Itertools;
use ringbuf::LocalRb;

fn main() {
    let message = std::fs::read_to_string("input/6").unwrap();

    println!("{}", calculate(message.chars(), 4));
    println!("{}", calculate(message.chars(), 14));
}

fn calculate(input: impl Iterator<Item = char>, size: usize) -> usize {
    let mut buffer = LocalRb::<char, _>::new(size);
    let (mut prod, mut cons) = buffer.split_ref();

    let mut en = input.enumerate();

    for (_, c) in (&mut en).take(size) {
        let _ = prod.push(c);
    }

    for (i, c) in &mut en {
        let current: String = cons.iter().sorted().collect();
        let found =
            std::iter::zip(current.chars(), current.chars().skip(1)).find(|(c1, c2)| c1 == c2);

        if found.is_none() {
            return i;
        }
        let _ = cons.pop();
        let _ = prod.push(c);
    }

    panic!("didn't find a match")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        for (message, size, out) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ] {
            assert_eq!(calculate(message.chars(), size), out);
        }
    }
}
