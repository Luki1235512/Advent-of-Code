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
        return input
            .split("\r\n")
            .map(|elem| elem.parse::<i32>().unwrap())
            .sum();
    }

    fn part2(input: &str) -> i32 {
        let mut current_frequency = 0;
        let mut frequencies = std::collections::HashSet::new();

        loop {
            for elem in input.split("\r\n") {
                current_frequency += elem.parse::<i32>().unwrap();

                if frequencies.contains(&current_frequency) {
                    return current_frequency;
                }
                frequencies.insert(current_frequency);
            }
        }
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
