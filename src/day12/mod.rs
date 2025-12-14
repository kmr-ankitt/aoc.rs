use std::fs;

pub fn run(input_path: &str) {
    let (shapes, regions) = read_input(input_path)
        .expect("Failed to read input");

    let solver = Solver::new(&shapes, &regions);

    let part1 = solver.part1();
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", solver.part2());
}

/* ---------------- Parsing ---------------- */

fn read_input(file: &str) -> Result<(Vec<Shape>, Vec<Region>), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(file)?;
    let mut lines = input.lines();

    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    // Read 6 shapes
    for _ in 0..6 {
        lines.next(); // index line
        let mut shape_lines = String::new();
        for _ in 0..3 {
            shape_lines.push_str(lines.next().unwrap());
            shape_lines.push('\n');
        }
        shapes.push(Shape::from_str(&shape_lines));
        lines.next(); // empty line
    }

    // Read regions
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (dims, counts) = line.split_once(':').ok_or("Invalid region")?;
        let (w, h) = dims.split_once('x').ok_or("Invalid dimensions")?;

        let nums: Vec<usize> = counts
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        if nums.len() != 6 {
            return Err("Invalid shape counts".into());
        }

        regions.push(Region {
            width: w.parse()?,
            height: h.parse()?,
            shape_quantity: [nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]],
        });
    }

    Ok((shapes, regions))
}

/* ---------------- Domain ---------------- */

struct Region {
    width: usize,
    height: usize,
    shape_quantity: [usize; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Shape {
    rows: [u64; 3],
    size: usize,
}

impl Shape {
    fn from_str(s: &str) -> Self {
        let mut rows = [0u64; 3];
        let mut size = 0;

        for (y, line) in s.lines().enumerate().take(3) {
            for (x, ch) in line.chars().enumerate().take(3) {
                if ch == '#' {
                    rows[y] |= 1 << x;
                    size += 1;
                }
            }
        }

        Self { rows, size }
    }

    fn rotate(&self) -> Self {
        let mut rows = [0u64; 3];
        for y in 0..3 {
            for x in 0..3 {
                if (self.rows[y] & (1 << x)) != 0 {
                    rows[x] |= 1 << (2 - y);
                }
            }
        }
        Self { rows, size: self.size }
    }

    fn flip(&self) -> Self {
        let mut rows = [0u64; 3];
        for y in 0..3 {
            for x in 0..3 {
                if (self.rows[y] & (1 << x)) != 0 {
                    rows[y] |= 1 << (2 - x);
                }
            }
        }
        Self { rows, size: self.size }
    }

    fn variants(&self) -> Vec<Self> {
        let mut v = Vec::new();
        let mut cur = *self;

        for _ in 0..4 {
            v.push(cur);
            v.push(cur.flip());
            cur = cur.rotate();
        }

        v.sort_by_key(|s| s.rows);
        v.dedup_by_key(|s| s.rows);
        v
    }
}

/* ---------------- Solver ---------------- */

struct Grid {
    rows: Vec<u64>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            rows: vec![0; h],
            width: w,
            height: h,
        }
    }

    fn can_fit(&self, shape: &Shape, x: usize, y: usize) -> bool {
        if x + 3 > self.width || y + 3 > self.height {
            return false;
        }
        for r in 0..3 {
            if (self.rows[y + r] & (shape.rows[r] << x)) != 0 {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, x: usize, y: usize) {
        for r in 0..3 {
            self.rows[y + r] |= shape.rows[r] << x;
        }
    }

    fn remove(&mut self, shape: &Shape, x: usize, y: usize) {
        for r in 0..3 {
            self.rows[y + r] &= !(shape.rows[r] << x);
        }
    }
}

struct Solver<'a> {
    regions: &'a [Region],
    variants: Vec<Vec<Shape>>,
}

impl<'a> Solver<'a> {
    fn new(shapes: &'a [Shape], regions: &'a [Region]) -> Self {
        let variants = shapes.iter().map(|s| s.variants()).collect();
        Self { regions, variants }
    }

    fn backtrack(&self, grid: &mut Grid, items: &[&Vec<Shape>], idx: usize) -> bool {
        if idx == items.len() {
            return true;
        }

        for y in 0..grid.height {
            for x in 0..grid.width {
                for shape in items[idx] {
                    if grid.can_fit(shape, x, y) {
                        grid.place(shape, x, y);
                        if self.backtrack(grid, items, idx + 1) {
                            return true;
                        }
                        grid.remove(shape, x, y);
                    }
                }
            }
        }
        false
    }

    fn part1(&self) -> u64 {
        let mut count = 0;

        for region in self.regions {
            let mut items = Vec::new();
            let mut area = 0;

            for (i, &n) in region.shape_quantity.iter().enumerate() {
                for _ in 0..n {
                    items.push(&self.variants[i]);
                    area += self.variants[i][0].size;
                }
            }

            if area > region.width * region.height {
                continue;
            }

            items.sort_by_key(|v| usize::MAX - v[0].size);

            let mut grid = Grid::new(region.width, region.height);
            if self.backtrack(&mut grid, &items, 0) {
                count += 1;
            }
        }

        count
    }

    fn part2(&self) -> &str {
        "There is no part 2"
    }
}
