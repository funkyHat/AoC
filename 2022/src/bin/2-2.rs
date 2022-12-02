use rust2022::read_lines;

fn main() {
    let mut total: u32 = 0;
    let mut shapes: [u8; 2] = [0; 2];

    for line in read_lines("input/2").flatten() {
        for (i, s) in line.split_ascii_whitespace().enumerate().take(2) {
            // silly ASCII hacking. X-A = 23
            shapes[i] = (s.as_bytes()[0] - 1) % 23 % 3;
        }

        // the correct move is always <strategy>+2
        // steps around the ring of choices
        let play = (shapes[0] + shapes[1] + 2) % 3;
        total += (play as u32 + 1) + (shapes[1] * 3) as u32;

        println!("{} {} {} {} {}", line, shapes[0], shapes[1], play, total);
    }

    println!("Total: {}", total);
}
