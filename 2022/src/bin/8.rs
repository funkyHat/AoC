use itertools::Itertools;
fn main() {
    let input = std::fs::read_to_string("input/8").unwrap();
    let trees = parse_forest(&input);
    println!("{}", count_visible(&trees));
    println!("{}", scenic_score(&trees));
}

fn count_visible(trees: &Vec<Vec<u8>>) -> u32 {
    let mut visible_total = 0;

    dbg!(&trees);

    for (i, row) in trees.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            if i == 0
                || j == 0
                || i == trees.len() - 1
                || j == row.len() - 1
                || row.iter().take(j).all(|h| h < t)
                || row.iter().skip(j + 1).all(|h| h < t)
                || trees.iter().take(i).map(|v| &v[j]).all(|h| h < t)
                || trees.iter().skip(i + 1).map(|v| &v[j]).all(|h| h < t)
            {
                visible_total += 1;
                continue;
            };
        }
    }

    visible_total
}

fn scenic_score(trees: &[Vec<u8>]) -> usize {
    let mut max_score = 0;

    for (i, row) in trees.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            let score = shorter_and_first_same(t, row.iter().take(j).rev())
                * shorter_and_first_same(t, row.iter().skip(j + 1))
                * shorter_and_first_same(t, trees.iter().take(i).map(|v| &v[j]).rev())
                * shorter_and_first_same(t, trees.iter().skip(i + 1).map(|v| &v[j]));
            max_score = std::cmp::max(max_score, score);
        }
    }

    max_score
}

fn shorter_and_first_same<'a, I>(height: &u8, mut trees: I) -> usize
where
    I: Iterator<Item = &'a u8>,
    I: Clone,
{
    let mut count = trees.take_while_ref(|h| h < &height).count();
    if trees.next().is_some() {
        count += 1;
    }

    count
}

fn parse_forest(forest: &str) -> Vec<Vec<u8>> {
    let mut trees: Vec<Vec<u8>> = vec![];
    for row in forest.lines() {
        let mut tree_row: Vec<u8> = vec![];
        for c in row.chars() {
            tree_row.push(c.to_digit(10).unwrap() as u8);
        }
        trees.push(tree_row);
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible() {
        for (file, score) in [("input/8-sample", 21), ("input/8", 1647)] {
            let input = std::fs::read_to_string(file).unwrap();
            let trees = parse_forest(&input);
            assert_eq!(count_visible(&trees), score);
        }
    }

    #[test]
    fn test_scenic_score() {
        for (file, score) in [("input/8-sample", 8), ("input/8", 392080)] {
            let input = std::fs::read_to_string(file).unwrap();
            let trees = parse_forest(&input);
            assert_eq!(scenic_score(&trees), score);
        }
    }
}
