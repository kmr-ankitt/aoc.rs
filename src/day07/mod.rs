use std::fs;

pub fn run(input_path: &str) {
    let input_str = fs::read_to_string(input_path)
        .expect("Cannot find input file.");

    let input_data: Vec<Vec<i64>> = parse_input(&input_str);
    let (res_part1, res_part2) = day7_solve(input_data);

    println!("Part 1 = {}", res_part1);
    println!("Part 2 = {}", res_part2);
}

fn propagate_beam(
    beam_comming: &[i64],
    line_beam_incomming: &[i64],
) -> (Vec<i64>, i64) {
    let mut line_after_beam_propagation: Vec<i64> = line_beam_incomming.to_vec();
    let mut split_count: i64 = 0;

    for (i, &beam_strenght) in beam_comming.iter().enumerate() {
        if beam_strenght == -1 {
            continue;
        }

        match line_beam_incomming[i] {
            -1 => {
                if i + 1 < line_after_beam_propagation.len() {
                    line_after_beam_propagation[i + 1] += beam_strenght;
                }
                if i > 0 {
                    line_after_beam_propagation[i - 1] += beam_strenght;
                }

                if beam_strenght != 0 {
                    split_count += 1;
                }
            }
            _ => line_after_beam_propagation[i] += beam_strenght,
        }
    }

    (line_after_beam_propagation, split_count)
}

fn day7_solve(input_lines: Vec<Vec<i64>>) -> (i64, i64) {
    let mut line_after_beam_propagation = input_lines[0].clone();
    let mut total_split_count = 0;

    for curr in &input_lines[1..] {
        let (beam_propagated, line_split_count) =
            propagate_beam(&line_after_beam_propagation, curr);

        line_after_beam_propagation = beam_propagated;
        total_split_count += line_split_count;
    }

    (
        total_split_count,
        line_after_beam_propagation.iter().sum(),
    )
}

fn parse_input(input_str: &str) -> Vec<Vec<i64>> {
    input_str
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '^' => -1,
                    '.' => 0,
                    _ => 1, 
                })
                .collect()
        })
        .collect()
}

