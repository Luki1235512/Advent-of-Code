import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  let fuel = 0;

  for (const mass of input.split("\r\n")) {
    fuel += Math.floor(Number(mass) / 3) - 2;
  }

  return fuel;
}

function part2(input: string): number {
  let fuel = 0;

  for (const mass of input.split("\r\n")) {
    let tmpFuel = Math.floor(Number(mass) / 3) - 2;

    while (tmpFuel > 0) {
      fuel += tmpFuel;
      tmpFuel = Math.floor(tmpFuel / 3) - 2;
    }
  }

  return fuel;
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
