import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  return input.split("\r\n").reduce((sum, elem) => sum + Number(elem), 0);
}

function part2(input: string): number {
  let currentFrequency = 0;
  const frequencies = new Set<number>();

  while (true) {
    for (const elem of input.split("\r\n")) {
      currentFrequency += Number(elem);

      const frequency = currentFrequency;
      if (frequencies.has(frequency)) {
        return frequency;
      }
      frequencies.add(frequency);
    }
  }
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
