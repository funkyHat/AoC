use rust2022::read_lines;

fn main() {
    let mut subsets: u32 = 0;
    let mut overlaps: u32 = 0;

    for line in read_lines("input/4").flatten() {
        let mut ranges: Vec<Vec<i32>> = Vec::new();
        let elves = line.split(',');
        for elf in elves {
            let mut r: Vec<i32> = Vec::new();
            for s in elf.split('-') {
                r.push(s.parse::<i32>().unwrap());
            }
            ranges.push(r);
        }

        ranges.sort_by_key(|x| (x[0], -x[1]));

        if ranges[0][1] >= ranges[1][1] {
            subsets += 1;
        }
        if ranges[0][1] >= ranges[1][0] {
            overlaps += 1;
        }
    }

    println!("Contained: {}", subsets);
    println!("Overlaps:  {}", overlaps);
}
