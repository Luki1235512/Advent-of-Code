use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn read_input() -> String {
    let input_path: PathBuf = Path::new(file!()).parent().unwrap().join("../input/01.txt");
    fs::read_to_string(input_path)
        .expect("Failed to read input file")
        .trim()
        .to_string()
}

const MOVES: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

fn part1(input: &str) -> i32 {
    let mut current_dir = 0;
    let mut x = 0;
    let mut y = 0;

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse().unwrap_or(0);

        current_dir = (current_dir + if turn == 'R' { 1 } else { -1 } + 4) % 4;
        x += MOVES[current_dir as usize][0] * distance;
        y += MOVES[current_dir as usize][1] * distance;
    }

    return x.abs() + y.abs();
}

fn part2(input: &str) -> i32 {
    let mut current_dir = 0;
    let mut x = 0;
    let mut y = 0;
    let mut visited = HashSet::new();

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse().unwrap_or(0);

        current_dir = (current_dir + if turn == 'R' { 1 } else { -1 } + 4) % 4;

        for _step in 0..distance {
            x += MOVES[current_dir as usize][0];
            y += MOVES[current_dir as usize][1];

            let location = format!("{},{}", x, y);
            if visited.contains(&location) {
                return x.abs() + y.abs();
            }
            visited.insert(location);
        }
    }

    return x.abs() + y.abs();
}

fn main() {
    let input: String = read_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
