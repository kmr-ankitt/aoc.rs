
use std::{env, panic};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: aoc <day>");
        std::process::exit(1);
    }

    let day = &args[1];
    let file_path = format!("day{}/mod.rs", day);

    if let Err(e) = std::fs::metadata(&file_path) {
        panic!("day {} not found: {}", day, e);
    }

    let input_file_path = format!("day{}/input.txt", day);

    match day.as_str() {
        _ => panic!("day {} not implemented yet", day),
    }
}
