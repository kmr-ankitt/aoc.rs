use std::{collections::VecDeque, fs};

use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashSet;

const EPSILON: f64 = 1e-9;

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

pub fn run(input_path: &str) {
    let input = fs::read_to_string(input_path)
        .expect("Cannot read input file");

    let part1 = p1(&input);
    let part2 = p2(&input);

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}

fn parse(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.trim().lines().map(Machine::from)
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();

        let lights = parts
            .next()
            .map(|l| {
                l.trim_matches(['[', ']'])
                    .chars()
                    .rev()
                    .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            })
            .unwrap();

        let mut parts: Vec<&str> = parts.collect();

        let joltages = parts
            .pop()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let mut buttons: Vec<Vec<usize>> = parts
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        // Heuristic optimization from original solution
        buttons.sort_by_key(|b| std::cmp::Reverse(b.len()));

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

/* ---------------- Part 1 ---------------- */

fn p1(input: &str) -> usize {
    parse(input)
        .map(|machine| {
            let mut q = VecDeque::from([(0usize, 0usize)]);
            let mut seen = FxHashSet::default();
            seen.insert(0);

            while let Some((state, dist)) = q.pop_front() {
                if state == machine.lights {
                    return dist;
                }

                for button in machine.buttons.iter() {
                    let next = button.iter().fold(state, |acc, &b| acc ^ (1 << b));
                    if seen.insert(next) {
                        q.push_back((next, dist + 1));
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

/* ---------------- Part 2 ---------------- */

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        for (c, button) in machine.buttons.iter().enumerate() {
            for &r in button {
                if r < rows {
                    data[r][c] = 1.0;
                }
            }
        }

        for (r, &v) in machine.joltages.iter().enumerate() {
            data[r][cols] = v as f64;
        }

        let mut m = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        m.gaussian_elimination();
        m
    }

    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {
            let (best_row, best_val) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            if best_val < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            let pivot_val = self.data[pivot][col];
            for v in &mut self.data[pivot][col..=self.cols] {
                *v /= pivot_val;
            }

            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        for (v, pv) in self.data[r][col..=self.cols].iter_mut().zip(pivot_row) {
                            *v -= factor * pv;
                        }
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        self.independents.extend(col..self.cols);
    }

    fn valid(&self, values: &[usize]) -> Option<usize> {
        let mut total = values.iter().sum::<usize>();

        for row in 0..self.dependents.len() {
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &c)| {
                    acc - self.data[row][c] * values[i] as f64
                });

            if val < -EPSILON {
                return None;
            }
            let r = val.round();
            if (val - r).abs() > EPSILON {
                return None;
            }
            total += r as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], best: &mut usize, max: usize) {
    if idx == matrix.independents.len() {
        if let Some(v) = matrix.valid(values) {
            *best = (*best).min(v);
        }
        return;
    }

    let prefix: usize = values[..idx].iter().sum();
    for v in 0..max {
        if prefix + v >= *best {
            break;
        }
        values[idx] = v;
        dfs(matrix, idx + 1, values, best, max);
    }
}

fn p2(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .map(|machine| {
            let matrix = Matrix::from_machine(&machine);

            let max = machine.joltages.iter().max().unwrap() + 1;
            let mut best = usize::MAX;
            let mut values = vec![0; matrix.independents.len()];

            dfs(&matrix, 0, &mut values, &mut best, max);
            best
        })
        .sum()
}
