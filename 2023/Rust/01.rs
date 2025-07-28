mod day_01 {
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
        input.lines().map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                0
            } else {
                let first = digits[0];
                let last = digits[digits.len() - 1];
                format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
            }
        }).sum()
    }

    fn part2(input: &str) -> i32 {
        input
        .lines()
        .filter_map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            
            let mut digits = line.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next()?;
            let last = digits.last().unwrap_or(first);
            
            Some(format!("{}{}", first, last).parse::<i32>().ok()?)
        })
        .sum()
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
