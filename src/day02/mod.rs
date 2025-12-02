use std::fs;

pub fn run(input: &str) {
    let testcases = fs::read_to_string(input).expect("can't read file");
    let ans = solve(&testcases);
    println!("{}", ans[0]);
    println!("{}", ans[1]);
}

pub fn solve(s: &str) -> [u64; 2] {
    let mut p1 = 0;
    let mut p2 = 0;
    for pair in s.split(',') {
        let Some((a, b)) = pair.trim().split_once('-') else {
            continue;
        };
        let [a, b] = [a, b].map(|x| x.parse::<u64>().unwrap());
        for num in a..=b {
            let width = 1 + num.ilog10();
            for i in (1..=width / 2).rev().filter(|i| width % i == 0) {
                let p = 10_u64.pow(i);
                let rem = num % p;
                let mut x = num / p;
                while x > rem && x % p == rem {
                    x /= p;
                }
                if x == rem {
                    p2 += num;
                    if 2 * i == width {
                        p1 += num;
                    }
                    break;
                }
            }
        }
    }
    [p1, p2]
}
