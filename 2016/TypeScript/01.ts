import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

const moves = [[-1, 0], [0, -1], [1, 0], [0, 1]];

function part1(input: string): number {
  let currentDir = 0;
  let x = 0;
  let y = 0;

  for (const instruction of input.split(", ")) {
    const turn = instruction[0];
    const distance = Number(instruction.slice(1));

    currentDir = (currentDir + (turn === "R" ? 1 : -1) + 4) % 4;

    x += moves[currentDir][0] * distance;
    y += moves[currentDir][1] * distance;
  }

  return Math.abs(x) + Math.abs(y)
}

function part2(input: string): number {
  let currentDir = 0;
  let x = 0;
  let y = 0;
  const visited = new Set<string>();
  visited.add("0,0");

  for (const instruction of input.split(", ")) {
    const turn = instruction[0];
    const distance = Number(instruction.slice(1));

    currentDir = (currentDir + (turn === "R" ? 1 : -1) + 4) % 4;

    for (let _step = 0; _step < distance; _step++) {
      x += moves[currentDir][0];
      y += moves[currentDir][1];

      const location = `${x},${y}`;
      if (visited.has(location)) {
        return Math.abs(x) + Math.abs(y);
      }
      visited.add(location);
    }
  }

  return Math.abs(x) + Math.abs(y)
}

const input = readInput();
  
console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
