import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  let floor = 0;

  for (const char of input) {
    floor += char === "(" ? 1 : -1;
  }

  return floor;
}

function part2(input: string): number {
  let floor = 0;

  for (const [index, char] of input.split("").entries()) {
    floor += char === "(" ? 1 : -1;

    if (floor === -1) {
      return index + 1;
    }
  }

  return floor;
}

const input = readInput();
  
console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
