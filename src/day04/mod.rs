use std::{collections::VecDeque, fs};

pub fn run(input: &str) {
   let testcases = fs::read_to_string(input).unwrap();

    let grid: Vec<Vec<char>> = testcases
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n = grid.len();
    let m = grid[0].len();

    let dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];

    // -------------------------
    // PART 1
    // -------------------------
    let mut accessible = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '@' { continue; }

            let count = dirs.iter().filter(|(di, dj)| {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                ni >= 0 && ni < n as isize &&
                nj >= 0 && nj < m as isize &&
                grid[ni as usize][nj as usize] == '@'
            }).count();

            if count < 4 {
                accessible += 1;
            }
        }
    }

    println!("Part 1: {}", accessible);

    // -------------------------
    // PART 2  (Topological removal)
    // -------------------------

    // initially set all to i32::MAX
    let mut adj = vec![vec![i32::MAX; m]; n];
    let mut stack = VecDeque::new();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '@' { continue; }

            let count = dirs.iter().filter(|(di, dj)| {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                ni >= 0 && ni < n as isize &&
                nj >= 0 && nj < m as isize &&
                grid[ni as usize][nj as usize] == '@'
            }).count() as i32;

            adj[i][j] = count;

            if count < 4 {
                stack.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = stack.pop_back() {
        for (di, dj) in dirs {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni < 0 || ni >= n as isize || nj < 0 || nj >= m as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);

            if grid[ni][nj] != '@' { continue; }

            adj[ni][nj] -= 1;

            if adj[ni][nj] == 3 {
                stack.push_back((ni, nj));
            }
        }
    }

    let removable = adj
        .iter()
        .flatten()
        .filter(|&&x| x < 4)
        .count();

    println!("Part 2: {}", removable);
}
