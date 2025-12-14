use std::collections::HashMap;
use std::fs;
use std::cmp::Reverse;
use std::time::Instant;

use keyed_priority_queue::KeyedPriorityQueue;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

fn distance(a: &Vec3d, b: &Vec3d) -> f64 {
    ((a.x - b.x).powi(2)
        + (a.y - b.y).powi(2)
        + (a.z - b.z).powi(2))
        .sqrt()
}

pub fn run(input_path: &str) {
    let data = fs::read_to_string(input_path)
        .expect("Could not read input file");

    let timer = Instant::now();

    let mut junctions: Vec<Vec3d> = Vec::new();
    for line in data.lines() {
        let mut it = line.split(',');
        junctions.push(Vec3d {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
            z: it.next().unwrap().parse().unwrap(),
        });
    }

    let mut queue: KeyedPriorityQueue<(usize, usize), Reverse<OrderedFloat<f64>>> =
        KeyedPriorityQueue::new();

    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            queue.push(
                (i, j),
                Reverse(OrderedFloat(distance(&junctions[i], &junctions[j]))),
            );
        }
    }

    let mut circuits: Vec<usize> = (0..junctions.len()).collect();
    let mut num_connections = 0;
    let mut part1_done = false;

    while let Some(((i, j), Reverse(_))) = queue.pop() {
        if circuits[i] != circuits[j] {
            let (new_idx, old_idx) = if circuits[i] < circuits[j] {
                (circuits[i], circuits[j])
            } else {
                (circuits[j], circuits[i])
            };

            for c in circuits.iter_mut() {
                if *c == old_idx {
                    *c = new_idx;
                }
            }

            if is_last(&circuits) {
                println!(
                    "Part 2 = {}",
                    junctions[i].x * junctions[j].x
                );
                println!("Part 2 time: {} ms", timer.elapsed().as_millis());
                break;
            }
        }

        num_connections += 1;

        if num_connections == 1000 && !part1_done {
            let mut counts: HashMap<usize, i32> = HashMap::new();
            for &c in circuits.iter() {
                *counts.entry(c).or_insert(0) += 1;
            }

            let mut vals: Vec<i32> = counts.values().cloned().collect();
            vals.sort_by_key(|&v| Reverse(v));

            println!(
                "Part 1 = {}",
                vals[0] * vals[1] * vals[2]
            );
            println!("Part 1 time: {} ms", timer.elapsed().as_millis());

            part1_done = true;
        }
    }
}

fn is_last(circuits: &Vec<usize>) -> bool {
    circuits.iter().all(|&c| c == 0)
}

