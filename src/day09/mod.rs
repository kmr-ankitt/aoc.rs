use std::fs;

pub fn run(input_path: &str) {
    let input = fs::read_to_string(input_path)
        .expect("Cannot read input file");

    let (part1, part2) = solve(&input);

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}

fn solve(input: &str) -> (usize, usize) {
    let tiles = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line
                .trim()
                .split_once(',')
                .expect("Cannot read coordinates");
            (
                x.parse::<usize>().expect("Cannot parse"),
                y.parse::<usize>().expect("Cannot parse"),
            )
        })
        .collect::<Box<[(usize, usize)]>>();

    // Ensure polygon edges are axis-aligned
    debug_assert!(
        tiles
            .iter()
            .zip(tiles.iter().cycle().skip(1).take(tiles.len()))
            .all(|(&(px, py), &(qx, qy))| px == qx || py == qy)
    );

    tiles
        .iter()
        .enumerate()
        .fold((0usize, 0usize), |best, (i, (ix, iy))| {
            tiles
                .iter()
                .skip(i + 1)
                .fold(best, |(p1, p2), (jx, jy)| {
                    let (xmin, xmax) = if ix > jx { (*jx, *ix) } else { (*ix, *jx) };
                    let (ymin, ymax) = if iy > jy { (*jy, *iy) } else { (*iy, *jy) };

                    let area = (xmax + 1 - xmin) * (ymax + 1 - ymin);

                    (
                        p1.max(area),
                        if is_box_inside_polygon(&tiles, xmin, xmax, ymin, ymax) {
                            p2.max(area)
                        } else {
                            p2
                        },
                    )
                })
        })
}

fn is_box_inside_polygon(
    tiles: &[(usize, usize)],
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
) -> bool {
    tiles
        .iter()
        .zip(tiles.iter().cycle().skip(1).take(tiles.len()))
        .all(|(&(px, py), &(qx, qy))| {
            if py == qy {
                // horizontal edge
                py >= ymax
                    || py <= ymin
                    || (px <= xmin && qx <= xmin)
                    || (px >= xmax && qx >= xmax)
            } else {
                // vertical edge
                px >= xmax
                    || px <= xmin
                    || (py <= ymin && qy <= ymin)
                    || (py >= ymax && qy >= ymax)
            }
        })
}
