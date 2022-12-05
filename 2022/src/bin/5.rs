use itertools::Itertools;
use rust2022::read_lines;

fn main() {
    let mut lines = read_lines("input/5").flatten();

    let mut stacks = parse_stacks(&mut lines);

    dbg!(&stacks);

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap());

        let count = numbers.next().unwrap();
        let from = numbers.next().unwrap() - 1;
        let to = numbers.next().unwrap() - 1;

        for _ in 0..count {
            let c = &stacks[from].pop().unwrap();
            stacks[to].push(*c);
        }
    }

    println!("{}", &stacks.iter().map(|x| x[x.len() - 1]).join(""))
}

fn parse_stacks(lines: impl Iterator<Item = String>) -> Vec<Vec<char>>
where {
    let mut rows: Vec<String> = vec![];

    for line in lines.take_while(|x| !x.is_empty()) {
        rows.push(line)
    }

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in rows.pop().unwrap().split_whitespace() {
        stacks.push(vec![])
    }

    for row in rows.iter().rev() {
        for (i, item) in row.chars().skip(1).step_by(4).enumerate() {
            if item != ' ' {
                stacks[i].push(item)
            }
        }
    }

    stacks
}
