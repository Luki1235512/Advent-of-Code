import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  return 0;
}

function part2(input: string): number {
  return 0;
}

const input = readInput();
  
console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
