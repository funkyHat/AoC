use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::read_to_string;

const DECODER_PACKETS: &str = "\
    [[2]]
    [[6]]
";

fn main() {
    println!("{}", calculate("input/13"));
    println!("{}", decoder_key("input/13"));
}

#[derive(Serialize, Deserialize, Debug, Eq)]
#[serde(untagged)]
enum IntList {
    Int(i32),
    List(Vec<IntList>),
}

impl PartialOrd for IntList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntList {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            IntList::Int(l) => match other {
                IntList::Int(r) => l.cmp(r),
                IntList::List(r) => vec![IntList::Int(*l)].cmp(r),
            },
            IntList::List(l) => match other {
                IntList::Int(r) => l.cmp(&vec![IntList::Int(*r)]),
                IntList::List(r) => l.cmp(r),
            },
        }
    }
}

impl PartialEq for IntList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn parse_line(l: &str) -> IntList {
    serde_json::from_str(l).unwrap()
}

fn calculate(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .enumerate()
        .map(|(i, packets)| {
            let (l, r) = packets.lines().map(parse_line).next_tuple().unwrap();
            if l < r {
                i + 1
            } else {
                0
            }
        })
        .sum()
}

fn sort_packets(s: &str) -> Vec<IntList> {
    s.split_whitespace()
        .chain(DECODER_PACKETS.split_whitespace())
        .map(parse_line)
        .sorted()
        .collect()
}

fn decoder_packets() -> Vec<IntList> {
    sort_packets(DECODER_PACKETS)
}

fn decoder_key(s: &str) -> usize {
    sort_packets(&read_to_string(s).unwrap())
        .iter()
        .enumerate()
        .filter_map(|(i, p)| -> Option<usize> {
            match decoder_packets().contains(p) {
                true => Some(i + 1),
                false => None,
            }
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare() {
        for (left, right, order) in [
            ("[9]", "[[8,7,6]]", Ordering::Greater),
            ("[]", "[3]", Ordering::Less),
        ] {
            let l = parse_line(left);
            let r = parse_line(right);
            assert_eq!(l.cmp(&r), order);
        }
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("input/13-sample"), 13);
    }

    #[test]
    fn test_calculate_all() {
        let sorted = sort_packets(read_to_string("input/13-sample").unwrap().as_str());
        assert_eq!(
            sorted
                .iter()
                .map(|l| serde_json::to_string(&l).unwrap())
                .join("\n"),
            "\
[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]"
        );
    }

    #[test]
    fn test_decoder_packet_indces() {
        assert_eq!(decoder_key("input/13-sample"), 140);
    }
}
