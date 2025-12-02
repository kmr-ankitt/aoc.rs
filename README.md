# AOC

contains solutions for advent of code problems in rust.

## Usage

```bash
cargo run <day> <input_filename>
```

Example:

```bash
cargo run 01 sample
```
## Project Structure

```
day01/
  ├─ mod.rs
  ├─ sample.txt
  └─ input.txt
day02/
  ├─ mod.rs
  ├─ sample.txt
  └─ input.txt
```

## Adding a New Day

1. Create folder:

```
dayXX/mod.rs
dayXX/sample.txt
dayXX/input.txt
```

2. Add match arm:

```rust
"XX" => dayXX::run(&input_file_path),
```

That's it.
