# Advent-of-Code

Solutions for Advent of Code challenges implemented in both TypeScript and Rust.

## Project Structure

```
{year}/
├── input/           # Input files
├── Rust/            # Rust solutions
└── TypeScript/      # TypeScript solutions
templates/           # Template files for new solutions
```

## Running Solutions

### TypeScript

```bash
npm run day {year}\TypeScript\{day}.ts
```

### Rust

```bash
cargo run --bin {year}
```

## Setup

### Prerequisites

- Node.js and npm (for TypeScript solutions)
- Rust and Cargo (for Rust solutions)

## Templates

Template files are available in the `templates/` directory:

- [`templates/tsTemplate.ts`](templates/tsTemplate.ts) - TypeScript template
- [`templates/rustTemplate.rs`](templates/rustTemplate.rs) - Rust template
