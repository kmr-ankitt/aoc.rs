
use std::{env, panic};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: aoc <day>");
        std::process::exit(1);
    }

    let day : &String = &args[1];

    let input_file_path = format!("src/day{}/input.txt", day);

    match day.as_str() {
        _ => panic!("day {} not implemented yet", day),
    }
}
