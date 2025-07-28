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

        for (i, &num) in numbers.iter().enumerate() {
            let complement = 2020 - num;

            if let Some(_) = numbers[i + 1..].iter().position(|&x| x == complement) {
                return num * complement;
            }
        }

        -1
    }

    fn part2(input: &str) -> i32 {
        let mut numbers: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
        numbers.sort();

        for i in 0..numbers.len() - 2 {
            let mut left = i + 1;
            let mut right = numbers.len() - 1;

            while left < right {
                let sum = numbers[i] + numbers[left] + numbers[right];

                if sum == 2020 {
                    return numbers[i] * numbers[left] * numbers[right];
                } else if sum < 2020 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }


        -1
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_01::run;
