import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  let sum = 0;

  for (let i = 0; i < input.length; i++) {
    const nextIndex = (i + 1) % input.length;
    
    if (input[i] === input[nextIndex]) {
      sum += Number(input[i]);
    }
  }

  return sum;
}

function part2(input: string): number {
  const halfway = input.length / 2;
  let sum = 0;

  for (let i = 0; i < input.length; i++) {
    const compareIndex = (i + halfway) % input.length;
    if (input[i] === input[compareIndex]) {
      sum += Number(input[i]);
    }
  }

  return sum;
}

const input = readInput();
  
console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
