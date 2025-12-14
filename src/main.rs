mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::{env, panic};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: aoc <day> <input_file_path>");
        std::process::exit(1);
    }

    let day : &String = &args[1];
    let input : &String = &args[2];

    let input_file_path = format!("src/day{}/{}.txt", day, input);

    match day.as_str() {
        "01" => day01::run(&input_file_path),
        "02" => day02::run(&input_file_path),
        "03" => day03::run(&input_file_path), 
        "04" => day04::run(&input_file_path),
        "05" => day05::run(&input_file_path),
        "06" => day06::run(&input_file_path),
        "07" => day07::run(&input_file_path),
        "08" => day08::run(&input_file_path),
        _ => panic!("day {} not implemented yet", day),
    }
}
