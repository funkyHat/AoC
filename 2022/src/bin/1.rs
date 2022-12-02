use rust2022::read_lines;

fn main() {
    let mut acc = 0;
    let mut max = 0;

    for line in read_lines("input/1").flatten() {
        if line.is_empty() {
            if acc > max {
                max = acc
            }
            acc = 0;
        } else {
            acc += line.parse::<i32>().unwrap();
        }
    }

    println!("{}", max)
}
