use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let output = std::fs::read_to_string("input/7").unwrap();
    println!("{}", find_smaller_dirs(&output, 100000));
    println!("{}", find_smallest_dir_to_free(&output, 70000000, 30000000));
}

fn parse_output(output: &str) -> HashMap<String, u32> {
    let mut dirs = HashMap::<String, u32>::new();
    let mut current_dir = "".to_string();

    for line in output.lines() {
        if line.starts_with("$ cd") {
            let path = line
                .split_whitespace()
                .nth(2)
                .expect("cd should have an argument");
            if path == "/" {
                current_dir = "".to_string();
            } else if path == ".." {
                current_dir = dir_parents(current_dir.clone())[0].clone();
            } else {
                current_dir = format!("{}/{}", &current_dir, &path).to_string();
            }
        } else {
            let size = line.split(' ').next().unwrap().parse::<u32>();
            if let Ok(..) = size {
                let size = size.unwrap();
                for dir in
                    itertools::chain!([current_dir.clone()], dir_parents(current_dir.clone()))
                {
                    dirs.entry(String::from(&dir))
                        .and_modify(|n| *n += size)
                        .or_insert(size);
                }
            }
        }
    }

    dirs
}

fn dir_parents(dir: String) -> Vec<String> {
    let mut parents = vec![];
    let mut d = dir.as_str();

    loop {
        let mut split = d.rsplitn(2, '/');
        let parent = split.nth(1);
        if parent.is_none() {
            break;
        }
        parents.push(parent.unwrap().to_string());
        d = parent.unwrap();
    }

    parents
}

fn find_smaller_dirs(output: &str, max_size: u32) -> u32 {
    let dirs = parse_output(output);

    let mut total = 0;

    for (_, v) in dirs.iter() {
        if v < &max_size {
            total += v;
        }
    }

    total
}

fn find_smallest_dir_to_free(output: &str, fs_size: u32, required_space: u32) -> u32 {
    let dirs = parse_output(output);
    let existing_space = fs_size - dirs[""];
    let space_needed = required_space - existing_space;

    dirs.into_values()
        .filter(|n| *n > space_needed)
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smaller_dirs() {
        let output = std::fs::read_to_string("input/7-sample").unwrap();
        assert_eq!(find_smaller_dirs(&output, 100000), 95437);
    }

    #[test]
    fn test_find_smallest_dir_to_free() {
        let output = std::fs::read_to_string("input/7-sample").unwrap();
        assert_eq!(
            find_smallest_dir_to_free(&output, 70000000, 30000000),
            24933642
        );
    }

    #[test]
    fn test_dir_parents() {
        assert_eq!(dir_parents("a/test/dir".to_string()), vec!["a/test", "a"]);
    }
}
