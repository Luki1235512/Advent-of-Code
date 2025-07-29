mod day_01 {
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn read_input() -> String {
        let input_path: PathBuf = Path::new(file!()).parent().unwrap().join("../input/01.txt");
        fs::read_to_string(input_path)
            .expect("Failed to read input file")
            .trim()
            .to_string()
    }

    fn part1(input: &str) -> i32 {
        let pairs: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (nums[0], nums[1])
            })
            .collect();

        let mut left: Vec<i32> = pairs.iter().map(|(l, _)| *l).collect();
        let mut right: Vec<i32> = pairs.iter().map(|(_, r)| *r).collect();

        left.sort();
        right.sort();

        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum()
    }

    fn part2(input: &str) -> i32 {
        let pairs: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (nums[0], nums[1])
            })
            .collect();

        let left: Vec<i32> = pairs.iter().map(|(l, _)| *l).collect();
        let right: Vec<i32> = pairs.iter().map(|(_, r)| *r).collect();

        let mut right_freq: HashMap<i32, i32> = HashMap::new();
        for num in right {
            *right_freq.entry(num).or_insert(0) += 1;
        }

        left.iter()
            .map(|left_num| left_num * right_freq.get(left_num).unwrap_or(&0))
            .sum()
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
