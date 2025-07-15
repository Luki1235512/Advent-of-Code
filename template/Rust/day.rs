mod day_ {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn read_input() -> String {
        let input_path: PathBuf = Path::new(file!()).parent().unwrap().join("../input/.txt");
        fs::read_to_string(input_path)
            .expect("Failed to read input file")
            .trim()
            .to_string()
    }

    fn part1(_input: &str) -> i32 {
        0
    }

    fn part2(_input: &str) -> i32 {
        0
    }

    pub fn run() {
        let input: String = read_input();

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    }
}

pub use day_::run;
