use std::fs;

pub fn run(input_path: &str) {
    let input = fs::read_to_string(input_path).expect("failed to read input file");

    let p1 = part_1(&input);
    let p2 = part_2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges: Vec<[u64; 2]> = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        ranges.push([start, end]);
    }

    ranges.sort_by_key(|r| r[0]);

    let mut merged: Vec<[u64; 2]> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range[0] <= last[1] + 1 {
                last[1] = last[1].max(range[1]);
                continue;
            }
        }
        merged.push(range);
    }

    // Count ingredients inside any merged range
    lines
        .filter(|line| {
            let value: u64 = line.parse().unwrap();

            merged
                .binary_search_by(|&[start, end]| {
                    if value < start {
                        std::cmp::Ordering::Greater
                    } else if value > end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count()
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut ranges: Vec<[u64; 2]> = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        ranges.push([start, end]);
    }

    ranges.sort_by_key(|r| r[0]);

    let mut merged: Vec<[u64; 2]> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range[0] <= last[1] + 1 {
                last[1] = last[1].max(range[1]);
                continue;
            }
        }
        merged.push(range);
    }

    merged.into_iter().map(|[start, end]| end - start + 1).sum()
}
