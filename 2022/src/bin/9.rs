use itertools::Itertools;
use lending_iterator::prelude::*;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};

fn main() {
    let input = std::fs::read_to_string("input/9").unwrap();
    for tail_len in [1, 9] {
        println!(
            "Tail: {}: {}",
            tail_len,
            tail_visited_count(&input, tail_len)
        );
    }
}

fn tail_visited_count(input: &str, length: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); length + 1];
    // One set of visited locations for each tail segment. We
    // only need the last one but doing it for all is simpler
    let mut visited: Vec<HashSet<(i32, i32)>> = vec![HashSet::from([(0, 0)]); length];

    for line in input.lines() {
        let (dir, count) = line.split_whitespace().take(2).tuples().next().unwrap();
        let count: u8 = count.parse().unwrap();

        for _ in 0..count {
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                _ => {
                    panic!("unknown input")
                }
            }

            let mut count = 0;
            let mut pairs = rope.windows_mut::<2>();
            while let Some(&mut [a, ref mut b]) = pairs.next() {
                follow(&a, b, &mut visited[count]);
                count += 1;
            }
        }
    }

    visited[visited.len() - 1].len()
}

fn mv_axis(h: &i32, t: &mut i32) {
    match h.cmp(t) {
        Greater => *t += 1,
        Less => *t -= 1,
        Equal => (),
    }
}

fn follow(h_pos: &(i32, i32), t_pos: &mut (i32, i32), visited: &mut HashSet<(i32, i32)>) {
    let (hx, hy) = h_pos;
    let (ref mut tx, ref mut ty) = *t_pos;
    let mx: i32 = *hx - *tx;
    let my: i32 = *hy - *ty;

    if (-1..=1).contains(&mx) && (-1..=1).contains(&my) {
        // no move
        return;
    }

    mv_axis(hy, &mut *ty);
    mv_axis(hx, &mut *tx);

    visited.insert((*tx, *ty));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_visited_count() {
        for (file, len, ans) in [
            ("input/9-sample", 1, 13),
            ("input/9", 1, 6367),
            ("input/9-sample", 9, 1),
            ("input/9-sample2", 9, 36),
        ] {
            let input = std::fs::read_to_string(file).unwrap();
            assert_eq!(tail_visited_count(&input, len), ans);
        }
    }
}
