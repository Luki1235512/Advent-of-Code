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
        let numbers: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    
        numbers
            .windows(2)
            .filter(|window| window[1] > window[0])
            .count() as i32
    }

    fn part2(input: &str) -> i32 {
        let numbers: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

        numbers
            .windows(4)
            .filter(|window| {
                let sum1 = window[0] + window[1] + window[2];
                let sum2 = window[1] + window[2] + window[3];
                sum2 > sum1
            })
            .count() as i32
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
