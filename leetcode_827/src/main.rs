struct Solution {}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, VecDeque};
        let mut best = 1;
        let mut grid = grid;
        let n = grid.len();
        let m = grid.first().map_or(0, |r0| r0.len());
        let mut queue = VecDeque::new();
        let mut count = 2;
        let mut record = vec![0; n * m];

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 {
                    let mut area = 1;
                    grid[r][c] = count;
                    queue.push_back((r, c));
                    while let Some((r1, c1)) = queue.pop_front() {
                        let neighbours = [
                            (r1 + 1, c1),
                            (r1, c1 + 1),
                            (r1.wrapping_sub(1), c1),
                            (r1, c1.wrapping_sub(1)),
                        ];
                        for (r2, c2) in neighbours {
                            if r2 < n && c2 < m && grid[r2][c2] == 1 {
                                grid[r2][c2] = count;
                                area += 1;
                                queue.push_back((r2, c2));
                            }
                        }
                    }
                    record[count as usize] = area;
                    best = best.max(area);
                    count += 1;
                }
            }
        }

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 0 {
                    let mut area = 1;
                    let mut near_lands = HashSet::new();
                    let neighbours = [
                        (r + 1, c),
                        (r, c + 1),
                        (r.wrapping_sub(1), c),
                        (r, c.wrapping_sub(1)),
                    ];
                    for (r1, c1) in neighbours {
                        if r1 < n && c1 < m {
                            near_lands.insert(grid[r1][c1]);
                        }
                    }
                    for land_count in near_lands.into_iter() {
                        area += record[land_count as usize];
                    }
                    best = best.max(area);
                }
            }
        }

        best
    }
}

fn main() {
    println!("Hello, world!");
}
