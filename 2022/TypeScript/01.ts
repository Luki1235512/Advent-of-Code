import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

const calories = readInput()
  .split("\r\n\r\n")
  .map((elf) =>
    elf.split("\r\n").reduce((sum, calories) => sum + Number(calories), 0)
  );

function part1(): number {
  return Math.max(...calories);
}

function part2(): number {
  return calories
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((sum, calories) => sum + calories, 0);
}

console.log("Part 1:", part1());
console.log("Part 2:", part2());
