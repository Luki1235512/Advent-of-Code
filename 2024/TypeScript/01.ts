import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  const pairs = input
    .split("\r\n")
    .map((line) => line.split("   ").map(Number));
  const left = pairs.map((pair) => pair[0]).sort();
  const right = pairs.map((pair) => pair[1]).sort();

  return left.reduce(
    (sum, leftNum, i) => sum + Math.abs(leftNum - right[i]),
    0
  );
}

function part2(input: string): number {
  const pairs = input
    .split("\r\n")
    .map((line) => line.split("   ").map(Number));
  const left = pairs.map((pair) => pair[0]);
  const right = pairs.map((pair) => pair[1]);

  const rightFreq = right.reduce((freq, num) => {
    freq.set(num, (freq.get(num) || 0) + 1);
    return freq;
  }, new Map<number, number>());

  return left.reduce((sum, leftNum) => {
    return sum + leftNum * (rightFreq.get(leftNum) || 0);
  }, 0);
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
