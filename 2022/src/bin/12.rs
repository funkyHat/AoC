use pathfinding::directed::astar::astar;
use std::collections::HashSet;

fn main() {
    let (grid, start, end) = parse("input/12");
    println!("{}", calculate(&grid, HashSet::from([start]), &end));

    let start_points = get_start_points(&grid);

    println!("{}", calculate(&grid, start_points, &end));
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point(usize, usize);

fn parse(filename: &str) -> (Vec<Vec<u8>>, Point, Point) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut start: Point = Point(0, 0);
    let mut end: Point = Point(0, 0);

    (
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(j, c)| match c {
                        // E
                        69 => {
                            end = Point(j, i);
                            25
                        }
                        // S
                        83 => {
                            start = Point(j, i);
                            0
                        }
                        _ => c - 97,
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}

fn get_start_points(grid: &[Vec<u8>]) -> HashSet<Point> {
    let mut start_points = HashSet::new();
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if c == &0_u8 {
                start_points.insert(Point(j, i));
            }
        }
    }
    start_points
}

fn calculate(grid: &Vec<Vec<u8>>, start_points: HashSet<Point>, end: &Point) -> usize {
    // pathfinding A* function supports multiple END points
    // because you provide the heuristic & success function
    // so I've flipped it around to "start" at E, so it can
    // feed in all of the calculated staring positions.
    astar(
        end,
        |p| {
            let mut moves = vec![];
            for (h, v) in [(1, 0), (0, 1), (-1_i32, 0), (0, -1_i32)] {
                let Some(x) = ({
                    let abs = h.abs().try_into().unwrap();
                    if h.is_positive() {
                        p.0.checked_add(abs)
                    } else {
                        p.0.checked_sub(abs)
                    }
                }) else {continue};

                let Some(y) = ({
                    let abs = v.abs().try_into().unwrap();
                    if v.is_positive() {
                        p.1.checked_add(abs)
                    } else {
                        p.1.checked_sub(abs)
                    }
                })
                else {continue};

                let m = Point(x, y);

                if y >= grid.len() || x >= grid[y].len() || grid[m.1][m.0] + 1 < grid[p.1][p.0] {
                    continue;
                };
                moves.push((m, 1))
            }
            moves
        },
        |p| {
            start_points
                .iter()
                .map(|s| p.0.abs_diff(s.0) + p.1.abs_diff(s.1))
                .min()
                .unwrap()
        },
        |p| start_points.contains(p),
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let (grid, _, _) = parse("input/12-sample");

        assert_eq!(grid[0], vec![0, 0, 1, 16, 15, 14, 13, 12]);
        assert_eq!(grid[2], vec![0, 2, 2, 18, 25, 25, 23, 10]);
    }

    #[test]
    fn test_calculate() {
        for (input, output) in [("input/12-sample", 31), ("input/12", 528)] {
            let (grid, start, end) = parse(input);

            assert_eq!(calculate(&grid, HashSet::from([start]), &end), output);
        }
    }

    #[test]
    fn test_calculate_any_start() {
        for (input, output) in [("input/12-sample", 29), ("input/12", 522)] {
            let (grid, _start, end) = parse(input);
            let start_points = get_start_points(&grid);

            assert_eq!(calculate(&grid, start_points, &end), output);
        }
    }
}
