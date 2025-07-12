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
    let mut floor: i32 = 0;

    for char in input.chars() {
        floor += if char == '(' { 1 } else { -1 };
    }

    return floor;
}

fn part2(input: &str) -> i32 {
    let mut floor: i32 = 0;

    for (index, char) in input.chars().enumerate() {
        floor += if char == '(' { 1 } else { -1 };

        if floor == -1 {
            return (index + 1) as i32;
        }
    }

    return floor;
}

fn main() {
    let input: String = read_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
