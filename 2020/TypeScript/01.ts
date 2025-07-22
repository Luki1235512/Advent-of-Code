import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  const numbers = input.split("\r\n").map(Number);

  for (let i = 0; i < numbers.length; i++) {
    const complement = 2020 - numbers[i];
    const complementIndex = numbers.indexOf(complement, i + 1);

    if (complementIndex !== -1) {
      return numbers[i] * complement;
    }
  }

  return 0;
}

function part2(input: string): number {
  const numbers = input
    .split("\r\n")
    .map(Number)
    .sort((a, b) => a - b);

  for (let i = 0; i < numbers.length - 2; i++) {
    let left = i + 1;
    let right = numbers.length - 1;

    while (left < right) {
      const sum = numbers[i] + numbers[left] + numbers[right];

      if (sum === 2020) {
        return numbers[i] * numbers[left] * numbers[right];
      } else if (sum < 2020) {
        left++;
      } else {
        right--;
      }
    }
  }

  return 0;
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
