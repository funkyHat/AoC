use rust2022::read_lines;

fn main() {
    let mut total: u32 = 0;
    let mut shapes: [u8; 2] = [0; 2];

    for line in read_lines("input/2").flatten() {
        for (i, s) in line.split_ascii_whitespace().enumerate().take(2) {
            // silly ASCII hacking. X-A = 23
            shapes[i] = (s.as_bytes()[0] - 1) % 23 % 3;
        }

        if shapes[0] == shapes[1] {
            // draw
            total += 3
        } else if (shapes[0] + 1) % 3 == shapes[1] {
            // win
            total += 6
        }

        total += (shapes[1] + 1) as u32;
    }

    println!("{}", total);
}
