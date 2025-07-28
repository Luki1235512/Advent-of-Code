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
        input
            .lines()
            .map(|mass| {
                let fuel = mass.parse::<i32>().unwrap();
                (fuel / 3) - 2
            })
            .sum()
    }

    fn part2(input: &str) -> i32 {
        let mut fuel = 0;

        for mass in input.split("\r\n") {
            let mut tmp_fuel = (mass.parse::<i32>().unwrap() / 3) - 2;

            while tmp_fuel > 0 {
                fuel += tmp_fuel;
                tmp_fuel = (tmp_fuel / 3) - 2;
            }
        }

        fuel
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
