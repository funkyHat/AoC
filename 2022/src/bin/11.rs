use core::panic;
use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

fn main() {
    let mut monkeys = parse("input/11");
    println!("{}", calculate(&mut monkeys, 20, 3));

    monkeys = parse("input/11");
    println!("{}", calculate(&mut monkeys, 10000, 1));
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<u128>,

    left: Operand,
    operator: char,
    right: Operand,

    test: u128,
    if_true: usize,
    if_false: usize,

    inspections: usize,
}

#[derive(Debug, PartialEq)]
enum Operand {
    Number(u128),
    Old,
}

fn get_operand(s: &str) -> Operand {
    match s {
        "old" => Operand::Old,
        _ => Operand::Number(s.parse::<u128>().unwrap()),
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut items = None;
        let mut left = None;
        let mut operator = None;
        let mut right = None;
        let mut test = None;
        let mut if_true = None;
        let mut if_false = None;

        for line in s.lines() {
            if line.starts_with("Monkey ") {
                continue;
            }
            let (label, value) = line.trim().splitn(2, ':').tuples().next().unwrap();
            match label {
                "Starting items" => {
                    items = Some(
                        value
                            .trim()
                            .split(", ")
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                }
                "Operation" => {
                    let mut calc = value.split_whitespace().skip(2).take(3);
                    left = Some(get_operand(calc.next().unwrap()));
                    operator = Some(calc.next().unwrap().chars().next().unwrap());
                    if !"*+".contains(operator.unwrap()) {
                        panic!();
                    };
                    right = Some(get_operand(calc.next().unwrap()));
                }
                "Test" => {
                    test = Some(
                        value
                            .split_ascii_whitespace()
                            .nth(2)
                            .unwrap()
                            .parse()
                            .unwrap(),
                    );
                }
                "If true" => {
                    if_true = Some(value.rsplit(' ').next().unwrap().parse().unwrap());
                }
                "If false" => {
                    if_false = Some(value.rsplit(' ').next().unwrap().parse().unwrap());
                }
                _ => panic!(),
            }
        }

        Monkey {
            items: items.unwrap(),
            left: left.unwrap(),
            operator: operator.unwrap(),
            right: right.unwrap(),
            test: test.unwrap(),
            if_true: if_true.unwrap(),
            if_false: if_false.unwrap(),
            inspections: 0,
        }
    }
}

fn parse(filename: &str) -> Vec<Monkey> {
    let input = std::fs::read_to_string(filename).unwrap();

    let monkeys: Vec<Monkey> = input.split("\n\n").map_into::<Monkey>().collect();

    monkeys
}

fn calculate(monkeys: &mut [Monkey], rounds: usize, releif_division: u128) -> usize {
    let supermod: u128 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            let mut throws: HashMap<usize, Vec<u128>> = HashMap::new();
            {
                let mut monkey = monkeys.get_mut(m).unwrap();
                for item in monkey.items.drain(..) {
                    monkey.inspections += 1;
                    let left = match monkey.left {
                        Operand::Old => item,
                        Operand::Number(n) => n,
                    };
                    let right = match monkey.right {
                        Operand::Old => item,
                        Operand::Number(n) => n,
                    };
                    let new_worry = match monkey.operator {
                        '*' => left * right,
                        '+' => left + right,
                        _ => panic!(),
                    } / releif_division
                        % supermod;

                    match throws.entry(match new_worry % monkey.test {
                        0 => monkey.if_true,
                        _ => monkey.if_false,
                    }) {
                        Entry::Occupied(mut o) => o.get_mut().push(new_worry),
                        Entry::Vacant(v) => {
                            v.insert(vec![new_worry]);
                        }
                    }
                }
            }

            for (m, items) in throws.iter_mut() {
                monkeys.get_mut(*m).unwrap().items.append(items);
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let monkeys = parse("input/11-sample");

        let monkey_1 = &monkeys[0];

        assert_eq!(
            monkey_1,
            &Monkey {
                items: vec![79, 98],
                left: Operand::Old,
                operator: '*',
                right: Operand::Number(19),
                test: 23,
                if_true: 2,
                if_false: 3,
                inspections: 0,
            }
        );
    }

    #[test]
    fn test_calculate() {
        let mut monkeys = parse("input/11-sample");

        assert_eq!(calculate(&mut monkeys, 20, 3), 10605);
    }
}
