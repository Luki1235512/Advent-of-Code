import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  const numbers = input.split("\r\n").map(Number);

  return numbers.reduce((increase, current, index) => {
    return index > 0 && current > numbers[index - 1] ? increase + 1 : increase;
  }, 0);
}

function part2(input: string): number {
  const numbers = input.split("\r\n").map(Number);
  let measurementIncreases = 0;

  for (let i = 3; i < numbers.length; i++) {
    if (numbers[i] > numbers[i - 3]) {
      measurementIncreases++;
    }
  }

  return measurementIncreases;
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
