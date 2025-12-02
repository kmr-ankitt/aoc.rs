use std::fs;

pub fn run(input: &str){
    let testcases = fs::read_to_string(input).expect("can't read file");

    let mut dial: u32 = 50;
    let mut count : u32 = 0;
    part_one(&mut dial, &mut count, &testcases);

    dial = 50;
    count = 0;
    println!("-------------------");

    part_two(&mut dial, &mut count, &testcases);
}

fn part_one(dial: &mut u32, count: &mut u32, testcases: &str){
    for line in testcases.lines() {
        if line.len() < 2 {
            continue;
        }

        let (direction, magnitude) = line.split_at(1);
        let magnitude : u32 = magnitude.parse().unwrap();

        match direction {
            "L" => {
                *dial = (*dial + 100 - (magnitude % 100)) % 100;
            },
            "R" => {
                *dial = (*dial + magnitude) % 100;
            }
            _ => panic!("invalid direction"),
        }
        if *dial == 0 {
            *count += 1;
        }
    }

    println!("final dial position: {}", dial);
    println!("number of times dial hit 0: {}", count);
}

fn part_two(dial: &mut u32, count: &mut u32, testcases: &str) { 
    for line in testcases.lines() {
        if line.len() < 2 {
            continue;
        }

        let (direction, magnitude) = line.split_at(1);
        let magnitude : u32 = magnitude.parse().unwrap();

        let step = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("invalid direction"),
        };

        for _ in 0..magnitude {
            let new_val = (*dial as i32 + 100 + step) % 100;
            *dial = new_val as u32;

            if *dial == 0 {
                *count += 1;
            }
        }
    }

    println!("final dial position: {}", dial);
    println!("total number of times dial hit 0: {}", count);
}
