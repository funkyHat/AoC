use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
    iter::repeat_with,
};

use itertools::Itertools;

fn main() {
    println!("Sand units: {}", calculate("input/14", false));
    println!("With floor: {}", calculate("input/14", true));
}

#[derive(Debug, PartialEq)]
enum Tile {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Map {
    grid: HashMap<(isize, isize), Tile>,
    lowest_point: isize,
}

fn parse(filename: &str) -> Map {
    let input = read_to_string(filename).unwrap();
    let mut grid = HashMap::new();
    let mut lowest_point = 0;

    for line in input.lines() {
        for (point1, point2) in line
            .split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(|v| v.parse::<isize>().unwrap())
                    .tuples::<(_, _)>()
                    .next()
                    .unwrap()
            })
            .tuple_windows()
            .map(|(a, b)| if a > b { (b, a) } else { (a, b) })
        {
            lowest_point = lowest_point.max(point2.1);

            if point1.0 == point2.0 {
                repeat_with(|| point1.0)
                    .zip(point1.1..=point2.1)
                    .for_each(|p| {
                        grid.entry(p).or_insert(Tile::Rock);
                    });
            } else if point1.1 == point2.1 {
                (point1.0..=point2.0)
                    .zip(repeat_with(|| point1.1))
                    .for_each(|p| {
                        grid.entry(p).or_insert(Tile::Rock);
                    });
            } else {
                panic!("invalid pair of points: {:?} {:?}", point1, point2)
            }
        }
    }

    Map { grid, lowest_point }
}

fn calculate(filename: &str, with_floor: bool) -> usize {
    let mut map = parse(filename);

    'item: loop {
        let mut pos = (500, 0);

        if map.grid.contains_key(&pos) {
            break;
        }

        'step: loop {
            if with_floor {
                if pos.1 > map.lowest_point {
                    break 'step;
                }
            } else if pos.1 > map.lowest_point {
                break 'item;
            }
            for p in [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ]
            .iter()
            {
                match map.grid.entry(*p) {
                    Entry::Vacant(_) => {
                        // dbg!(&pos, &p);
                        pos = *p;
                        continue 'step;
                    }
                    Entry::Occupied(_) => {}
                }
            }
            break;
        }
        map.grid.entry(pos).or_insert(Tile::Sand);
    }

    map.grid.iter().fold(0, |acc, e| match e.1 {
        Tile::Sand => acc + 1,
        Tile::Rock => acc,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let map = parse("input/14-sample");
        dbg!(&map);
        assert_eq!(map.grid[&(498, 5)], Tile::Rock);
        assert_eq!(map.lowest_point, 9);
        assert_eq!(map.grid.len(), 20);
    }

    #[test]
    fn test_calculate() {
        for (file, floor, out) in [
            ("input/14-sample", false, 24),
            ("input/14-sample", true, 93),
            ("input/14", false, 838),
            ("input/14", true, 27539),
        ] {
            assert_eq!(calculate(file, floor), out);
        }
    }
}
