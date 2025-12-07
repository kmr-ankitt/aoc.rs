use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Alignment {
    Left,
    Right,
    None,
}

pub fn run(input_path: &str) {
    let input = fs::read_to_string(input_path)
        .expect("failed to read input file");

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let (part1, part2) = solve(&lines);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(lines: &[String]) -> (usize, usize) {
    let mut ops: Vec<(usize, char, Alignment)> =
        lines.last().unwrap()
            .char_indices()
            .filter(|(_, c)| !c.is_whitespace())
            .map(|(i, op)| (i, op, Alignment::None))
            .collect();

    ops.push((lines[0].len() + 2, '?', Alignment::None));

    let mut columns: Vec<Vec<usize>> = Vec::with_capacity(ops.len());

    for idx in 0..ops.len() - 1 {
        let next = ops[idx + 1];
        let op = &mut ops[idx];
        let mut digits = Vec::with_capacity(lines.len() - 1);

        for line in lines.iter().take(lines.len() - 1) {
            let mut offset = op.0;

            if line[offset..].starts_with(' ') {
                op.2 = Alignment::Right;
                offset += line[offset..]
                    .find(|c: char| c.is_ascii_digit())
                    .unwrap();
            }

            if op.2 == Alignment::None {
                let num_finish = line[offset..]
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .unwrap()
                    .0 + 2 + offset;

                let next_offset = next.0;
                if num_finish < next_offset {
                    op.2 = Alignment::Left;
                }
            }

            let slice = line[offset..]
                .split_whitespace()
                .next()
                .unwrap();

            let digit = slice.trim().parse::<usize>().unwrap();
            digits.push(digit);
        }

        columns.push(digits);
    }

    let part1 = columns.iter().zip(ops.iter())
        .map(|(col, op)| {
            let init = col[0];
            col.iter().skip(1).fold(init, |acc, elem| {
                match op.1 {
                    '+' => acc + elem,
                    '-' => acc - elem,
                    '*' => acc * elem,
                    '/' => acc / elem,
                    _ => unreachable!(),
                }
            })
        })
        .sum::<usize>();

    fn num_digits(mut n: usize) -> usize {
        if n == 0 { return 1; }
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }

    fn get_num(alignment: Alignment, nums: &mut [usize]) -> usize {
        let mut acc = 0;
        let longest = nums.iter().map(|n| num_digits(*n)).max().unwrap();

        for num in nums.iter_mut() {
            if *num == 0 {
                continue;
            }

            if num_digits(*num) < longest && alignment == Alignment::Left {
                continue;
            }

            acc *= 10;
            acc += *num % 10;
            *num /= 10;
        }

        acc
    }

    let part2 = columns.iter_mut().zip(ops.iter())
        .map(|(col, op)| {
            let longest = col.iter().map(|n| num_digits(*n)).max().unwrap();

            let mut init = get_num(op.2, col);

            for _ in 1..longest {
                let n = get_num(op.2, col);
                init = match op.1 {
                    '+' => init + n,
                    '-' => init - n,
                    '*' => init * n,
                    '/' => init / n,
                    _ => unreachable!(),
                };
            }

            init
        })
        .sum::<usize>();

    (part1, part2)
}
