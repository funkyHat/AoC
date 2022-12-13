use itertools::Itertools;
use std::fs::read_to_string;

const ROWS: usize = 6;
const COLS: usize = 40;

fn main() {
    let input = read_to_string("input/10").unwrap();
    println!("{}", signal_strengths(&input, 20, 40));

    let cpu = Cpu::new(&input);
    let mut pixels: Vec<char> = vec![','; ROWS * COLS];

    for (i, x) in cpu.enumerate() {
        if (x - 1..x + 2).contains(&(i32::try_from(i % COLS).unwrap())) {
            pixels[i] = '#';
        }
    }

    for row in pixels.iter().chunks(COLS).into_iter() {
        println!("{}", row.collect::<String>());
    }
}

#[derive(Debug)]
struct Cpu<'a> {
    iter: std::str::Lines<'a>,

    cycles: u8,
    next_val: i32,

    x: i32,
}

impl<'a> Iterator for Cpu<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles > 0 {
            let prev_x = self.x;
            if self.cycles == 1 {
                self.x = self.next_val;
            }
            self.cycles -= 1;
            return Some(prev_x);
        }

        let Some(instruction) = self.iter.next() else {
             return None
             };
        let mut parts = instruction.split_whitespace();
        let Some(opcode) = parts.next() else {
            panic!("unknown input")
        };

        match opcode {
            "noop" => Some(self.x),
            "addx" => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                self.next_val = self.x + value;
                self.cycles = 1;

                Some(self.x)
            }

            _ => panic!("unknown opcode"),
        }
    }
}

impl<'a> Cpu<'a> {
    fn new(input: &'a str) -> Self {
        Cpu {
            iter: input.lines(),
            x: 1,
            next_val: 0,
            cycles: 0,
        }
    }
}

fn signal_strengths(input: &str, offset: usize, step: usize) -> i32 {
    let cpu = Cpu::new(input);

    cpu.enumerate()
        .skip(offset - 1)
        .step_by(step)
        .map(|(i, x)| (i as i32 + 1) * x)
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_cycles_small() {
        let input = read_to_string("input/10-sample1").unwrap();
        let cpu = Cpu::new(&input);
        assert_eq!(cpu.collect::<Vec<i32>>(), [1, 1, 1, 4, 4]);
    }

    #[test]
    fn test_signal_strengths() {
        let input = read_to_string("input/10-sample2").unwrap();
        assert_eq!(signal_strengths(&input, 20, 40), 13140);
    }
}
