use std::fs;

pub fn run(input: &str) {
    let data = fs::read_to_string(input).unwrap();

    let testcases: Vec<Vec<u32>> = data
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    part_one(&testcases);
    part_two(&testcases);
}

fn part_one(testcases: &Vec<Vec<u32>>) {
    let mut sum: u32 = 0;
    for bank in testcases {
        let mut best = 0;

        for i in 0..bank.len() {
            for j in i + 1..bank.len() {
                let num = bank[i] * 10 + bank[j];
                if num > best {
                    best = num;
                }
            }
        }

        sum += best;
        println!("{}", best);
    }
    println!("Sum is {}", sum);
}



fn part_two(testcases: &Vec<Vec<u32>>) {
    let mut sum: u64 = 0;

    for bank in testcases {
        // remove = bank.len() - keep
        let remove =  bank.len() - 12;  // or your logic

        let best_digits = max_joltage_big(bank, remove);

        let mut best_num: u64 = 0;
        for d in best_digits {
            best_num = best_num * 10 + d as u64;
        }

        println!("{}", best_num);
        sum += best_num;
    }

    println!("Total: {}", sum);
}

fn max_joltage_big(bank: &Vec<u32>, remove: usize) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut to_remove = remove;

    for &digit in bank {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }

    // If any removals left, remove from end
    stack.truncate(stack.len() - to_remove);

    stack
}
