use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut acc = 0;
    let mut totals: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("input/1") {
        for line in lines.flatten() {
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
    } else {
        eprintln!("file prombel");
        std::process::exit(exitcode::DATAERR);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
