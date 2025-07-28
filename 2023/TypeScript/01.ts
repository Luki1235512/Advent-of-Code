import * as fs from "fs";
import * as path from "path";

function readInput(): string {
  const inputPath = path.join(__dirname, "../input/01.txt");
  return fs.readFileSync(inputPath, "utf8").trim();
}

function part1(input: string): number {
  return input.split("\r\n").reduce((sum, line) => {
    const digits = line.match(/\d/g) || ["0"];
    return sum + Number(digits[0] + digits.at(-1));
  }, 0);
}

function part2(input: string): number {
  const digitMap: { [key: string]: string } = {
    one: "1",
    two: "2",
    three: "3",
    four: "4",
    five: "5",
    six: "6",
    seven: "7",
    eight: "8",
    nine: "9",
  };

  return input.split("\r\n").reduce((sum, line) => {
    const matches = [
      ...line.matchAll(
        /(?=(\d|one|two|three|four|five|six|seven|eight|nine))/g
      ),
    ];
    const digits = matches.map((match) => digitMap[match[1]] || match[1]);

    return sum + Number(digits[0] + digits[digits.length - 1]);
  }, 0);
}

const input = readInput();

console.log("Part 1:", part1(input));
console.log("Part 2:", part2(input));
