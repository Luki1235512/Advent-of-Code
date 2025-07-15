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
        let chars: Vec<char> = input.chars().collect();
        let mut sum = 0;

        for index in 0..chars.len() {
            let next_index = (index + 1) % chars.len();

            if chars[index] == chars[next_index] {
                sum += chars[index].to_digit(10).unwrap() as i32;
            }
        }

        return sum;
    }

    fn part2(input: &str) -> i32 {
        let chars: Vec<char> = input.chars().collect();
        let halfway = chars.len() / 2;
        let mut sum = 0;

        for index in 0..chars.len() {
            let compare_index = (index + halfway) % chars.len();
            if chars[index] == chars[compare_index] {
                sum += chars[index].to_digit(10).unwrap() as i32;
            }
        }

        return sum;
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
