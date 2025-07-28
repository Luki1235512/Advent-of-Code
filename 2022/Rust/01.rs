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

    fn get_calories() -> Vec<i32> {
        read_input()
            .split("\r\n\r\n")
            .map(|elf| {
                elf.split("\r\n")
                    .map(|calories| calories.parse::<i32>().unwrap())
                    .sum()
            })
            .collect()
    }

    fn part1() -> i32 {
        get_calories().into_iter().max().unwrap()
    }

    fn part2() -> i32 {
        let mut calories = get_calories();
        calories.sort_by(|a, b| b.cmp(a));
        calories.iter().take(3).sum()
    }

    pub fn run() {
        println!("Part 1: {}", part1());
        println!("Part 2: {}", part2());
    }
}

pub use day_01::run;
